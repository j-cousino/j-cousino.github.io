mod blogs;

use handlebars::Handlebars;
use serde_json::json;
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

struct Generator<'a> {
    handlebars: handlebars::Handlebars<'a>,
    out_directory: PathBuf,
    blogs: Vec<crate::blogs::Blog>,
}

impl<'a> Generator<'a> {
    fn create(out_directory: &Path, posts_directory: &Path) -> anyhow::Result<Self> {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);
        handlebars.register_templates_directory("templates", Default::default())?;

        anyhow::Ok(Generator {
            handlebars,
            out_directory: out_directory.into(),
            blogs: crate::blogs::load(posts_directory)?,
        })
    }

    fn generate(&self) -> anyhow::Result<()> {
        fs::create_dir_all(self.out_directory.clone())?;

        let data = json!({
            "title": "jeff.cousino",
            "description": "My personal blog"
        });

        let file_path = self.out_directory.join("index.html");
        let file = File::create(file_path)?;
        self.handlebars.render_to_write("index", &data, file)?;

        anyhow::Ok(())
    }
}

pub fn main() -> anyhow::Result<()> {
    let out_directory = Path::new("site");
    let posts_directory = Path::new("posts");

    Generator::create(out_directory, posts_directory)
        .unwrap()
        .generate()?;

    anyhow::Ok(())
}

