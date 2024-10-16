use std::path::Path;

pub struct Blog {

}

pub fn load(root: &Path) -> anyhow::Result<Vec<Blog>> {
    let mut blogs = Vec::new();

    anyhow::Ok(blogs)
}
