use wasm_bindgen::prelude::*;
mod utils;
use cenv_core::{parse_env, Config};

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    let keyword = match utils::get_keyword() {
        Ok(k) => k,
        Err(e) => {
            return Err(JsValue::from(e));
        }
    };

    let config = match Config::new(&keyword) {
        Ok(c) => c,
        Err(e) => {
            return Err(JsValue::from(e));
        }
    };
    let env = match utils::read_env_file() {
        Ok(d) => d,
        Err(e) => {
            return Err(JsValue::from(e));
        }
    };

    let new_env = match parse_env(&env, &config) {
        Ok(env) => env,
        Err(e) => {
            return Err(JsValue::from(e));
        }
    };

    match utils::write_to_file(&new_env) {
        Ok(_) => utils::log(&format!("Updated .env to {}", &config.keyword)),
        Err(e) => {
            return Err(JsValue::from(e));
        }
    }

    Ok(())
}
