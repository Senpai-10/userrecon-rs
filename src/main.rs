use colored::Colorize;
use reqwest;
use std::process::exit;
use whoami;

struct Urls {
    name: String,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() <= 0 {
        println!(
            "{} Please provide a username!",
            format!("[ ERROR ]").bright_red()
        );
        println!("Usage: userrecon-rs <username>");
        exit(1);
    }

    println!(
        "{}",
        format!(
            r#"
  _   _ ___  ___ _ __ _ __ ___  ___ ___  _ __        _ __ ___ 
  | | | / __|/ _ \ '__| '__/ _ \/ __/ _ \| '_ \ _____| '__/ __|
  | |_| \__ \  __/ |  | | |  __/ (_| (_) | | | |_____| |  \__ \
   \__,_|___/\___|_|  |_|  \___|\___\___/|_| |_|     |_|  |___/
                by: https://github.com/Senpai-10
   "#
        )
        .bright_green()
    );

    let username = &args[0];

    let home_dir = format!("/home/{}/", whoami::username());
    let db_dir = format!("{}/.config/userrecon-rs", home_dir);
    let db_file = format!("{}/urls.txt", db_dir);

    if std::path::Path::new(&db_dir).exists() == false {
        std::fs::create_dir_all(&db_dir)?;
    }
    if std::path::Path::new(&db_file).exists() == false {
        std::fs::copy("./urls.txt", &db_file)?;
    }

    let mut urls: Vec<Urls> = Vec::new();

    // loading urls into urls vec
    let contents = std::fs::read_to_string(db_file)?;

    for line in contents.lines() {
        if line.starts_with("//") == false && line.starts_with("#") == false {
            let line = line.split_once(':').unwrap();

            urls.push(Urls {
                name: line.0.to_string(),
                url: line.1.to_string(),
            });
        }
    }

    println!(
        "{} Checking username {}",
        format!("[*]").bold().bright_yellow(),
        username.bold()
    );
    for url in urls {
        let new_url = url.url.replace("{}", username);
        let res = reqwest::get(&new_url).await?;

        if res.status() == 200 {
            println!(
                "{} {}: {}",
                format!("[+]").bold().bright_green(),
                url.name,
                new_url.bold()
            );
        } else {
            println!(
                "{} {}: {}",
                format!("[-]").bold().bright_red(),
                url.name,
                format!("Not Found").bright_red().bold()
            );
        }
    }

    Ok(())
}
