#!/usr/bin/env node
const { main } = require('./pkg/cenv_wasm.js');

const logErrorAndExit = (err = "") => {
  const msgs = err.split('|');

  console.log("\x1b[31m%s\x1b[0m", "[Error] - Error while running");
  console.log("\x1b[31m%s\x1b[0m", " |", "Failed with the following error:");
  msgs.forEach(msg => console.log("\x1b[31m%s\x1b[0m", " |", msg));

  process.exit(1);
};

try {
  main()
  process.exit(0);
} catch (err) {
  logErrorAndExit(err);
}
