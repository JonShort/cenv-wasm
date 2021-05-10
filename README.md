# cenv-wasm

Completely _unnecessary_ port of cenv internals to node via. wasm :joy:

I'm interested to see if I can expose a version of `cenv` on npm while keeping the internals written in rust.

End goal would be for users to be able to install globally e.g. `npm i -g @jonshort/cenv`

It's not a straightforward swap; the big change is making `parse_env` accept two strings instead of structs - mostly just because it'd be a pain to instantiate a Config and EnvContents in JS-land. It is possible, but a bit cleaner just using the strings (for now).

## How to build this
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
