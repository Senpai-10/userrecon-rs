use colored::Colorize;
use reqwest;
use std::process::exit;
use whoami;

struct Urls {
    name: String,
    url: String,
}

const HELP_MESSAGE: &str = r#"Usage: userrecon-rs <username>

Options:
--clean-output      ,-c      Only output urls
--help              ,-h      Print this message
"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut clean_output = false;
    let mut username = &String::new();

    if args.len() <= 0 {
        println!(
            "{} Please provide a username!",
            format!("[ERROR]").bright_red()
        );
        println!("");
        println!("{}", HELP_MESSAGE);
        exit(1);
    }

    for arg in args.iter() {
        if arg.starts_with("-") {
            if arg == "--clean-output" || arg == "-c" {
                clean_output = true;
            }
            if arg == "--help" || arg == "-h" {
                println!("{}", HELP_MESSAGE);
                exit(0);
            }
        } else if username.is_empty() && arg.starts_with("-") == false {
            username = arg;
        }
    }

    if clean_output == false {
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
    }

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

    if clean_output == false {
        println!(
            "{} Checking username {}",
            format!("[*]").bold().bright_yellow(),
            username.bold()
        );
    }
    for url in urls {
        let new_url = url.url.replace("{}", username);
        let res = reqwest::get(&new_url).await?;

        if res.status() == 200 {
            if clean_output == false {
                println!(
                    "{} {}: {}",
                    format!("[+]").bold().bright_green(),
                    url.name,
                    new_url.bold()
                );
            } else if clean_output == true {
                println!("{}", new_url);
            }
        } else {
            if clean_output == false {
                println!(
                    "{} {}: {}",
                    format!("[-]").bold().bright_red(),
                    url.name,
                    format!("Not Found").bright_red().bold()
                );
            }
        }
    }

    Ok(())
}
