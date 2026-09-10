import { spawn } from "node:child_process";

const url = "http://127.0.0.1:1420";
const up = await fetch(url, { signal: AbortSignal.timeout(2500) })
  .then((response) => response.ok)
  .catch(() => false);

if (up) {
  console.log("Vite is already serving on 127.0.0.1:1420 — reusing it.");
  process.exit(0);
}

const child = spawn("npx", ["vite", "--host", "127.0.0.1"], {
  stdio: "inherit",
  shell: true,
});
child.on("exit", (code) => process.exit(code ?? 1));
