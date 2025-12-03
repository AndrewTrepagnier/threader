
use serde::Deserialize;
use std::fs;

// ============================================
// CHANGE THIS NUMBER TO THE PR YOU WANT
const PR_NUMBER: u32 = 115822;
// ============================================

#[derive(Deserialize)]
struct PullRequest {
    body: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/repos/rust-lang/rust/pulls/{}", PR_NUMBER);
    
    let client = reqwest::Client::builder()
        .user_agent("rust-pr-fetcher")
        .build()?;
    
    let pr: PullRequest = client.get(&url).send().await?.json().await?;
    
    let body = pr.body.unwrap_or_default();
    
    fs::create_dir_all("output")?;
    let filename = format!("output/pr_{}.txt", PR_NUMBER);
    
    fs::write(&filename, body)?;
    
    println!("Saved to {}", filename);
    
    Ok(())
}

