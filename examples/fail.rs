use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	pretty_env_logger::init();
	retrier::retry(|| reqwest::get("nope")).await?;
	Ok(())
}
