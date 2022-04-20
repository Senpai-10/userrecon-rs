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

    let mut home_dir = String::new();
    let mut db_dir = String::new();
    let mut db_file = String::new();

    match whoami::platform() {
        whoami::Platform::Linux => {
            home_dir = format!("/home/{}/", whoami::username());
            db_dir = format!("{}/.config/userrecon-rs", home_dir);
            db_file = format!("{}/urls.txt", db_dir);
        }
        whoami::Platform::Windows => {
            let home_dir = format!("C:\\Users\\{}", whoami::username());
            let db_dir = format!("{}\\userrecon-rs", home_dir);
            let db_file = format!("{}\\urls.txt", db_dir);
        }
        _ => {}
    }

    if std::path::Path::new(&db_dir).exists() == false {
        std::fs::create_dir_all(&db_dir)?;
    }
    if std::path::Path::new(&db_file).exists() == false {
        std::fs::write(&db_file, DB_FILE_CONTENTS)?;
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

const DB_FILE_CONTENTS: &str = r#"500px:https://500px.com/{}
About.me:https://about.me/{}
Angel:https://angel.co/{}
Badoo:https://www.badoo.com/en/{}
Bandcamp:https://www.bandcamp.com/{}
Basecamphq:https://{}.basecamphq.com/login
Behance:https://www.behance.net/{}
Bitbucket:https://bitbucket.org/{}
Blip.fm:https://blip.fm/{}
Blogspot:https://{}.blogspot.com
Buzzfeed:https://buzzfeed.com/{}
Canva:https://www.canva.com/{}
Cash:https://cash.me/{}
Codecademy:https://www.codecademy.com/{}
Codementor:https://www.codementor.io/{}
Colourlovers:https://www.colourlovers.com/love/{}
Contently:https://{}.contently.com
Creativemarket:https://creativemarket.com/{}
Dailymotion:https://www.dailymotion.com/{}
Deviantart:https://{}.deviantart.com
Disqus:https://disqus.com/{}
Dribbble:https://dribbble.com/{}
Ebay:https://www.ebay.com/usr/{}
Ello:https://ello.co/{}
Etsy:https://www.etsy.com/shop/{}
Facebook:https://www.facebook.com/{}
Flickr:https://www.flickr.com/people/{}
Flipboard:https://flipboard.com/@{}
Fotolog:https://fotolog.com/{}
Foursquare:https://foursquare.com/{}
Github:https://www.github.com/{}
Goodreads:https://www.goodreads.com/{}
Gravatar:https://en.gravatar.com/{}
Houzz:https://houzz.com/user/{}
Ifttt:https://www.ifttt.com/p/{}
Imgur:https://imgur.com/user/{}
Instagram:https://www.instagram.com/{}
Instructables:https://www.instructables.com/member/{}
Keybase:https://keybase.io/{}
Kongregate:https://kongregate.com/accounts/{}
Last.fm:https://last.fm/user/{}
Livejournal:https://{}.livejournal.com
Medium:https://medium.com/@{}
Mixcloud:https://www.mixcloud.com/{}
Newgrounds:https://{}.newgrounds.com
Okcupid:https://www.okcupid.com/profile/{}
Pastebin:https://pastebin.com/u/{}
Patreon:https://www.patreon.com/{}
Pinterest:https://www.pinterest.com/{}
Plus-google:https://plus.google.com/+{}/posts
Reddit:https://www.reddit.com/user/{}
Reverbnation:https://www.reverbnation.com/{}
Roblox:https://www.roblox.com/user.aspx?username={}
Scribd:https://www.scribd.com/{}
Slack:https://{}.slack.com
Slideshare:https://slideshare.net/{}
Soundcloud:https://soundcloud.com/{}
Spotify:https://open.spotify.com/user/{}
Steam:https://steamcommunity.com/id/{}
Trakt.tv:https://www.trakt.tv/users/{}
Tripit:https://www.tripit.com/people/{}
Trip.skyscanner:https://www.trip.skyscanner.com/user/{}
Tumblr:https://{}.tumblr.com
Twitter:https://www.twitter.com/{}
Vimeo:https://vimeo.com/{}
Wattpad:https://www.wattpad.com/user/{}
Wikipedia:https://www.wikipedia.org/wiki/User:{}
Wordpress:https://{}.wordpress.com
Ycombinator:https://news.ycombinator.com/user?id={}
Youtube:https://www.youtube.com/{}
"#;
