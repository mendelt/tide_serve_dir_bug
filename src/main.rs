#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    let mut app = tide::new();
    app.at("/swagger").serve_dir("./swagger/")?;
    app.listen("0.0.0.0:5000").await?;
    Ok(())
}
