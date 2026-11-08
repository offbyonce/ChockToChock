// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Saved data in one SQLite file in the app's data folder.
//!
//! The schema is versioned with SQLite's `user_version`. Each migration runs once,
//! in order, inside a transaction; never edit a migration that has shipped, add a new one.

use std::path::Path;

use checklist_core::Session;
use rusqlite::{Connection, OptionalExtension, params};

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("database error: {0}")]
    Sql(#[from] rusqlite::Error),
    #[error("saved data is damaged: {0}")]
    Corrupt(#[from] serde_json::Error),
    #[error(
        "saved data is from a newer version of the app (schema {found}, this app knows {known})"
    )]
    TooNew { found: u32, known: u32 },
}

/// Migrations in order. Index 0 takes the schema from version 0 to 1, and so on.
const MIGRATIONS: &[&str] = &[
    // 1: one saved session per pack, plus simple key/value settings.
    "CREATE TABLE sessions (
         pack_id    TEXT PRIMARY KEY,
         data       TEXT NOT NULL,
         updated_ms INTEGER NOT NULL
     );
     CREATE TABLE settings (
         key   TEXT PRIMARY KEY,
         value TEXT NOT NULL
     );",
];

#[derive(Debug)]
pub struct Store {
    conn: Connection,
}

impl Store {
    pub fn open(path: &Path) -> Result<Self, StoreError> {
        Self::init(Connection::open(path)?)
    }

    /// An in-memory store, for tests.
    pub fn open_in_memory() -> Result<Self, StoreError> {
        Self::init(Connection::open_in_memory()?)
    }

    fn init(mut conn: Connection) -> Result<Self, StoreError> {
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        migrate(&mut conn)?;
        Ok(Self { conn })
    }

    pub fn schema_version(&self) -> Result<u32, StoreError> {
        Ok(self
            .conn
            .pragma_query_value(None, "user_version", |r| r.get(0))?)
    }

    /// Save a pack's session. `now_ms` is passed in so the store never reads a clock.
    pub fn save_session(
        &self,
        pack_id: &str,
        session: &Session,
        now_ms: i64,
    ) -> Result<(), StoreError> {
        let data = serde_json::to_string(session)?;
        self.conn.execute(
            "INSERT INTO sessions (pack_id, data, updated_ms) VALUES (?1, ?2, ?3)
             ON CONFLICT(pack_id) DO UPDATE SET data = excluded.data, updated_ms = excluded.updated_ms",
            params![pack_id, data, now_ms],
        )?;
        Ok(())
    }

    pub fn load_session(&self, pack_id: &str) -> Result<Option<Session>, StoreError> {
        let data: Option<String> = self
            .conn
            .query_row(
                "SELECT data FROM sessions WHERE pack_id = ?1",
                [pack_id],
                |r| r.get(0),
            )
            .optional()?;
        Ok(data.map(|d| serde_json::from_str(&d)).transpose()?)
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn setting(&self, key: &str) -> Result<Option<String>, StoreError> {
        Ok(self
            .conn
            .query_row("SELECT value FROM settings WHERE key = ?1", [key], |r| {
                r.get(0)
            })
            .optional()?)
    }
}

fn migrate(conn: &mut Connection) -> Result<(), StoreError> {
    let known = u32::try_from(MIGRATIONS.len()).unwrap_or(u32::MAX);
    let current: u32 = conn.pragma_query_value(None, "user_version", |r| r.get(0))?;
    if current > known {
        return Err(StoreError::TooNew {
            found: current,
            known,
        });
    }
    for (version, sql) in (1..).zip(MIGRATIONS).skip(current as usize) {
        let tx = conn.transaction()?;
        tx.execute_batch(sql)?;
        tx.pragma_update(None, "user_version", version)?;
        tx.commit()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use checklist_core::{Checklist, Step, StepId};

    use super::*;

    fn list() -> Checklist {
        Checklist {
            id: "p".into(),
            title: "P".into(),
            steps: vec![Step {
                id: StepId::new("a"),
                text: "Battery".into(),
                expect: "ON".into(),
                note: None,
                sources: vec![],
            }],
        }
    }

    #[test]
    fn fresh_store_is_fully_migrated() {
        let store = Store::open_in_memory().unwrap();
        assert_eq!(store.schema_version().unwrap() as usize, MIGRATIONS.len());
    }

    #[test]
    fn sessions_round_trip_and_overwrite() {
        let store = Store::open_in_memory().unwrap();
        assert_eq!(store.load_session("demo").unwrap(), None);

        let mut session = Session::new();
        session.tick(&list(), &StepId::new("a")).unwrap();
        store.save_session("demo", &session, 1).unwrap();
        assert_eq!(store.load_session("demo").unwrap(), Some(session));

        store.save_session("demo", &Session::new(), 2).unwrap();
        assert_eq!(store.load_session("demo").unwrap(), Some(Session::new()));
    }

    #[test]
    fn settings_round_trip() {
        let store = Store::open_in_memory().unwrap();
        store.set_setting("simbrief_user", "simpilot").unwrap();
        assert_eq!(
            store.setting("simbrief_user").unwrap().as_deref(),
            Some("simpilot")
        );
        assert_eq!(store.setting("missing").unwrap(), None);
    }

    #[test]
    fn newer_database_is_refused() {
        let mut conn = Connection::open_in_memory().unwrap();
        conn.pragma_update(None, "user_version", 99).unwrap();
        assert!(matches!(
            migrate(&mut conn),
            Err(StoreError::TooNew { found: 99, .. })
        ));
    }

    #[test]
    fn reopening_a_file_keeps_data() {
        let dir = std::env::temp_dir().join(format!("chocktochock-store-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("test.db");
        {
            let store = Store::open(&path).unwrap();
            store.set_setting("k", "v").unwrap();
        }
        let store = Store::open(&path).unwrap();
        assert_eq!(store.setting("k").unwrap().as_deref(), Some("v"));
        drop(store);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
