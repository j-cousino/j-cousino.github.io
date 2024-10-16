use std::path::Path;

fn main() -> anyhow::Result<()> {
    let out_directory = Path::new("site");
    let posts_directory = Path::new("posts");

    website::Generator::create(out_directory, posts_directory)
        .unwrap()
        .generate()?;

    anyhow::Ok(())
}
