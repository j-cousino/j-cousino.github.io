use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize)]
struct Manifest {

}
pub struct Blog {

}

pub fn load(root: &Path) -> anyhow::Result<Vec<Blog>> {
    let mut blogs = Vec::new();

    anyhow::Ok(blogs)
}
