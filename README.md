
# Programming Memes Reddit

Well who doesn't like memes?
## API Reference

#### Get all memes

```
BASE_URL: https://memes.hellomemes.workers.dev
```
```http
  GET /
```
```http
curl --request GET http://152.67.11.203:8000/
```

Example response:

```json
{
"title":"that is not allowed for you"
"link":"https://i.redd.it/7tpnzz993rb91.jpg"
"is_safe":true
"subreddit_name":"ProgrammerHumor"
"up_votes":21390
"down_votes":0
}
```
#### Get a random meme

```http
  GET  <base_url>/random
```
Example response:

![image2](https://user-images.githubusercontent.com/73296863/179342826-528c3ffc-21a5-4449-8ea4-f408c7402bc2.jpg)



## Tech Stack

**Server:** Actix Web

