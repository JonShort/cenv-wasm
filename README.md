# cenv-wasm

Completely _unnecessary_ port of cenv internals to node via. wasm :joy:

I'm interested to see if I can expose a version of `cenv` on npm while keeping the internals written in rust.

End goal would be for users to be able to install globally e.g. `npm i -g @jonshort/cenv`

It's not a straightforward swap; the big change is making `parse_env` accept two strings instead of structs - mostly just because it'd be a pain to instantiate a Config and EnvContents in JS-land. It is possible, but a bit cleaner just using the strings (for now).

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

## How to build the internals
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

## In action!
![image](https://user-images.githubusercontent.com/21317379/117722925-a0479f00-b1d9-11eb-928f-d33d0bd5f4ea.png)
