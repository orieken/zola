mod cli;
mod config;
mod version_fetcher;

use crate::cli::parse_cli_args;
use crate::config::ArtifactoryConfig;
use crate::version_fetcher::get_version;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Ok((package_name, private)) = parse_cli_args() else { return Ok(()) };

    if private {
        println!("Fetching private package version...");
        let config = ArtifactoryConfig::new()?;
        let (latest, next, beta, rc) = get_version(&config.url, Some(&config.token), &package_name).await?;
        for i in [latest, next, beta, rc].iter() {
            if i != &"Not Found".to_string() {
                println!("{}@artifactory: {}", package_name, i);
            }
        }
    } else {
        println!("Fetching public package version...");
        let npmjs_url = "https://registry.npmjs.org/";
        let (latest, next, beta, rc) = get_version(&npmjs_url, None, &package_name).await?;
        for i in [latest, next, beta, rc].iter() {
            if i != &"Not Found".to_string() {
                println!("{}@npmjs: {}", package_name, i);
            }
        }
    }

    Ok(())
}
