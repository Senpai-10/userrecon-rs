use colored::Colorize;
use reqwest;
use std::process::exit;

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
            "{} Please provide a username!{}{}",
            format!("[ERROR]").bright_red(),
            "\n",
            HELP_MESSAGE,
        );
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
        } else if username.is_empty() && !arg.starts_with("-") {
            username = arg;
        }
    }

    if !clean_output {
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

    let mut urls: Vec<Urls> = Vec::new();

    // loading urls into urls vec
    let contents = DB_FILE_CONTENTS;

    for line in contents.lines() {
        if !line.starts_with("//") && !line.starts_with("#") {
            let line = line.split_once(':').unwrap();

            urls.push(Urls {
                name: line.0.to_string(),
                url: line.1.to_string(),
            });
        }
    }

    if !clean_output {
        println!(
            "{} Checking username {}",
            format!("[*]").bold().bright_yellow(),
            username.bold()
        );
    }

    for url in urls {
        let new_url = url.url.replace("{}", username);

        match reqwest::get(&new_url).await {
            Ok(res) => {
                if res.status() == 200 {
                    if !clean_output {
                        println!(
                            "{} {}: {}",
                            format!("[+]").bold().bright_green(),
                            url.name,
                            new_url.bold()
                        );
                    } else if clean_output {
                        println!("{}", new_url);
                    }
                } else {
                    if !clean_output {
                        println!(
                            "{} {}: {}",
                            format!("[-]").bold().bright_red(),
                            url.name,
                            format!("Not Found").bright_red().bold()
                        );
                    }
                }
            }
            Err(_) => {
                if !clean_output {
                    println!(
                        "{} {}: {}",
                        format!("[?]").bold().bright_blue(),
                        url.name,
                        format!("{}", NOT_AVAILABLE).bold().bright_blue(),
                    );
                } else if clean_output {
                    println!("{} - {}", new_url, NOT_AVAILABLE);
                }
            }
        }
    }

    Ok(())
}

const NOT_AVAILABLE: &str = "Current Not Available";

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
GitHub:https://www.github.com/{}
GitLab:https://gitlab.com/{}
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
