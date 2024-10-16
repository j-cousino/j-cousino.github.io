mod blogs;

use handlebars::Handlebars;
use serde_json::json;
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

pub struct Generator<'a> {
    handlebars: handlebars::Handlebars<'a>,
    out_directory: PathBuf,
}

impl<'a> Generator<'a> {
    pub fn create(out_directory: &Path, posts_directory: &Path) -> anyhow::Result<Self> {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);
        handlebars.register_templates_directory("templates", Default::default())?;

        anyhow::Ok(Generator {
            handlebars,
            out_directory: out_directory.into(),
        })
    }

    pub fn generate(&self) -> anyhow::Result<()> {
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
