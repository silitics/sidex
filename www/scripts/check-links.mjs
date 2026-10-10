import { spawnSync } from "node:child_process";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const dist = fileURLToPath(new URL("../dist", import.meta.url));
const escapedDist = dist.replace(/[\\^$.*+?()[\]{}|]/g, "\\$&");

// Offline file resolution cannot detect an omitted deployment prefix.
for (const entry of readdirSync(dist, {
  recursive: true,
  withFileTypes: true,
})) {
  if (!entry.isFile() || !entry.name.endsWith(".html")) continue;
  const file = join(entry.parentPath, entry.name);
  const html = readFileSync(file, "utf8");
  for (const [, url] of html.matchAll(/\b(?:href|src)="(\/[^"]*)"/g)) {
    if (url.startsWith("//") || url === "/sidex" || url.startsWith("/sidex/"))
      continue;
    throw new Error(`URL outside /sidex/: ${url} in ${file}`);
  }
}

const result = spawnSync(
  "nix",
  [
    "run",
    "nixpkgs#lychee",
    "--",
    "--offline",
    "--no-progress",
    "--include-fragments=anchor-only",
    "--root-dir",
    dist,
    "--index-files",
    "index.html",
    "--include",
    "^file://",
    "--remap",
    `^https://oss\\.silitics\\.com/sidex(?:/|$) file://${dist}/`,
    "--remap",
    `^file://${escapedDist}/sidex(?:/|$) file://${dist}/`,
    dist,
  ],
  { stdio: "inherit" },
);

if (result.error) throw result.error;
process.exit(result.status ?? 1);
