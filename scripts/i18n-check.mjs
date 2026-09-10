import fs from "node:fs";
import path from "node:path";

const zh = fs.readFileSync("src/i18n/zh.ts", "utf8");
const en = fs.readFileSync("src/i18n/en.ts", "utf8");
const keys = (f) =>
  new Set([...f.matchAll(/^ {2}"([^"]+)":/gm)].map((m) => m[1]));
const zk = keys(zh);
const ek = keys(en);

const walk = (d) =>
  fs
    .readdirSync(d, { withFileTypes: true })
    .flatMap((e) =>
      e.isDirectory() ? walk(path.join(d, e.name)) : [path.join(d, e.name)],
    );

const used = new Set();
const tpl = new Set();
for (const f of walk("src").filter(
  (f) => /\.(vue|ts)$/.test(f) && !f.includes("i18n"),
)) {
  const s = fs.readFileSync(f, "utf8");
  for (const m of s.matchAll(/t[ p]?\(\s*['"`]([^'"`]+)['"`]/g))
    used.add(m[1]);
  for (const m of s.matchAll(/t\(`([a-z.]+)\$\{/g)) tpl.add(m[1]);
}

// expand template prefixes into known suffixes used in code
const missing = [...used].filter((k) => !zk.has(k) || !ek.has(k));
console.log("missing:", missing);
console.log("zh-only:", [...zk].filter((k) => !ek.has(k)));
console.log("en-only:", [...ek].filter((k) => !zk.has(k)));
console.log("tpl-bases:", [...tpl]);
console.log(`zh keys: ${zk.size}, en keys: ${ek.size}, used: ${used.size}`);
