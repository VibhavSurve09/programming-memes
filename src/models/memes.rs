use dotenv;
use roux::util::{FeedOption, TimePeriod};
use roux::Subreddit;
use serde::{Deserialize, Serialize};
use serde_json;
use std::error;
use std::fs;
use std::io::prelude::*;
#[derive(Debug, Deserialize, Serialize)]
pub struct Meme {
    title: String,
    link: String,
}
impl Meme {
    pub fn new_meme(title: String, link: String) -> Self {
        Meme { title, link }
    }
    pub async fn collect_memes() -> Vec<Meme> {
        let mut all_memes = Vec::new();
        let hot_options = FeedOption::new().period(TimePeriod::ThisWeek);
        let top_options = FeedOption::new().period(TimePeriod::AllTime);
        let subreddit = Subreddit::new(dotenv::var("SUB_REDDIT_1").unwrap().as_str());
        //Hot category
        let hot = subreddit.hot(150, None).await.unwrap().data.children;
        // ALl time top category
        let top = subreddit.top(100, None).await.unwrap().data.children;
        let rising = subreddit.rising(100, None).await.unwrap().data.children;
        for posts in hot {
            let new_meme = Meme::new_meme(posts.data.title, posts.data.url.unwrap());
            all_memes.push(new_meme);
        }
        for posts in top {
            let new_meme = Meme::new_meme(posts.data.title, posts.data.url.unwrap());
            all_memes.push(new_meme);
        }
        for posts in rising {
            let new_meme = Meme::new_meme(posts.data.title, posts.data.url.unwrap());
            all_memes.push(new_meme);
        }
        return all_memes;
    }

    pub async fn cache_response() -> Result<Option<Vec<Meme>>, Box<dyn error::Error>> {
        let mut reponse = fs::File::open("memes.json");
        if let Ok(file) = reponse {
            return Ok(None);
        }
        let memes_vec = Meme::collect_memes().await;
        let str = serde_json::to_string(&memes_vec).unwrap();
        fs::write("memes.json", str);
        return Ok(Some(memes_vec));
    }
}
