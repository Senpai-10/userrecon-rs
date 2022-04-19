use reqwest;

struct Urls {
    name: String,
    url: String,
    found: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut urls: Vec<Urls> = Vec::new();

    for url in urls {
        let res = reqwest::get(url.url.replace("{}", "")).await?;

        if res.status() == 200 {
            println!("Found!");
        } else {
            println!("Not found!");
        }
    }

    Ok(())
}
