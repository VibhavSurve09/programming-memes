use dotenv;
use regex::Regex;
use roux::util::{FeedOption, TimePeriod};
use roux::Subreddit;
use serde::{Deserialize, Serialize};
use serde_json;
use std::error;
use std::fs;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Meme {
    title: String,
    pub link: String,
    is_safe: bool,
    subreddit_name: String,
    up_votes: f64,
    down_votes: f64,
}
impl Meme {
    pub fn new_meme(
        title: String,
        link: String,
        over_18: bool,
        name: String,
        ups: f64,
        down: f64,
    ) -> Self {
        Meme {
            title,
            link,
            is_safe: !over_18,
            subreddit_name: name,
            up_votes: ups,
            down_votes: down,
        }
    }
    pub async fn subreddit(all_memes: &mut Vec<Meme>, sub_reddit: &str) {
        let correct_links = Regex::new(r"^https://i.redd.it/").unwrap();

        let hot_options = FeedOption::new().period(TimePeriod::ThisMonth);
        let top_options = FeedOption::new().period(TimePeriod::AllTime);
        let subreddit = Subreddit::new(sub_reddit);
        //Hot category
        let hot = subreddit
            .hot(150, Some(hot_options))
            .await
            .unwrap()
            .data
            .children;
        // ALl time top category
        let top = subreddit
            .top(100, Some(top_options))
            .await
            .unwrap()
            .data
            .children;
        let rising = subreddit.rising(100, None).await.unwrap().data.children;
        for posts in hot {
            let link = posts.data.url;
            if correct_links.is_match(link.clone().unwrap().as_str()) {
                let new_meme = Meme::new_meme(
                    posts.data.title,
                    link.unwrap(),
                    posts.data.over_18,
                    posts.data.subreddit,
                    posts.data.ups,
                    posts.data.downs,
                );
                all_memes.push(new_meme);
            }
        }
        for posts in top {
            let link = posts.data.url;
            if correct_links.is_match(link.clone().unwrap().as_str()) {
                let new_meme = Meme::new_meme(
                    posts.data.title,
                    link.unwrap(),
                    posts.data.over_18,
                    posts.data.subreddit,
                    posts.data.ups,
                    posts.data.downs,
                );
                all_memes.push(new_meme);
            }
        }
        for posts in rising {
            let link = posts.data.url;
            if correct_links.is_match(link.clone().unwrap().as_str()) {
                let new_meme = Meme::new_meme(
                    posts.data.title,
                    link.unwrap(),
                    posts.data.over_18,
                    posts.data.subreddit,
                    posts.data.ups,
                    posts.data.downs,
                );
                all_memes.push(new_meme);
            }
        }
    }
    pub async fn collect_memes() -> Vec<Meme> {
        let mut all_memes = Vec::new();
        // Collects memes from sub_reddit 1
        Meme::subreddit(
            &mut all_memes,
            dotenv::var("SUB_REDDIT_1").unwrap().as_str(),
        )
        .await;
        // Collects memes from sub_reddit 2
        Meme::subreddit(
            &mut all_memes,
            dotenv::var("SUB_REDDIT_2").unwrap().as_str(),
        )
        .await;
        Meme::subreddit(
            &mut all_memes,
            dotenv::var("SUB_REDDIT_3").unwrap().as_str(),
        )
        .await;
        Meme::subreddit(
            &mut all_memes,
            dotenv::var("SUB_REDDIT_4").unwrap().as_str(),
        )
        .await;
        return all_memes;
    }
    /// This function is called when memes.json file is not found
    pub async fn cache_response() -> Result<Option<Vec<Meme>>, std::io::Error> {
        let memes_vec = Meme::collect_memes().await;
        let str = serde_json::to_string(&memes_vec).unwrap();
        let output = fs::write("memes.json", str); //If file is not found it creates a file named memes.json
                                                   //Delete memes.json after a day
        thread::spawn(|| {
            thread::sleep(Duration::from_secs(86400));
            fs::remove_file("memes.json");
        });
        match output {
            Ok(_) => {
                return Ok(Some(memes_vec));
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
}
