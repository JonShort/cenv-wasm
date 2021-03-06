use cenv_core::EnvContents;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "fs")]
extern "C" {
    #[wasm_bindgen(js_name = readFileSync, catch)]
    fn read_file_sync(path: &str, format: &str) -> Result<String, JsValue>;

    #[wasm_bindgen(js_name = writeFileSync, catch)]
    fn write_file_sync(path: &str, data: &str) -> Result<(), JsValue>;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

pub fn read_env_file() -> Result<EnvContents, &'static str> {
    let contents = match read_file_sync(".env", "utf-8") {
        Ok(w) => w,
        Err(_) => return Err("Unable to read .env file"),
    };
    Ok(EnvContents { contents })
}

pub fn write_to_file(env: &EnvContents) -> Result<(), &'static str> {
    match write_file_sync(".env", &env.contents) {
        Ok(_) => Ok(()),
        Err(_) => Err("Unable to write .env file"),
    }
}

// FYI - For anything more complex this would be a module #[wasm_bindgen(module = "/js/foo.js")]
#[wasm_bindgen(inline_js = "module.exports = { get_arg: () => process.argv[2]}")]
extern "C" {
    fn get_arg() -> Option<String>;
}

pub fn get_keyword() -> Result<String, &'static str> {
    match get_arg() {
        Some(k) => Ok(k),
        None => {
            Err("Please provide your keyword as the first argument to cenv, e.g. `cenv myKeyword`")
        }
    }
}
