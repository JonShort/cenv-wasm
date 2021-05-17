use wasm_bindgen::prelude::*;

#[derive(PartialEq)]
pub enum ParseStatus {
  Active,
  Inactive,
  Ignore,
}

#[derive(PartialEq, Debug)]
pub struct Config {
  pub keyword: String,
}

impl Config {
  pub fn new(keyword: &str) -> Result<Config, &'static str> {
    let keyword = match keyword {
      "" => return Err("Keyword missing"),
      word => word,
    };

    Ok(Config {
      keyword: String::from(keyword),
    })
  }
}

#[derive(PartialEq, Debug)]
pub struct EnvContents {
  pub contents: String,
}

impl EnvContents {
  pub fn new(contents: String) -> EnvContents {
    EnvContents { contents }
  }
}

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

pub fn write_to_file(env: &EnvContents) -> Result<(), String> {
  match write_file_sync(".env", &env.contents) {
    Ok(_) => Ok(()),
    Err(_) => Err(String::from("Unable to write .env file")),
  }
}
