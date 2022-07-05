use roux::Reddit;
use roux::Subreddit;
#[tokio::main]
async fn main() {
    // let client = Reddit::new("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/83.0.4103.97 Safari/537.36", "ULFNFmODJqubx5N9tJfeDw", "sb6GUn46jO8Ua6gWWtmlP68hWrqERw")
    //     .username("__HERE_FOR_MEMES__")
    //     .password("Fakepassword")
    //     .login()
    //     .await;
    // let me = client.unwrap();
    let subreddit = Subreddit::new("ProgrammerHumor");
    // Now you are able to:

    // Get moderators.

    // Get hot posts with limit = 25.
    let hot = subreddit.hot(1000, None).await.unwrap().data.children;
    for posts in hot {
        println!("{:?}", posts.data.thumbnail);
    }
}
