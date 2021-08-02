use wasm_bindgen::prelude::*;
mod utils;
use cenv_core::{list_available_keywords, parse_env, Config, EnvContents};

fn add_keywords_to_error(env: &EnvContents, err: &str) -> String {
    let keys = list_available_keywords(env);
    let keys = keys.join("| - ");

    format!("{}||Available keywords:| - {}", err, keys)
}

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    let env = match utils::read_env_file() {
        Ok(d) => d,
        Err(e) => {
            return Err(JsValue::from(e));
        }
    };

    let keyword = match utils::get_keyword() {
        Ok(k) => k,
        Err(e) => {
            let e = add_keywords_to_error(&env, e);
            return Err(JsValue::from(e));
        }
    };

    let config = match Config::new(&keyword) {
        Ok(c) => c,
        Err(e) => {
            return Err(JsValue::from(e));
        }
    };

    let new_env = match parse_env(&env, &config) {
        Ok(env) => env,
        Err(e) => {
            let e = add_keywords_to_error(&env, &e);
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
