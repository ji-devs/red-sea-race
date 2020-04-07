use web_sys::window;
use crate::config;
use wasm_bindgen::prelude::*;

pub fn get_uri_parts() -> Vec<String> {
    match window().unwrap_throw().location().pathname() {
        Ok(pathname) => {
            let uri = get_root(pathname.as_str());
            if uri == "" {
                vec![]
            } else {
                uri.split("/").map(|s| s.to_string()).collect()
            }
        },
        Err(_) => vec![]
    }
}

//simple stripping of host dir like if deploying to example.com/foo
pub fn get_root(input: &str) -> &str {
    let stripped = match config::HOST_DIRECTORY {
        None => input,
        Some(host_dir) => {
            input
                .find(host_dir)
                .map(|len| input.split_at(len + host_dir.len() - 1).1)
                .or(Some(input))
                .unwrap_throw()
        }
    };

    stripped.trim_matches('/')
}