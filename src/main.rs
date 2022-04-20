use reqwest;

struct Urls {
    name: String,
    url: String,
    found: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // [!] found (green)
    // [x] no found (red)

    let mut urls: Vec<Urls> = Vec::new();

    // loading urls into urls vec
    let contents = std::fs::read_to_string("./urls.txt")?;

    for line in contents.lines() {
        if line.starts_with("//") == false && line.starts_with("#") == false {
            let line = line.split_once(':').unwrap();

            urls.push(Urls {
                name: line.0.to_string(),
                url: line.1.to_string(),
                found: false,
            });
        }
    }

    for url in urls {
        println!("{}\t{}\t{}", url.name, url.url, url.found);
    }
    // for url in urls {
    //     let res = reqwest::get(url.url.replace("{}", "")).await?;

    //     if res.status() == 200 {
    //         println!("Found!");
    //     } else {
    //         println!("Not found!");
    //     }
    // }

    Ok(())
}
