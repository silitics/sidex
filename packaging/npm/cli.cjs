#!/usr/bin/env node
"use strict";

const { spawn } = require("node:child_process");
const { version } = require("./package.json");

let platform = `${process.platform}-${process.arch}`;
if (process.platform === "linux") {
  const libc = process.report.getReport().header.glibcVersionRuntime ? "glibc" : "musl";
  platform += `-${libc}`;
}
const packageName = `@sidex/cli-${platform}`;
let binary;
try {
  binary = require.resolve(`${packageName}/bin/sidex`);
} catch {
  console.error(
    `Sidex ${version} has no installed binary for ${platform}. ` +
    "Reinstall sidex-cli with optional dependencies enabled. " +
    "Supported platforms are Linux and macOS on x64 and ARM64."
  );
  process.exit(1);
}

const child = spawn(binary, process.argv.slice(2), { stdio: "inherit" });
const signals = ["SIGINT", "SIGTERM"];
const handlers = new Map(signals.map((signal) => [signal, () => child.kill(signal)]));
for (const [signal, handler] of handlers) {
  process.on(signal, handler);
}
child.on("error", (error) => {
  console.error(`Failed to run packaged Sidex: ${error.message}`);
  process.exitCode = 1;
});
child.on("exit", (code, signal) => {
  for (const [name, handler] of handlers) {
    process.removeListener(name, handler);
  }
  if (signal) {
    process.kill(process.pid, signal);
  } else {
    process.exitCode = code;
  }
});
