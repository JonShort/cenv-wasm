#!/usr/bin/env node
const { main } = require('./pkg/cenv_wasm.js');

const logErrorAndExit = (err, description = "Failed with the following error:") => {
  console.log("\x1b[31m%s\x1b[0m", "[Error] - Error while running");
  console.log("\x1b[31m%s\x1b[0m", " |", description);
  console.log("\x1b[31m%s\x1b[0m", " |", err);
  process.exit(1);
};

const jsMain = async () => {
  const providedKeyword = process.argv[2];

  if (!providedKeyword) {
    logErrorAndExit("e.g. `cenv myKeyword`", "Please provide your keyword as the first argument to cenv");
    return;
  }

  let newEnv;
  try {
    newEnv = main(providedKeyword)
  } catch (err) {
    logErrorAndExit(err);
    return;
  }

  process.exit(0);
}

jsMain();
