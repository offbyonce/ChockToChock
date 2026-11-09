// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// A small YAML writer for plain data. Strings that could be misread are double-quoted, which YAML
// reads the same way as JSON.

const RESERVED = /^(?:y|n|yes|no|on|off|true|false|null|~)$/i;
const PLAIN = /^[A-Za-z][A-Za-z0-9 _/.()-]*$/;

function scalar(value) {
  if (typeof value === "string") {
    const plain = PLAIN.test(value) && !RESERVED.test(value) && !value.endsWith(" ");
    return plain ? value : JSON.stringify(value);
  }
  return String(value);
}

function isEmpty(value) {
  return typeof value === "object" && value !== null && Object.keys(value).length === 0;
}

function isBlock(value) {
  return typeof value === "object" && value !== null && !isEmpty(value);
}

function write(value, indent, lines) {
  const pad = " ".repeat(indent);
  if (Array.isArray(value)) {
    for (const item of value) {
      if (isBlock(item)) {
        const inner = [];
        write(item, indent + 2, inner);
        lines.push(`${pad}- ${inner[0].slice(indent + 2)}`, ...inner.slice(1));
      } else {
        lines.push(`${pad}- ${inline(item)}`);
      }
    }
    return;
  }
  for (const [key, item] of Object.entries(value)) {
    if (item === undefined) continue;
    const name = PLAIN.test(key) && !RESERVED.test(key) ? key : JSON.stringify(key);
    if (isBlock(item)) {
      lines.push(`${pad}${name}:`);
      write(item, indent + 2, lines);
    } else {
      lines.push(`${pad}${name}: ${inline(item)}`);
    }
  }
}

function inline(value) {
  if (Array.isArray(value)) return "[]";
  if (typeof value === "object" && value !== null) return "{}";
  return scalar(value);
}

/** The YAML text for an object, after any header comment lines. */
export function toYaml(value, header = []) {
  const lines = [...header];
  write(value, 0, lines);
  return `${lines.join("\n")}\n`;
}
