use web_sys::window;
use std::rc::Rc;
use crate::config;
use crate::enums::*;
use crate::loader::*;
use manifest::*;

pub async fn get_page() -> Option<RootPage> {
    let uri_parts = get_uri_parts();
    if uri_parts.len() == 0 {
        let app_manifest = load_app_manifest().await.expect("unable to load app manifest!!");
        Some(RootPage::Home(Rc::new(app_manifest)))

    } else if uri_parts[0] == "topic" {
        get_topic(&uri_parts)
            .await
            .map(|manifest| RootPage::Topic(Rc::new(manifest)))
    } else {
        None
    }
}

async fn get_topic(uri_parts:&Vec<String>) -> Option<TopicManifest> {
    if uri_parts.len() > 1 {
        load_topic_manifest(&uri_parts[1]).await.ok()
    } else {
        None 
    }
}

fn get_uri_parts() -> Vec<String> {
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
fn get_root(input: &str) -> &str {
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