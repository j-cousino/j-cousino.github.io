#[tokio::main]
async fn main() -> anyhow::Result<()> {

    pretty_env_logger::init();
    
    website::main()?;

    warp::serve(warp::fs::dir("site"))
        .run(([0, 0, 0, 0], 8080))
        .await;

    anyhow::Ok(())
}