# cenv-wasm

[![npm version](https://badge.fury.io/js/%40jonshort%2Fcenv.svg)](https://badge.fury.io/js/%40jonshort%2Fcenv)

![usage](https://user-images.githubusercontent.com/21317379/111026089-60825800-83e0-11eb-99ab-054463749377.gif)

Using a comment pattern in your .env files, easily swap between envs in local development.

## Installation

Install globally via. npm
```bash
npm install -g @jonshort/cenv
```

## Usage

1. Add the "cenv" pattern to your .env file
```.env
VARIABLE_1=something

# ++ local ++
# API_ADDRESS=http://localhost:5000
# REQUIRE_LOGIN=false

# ++ live ++
# API_ADDRESS=https://myliveapi.com
# REQUIRE_LOGIN=true
```

2. Run cenv, choosing an env keyword to use
```bash
cenv live
```

3. Check your .env, the keyworded env vars will now be uncommented
```.env
VARIABLE_1=something

# ++ local ++
# API_ADDRESS=http://localhost:5000
# REQUIRE_LOGIN=false

# ++ live ++
API_ADDRESS=https://myliveapi.com
REQUIRE_LOGIN=true
```

## Use via. npx

You can also run this cli via. `npx` which avoids having to install it globally, e.g:
```bash
npx @jonshort/cenv keyword
```

## Uninstalling

1. Check globally installed packages
```bash
npm ls -g
```

2. Uninstall the package
```bash
npm uninstall -g @jonshort/cenv
```
----
## Development

### How to build the internals
1. Install `wasm-pack`
```bash
cargo install wasm-pack
```
2. Build with the target of nodejs
```bash
wasm-pack build --target nodejs
```
3. Check the `pkg/` folder - this is the compiled wasm
4. Once built run `node index.js` and check everything is working as expected
