use dotenv;
use regex::Regex;
use roux::util::{FeedOption, TimePeriod};
use roux::Subreddit;
use serde::{Deserialize, Serialize};
use serde_json;
use std::error;
use std::fs;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
extern crate redis;
use redis::Commands;

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

    pub fn subreddit(all_memes: Arc<Mutex<Vec<Meme>>>, sub_reddit: &str) {
        let correct_links = Regex::new(r"^https://i.redd.it/").unwrap();

        let hot_options = FeedOption::new().period(TimePeriod::ThisMonth);
        let top_options = FeedOption::new().period(TimePeriod::AllTime);
        let subreddit = Subreddit::new(sub_reddit);
        //Hot category
        let hot = subreddit.hot(150, Some(hot_options)).unwrap().data.children;
        // ALl time top category

        let top = subreddit.top(100, Some(top_options)).unwrap().data.children;

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
                all_memes.lock().unwrap().push(new_meme);
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
                all_memes.lock().unwrap().push(new_meme);
            }
        }
    }
    pub fn collect_memes() -> Arc<Mutex<Vec<Meme>>> {
        let mut all_memes = Arc::new(Mutex::new(Vec::new()));
        // Collects memes from sub_reddit 1
        let memes1 = Arc::clone(&all_memes);
        let h1 = thread::spawn(move || {
            Meme::subreddit(memes1, dotenv::var("SUB_REDDIT_1").unwrap().as_str());
        });
        h1.join().unwrap();
        // Collects memes from sub_reddit 2
        let memes2 = Arc::clone(&all_memes);
        let h2 = thread::spawn(move || {
            Meme::subreddit(memes2, dotenv::var("SUB_REDDIT_2").unwrap().as_str());
        });

        let memes3 = Arc::clone(&all_memes);
        let h3 = thread::spawn(move || {
            Meme::subreddit(memes3, dotenv::var("SUB_REDDIT_3").unwrap().as_str());
        });
        let memes4 = Arc::clone(&all_memes);
        let h4 = thread::spawn(move || {
            Meme::subreddit(memes4, dotenv::var("SUB_REDDIT_4").unwrap().as_str());
        });
        h2.join().unwrap();
        h3.join().unwrap();
        h4.join().unwrap();
        return all_memes;
    }
    //it takes 16 seconds
    pub async fn cache_response() -> Option<Vec<Meme>> {
        let memes_vec = Arc::try_unwrap(Meme::collect_memes()).unwrap();
        let memes = memes_vec.into_inner().unwrap();
        let time_to_live: usize = 6 * 60 * 60;
        let memes_string_vec = serde_json::to_string(&memes).unwrap();
        let con_uri = dotenv::var("REDIS").unwrap();
        let client = redis::Client::open(con_uri).unwrap();
        let mut con = client.get_connection().unwrap();
        let _: () = con.set("all_memes", memes_string_vec).unwrap();
        let _: () = con.expire("all_memes", time_to_live).unwrap();
        Some(memes)
    }
}
