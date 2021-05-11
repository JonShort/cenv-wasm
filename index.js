import { readFile, writeFile } from 'fs/promises';
import { parse_env } from './pkg/cenv_wasm.js';

const FILE = '.example';

const logFsErrorAndExit = (err) => {
  console.log("\x1b[31m%s\x1b[0m", "[Error] - Error while reading", FILE, "file");
  console.log("\x1b[31m%s\x1b[0m", " |", "Failed with the following error:");
  console.log("\x1b[31m%s\x1b[0m", " |", err.message);
  process.exit(1);
};

const currentEnv = await readFile(FILE, { encoding: 'utf-8' }).catch(logFsErrorAndExit)

const providedKeyword = process.argv[2];

if (!providedKeyword) {
  console.log("\x1b[31m%s\x1b[0m", "[Error] - No keyword provided");
  console.log("\x1b[31m%s\x1b[0m", " |", "Please provide your keyword as the first argument to cenv");
  console.log("\x1b[31m%s\x1b[0m", " |", "e.g. `cenv myKeyword`");
  process.exit(1);
}

await writeFile(FILE, parse_env(currentEnv, providedKeyword)).catch(logFsErrorAndExit)

console.log("Updated", FILE, "to", providedKeyword);
process.exit(0);
