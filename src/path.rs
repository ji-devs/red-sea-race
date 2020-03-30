use crate::config::*;

pub fn media_url(path:&str) -> String {
    format!("{}/{}", MEDIA_URL, path)
}