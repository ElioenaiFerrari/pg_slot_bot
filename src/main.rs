use std::time::Duration;

use futures::{SinkExt, StreamExt};
use warp::cors;
use warp::filters::ws::Message;
use warp::ws::WebSocket;
use warp::{Filter, Reply};
use ws::Result;

#[derive(Debug, serde::Serialize)]
struct Game {
    pub game: String,
    pub image_url: String,
    pub percentage: i32,
}

pub async fn client_connection(ws: WebSocket) {
    println!("establishing client connection... {:?}", ws);
}

#[derive(Debug, serde::Serialize)]
struct Response {
    pub games: Vec<Game>,
    pub best_times: Vec<&'static str>,
}

async fn fetch_games() -> Response {
    let url = "https://infobocorangacor.net";
    let response = reqwest::get(format!("{url}/pg.php?ref=AQUASLOTOFFICIAL"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let document = scraper::Html::parse_document(&response);
    let items_selector = scraper::Selector::parse(".items").unwrap();
    let items = document.select(&items_selector).collect::<Vec<_>>();

    let mut games: Vec<Game> = items.into_iter().fold(vec![], |mut acc, item| {
        let game_name_selector = scraper::Selector::parse(".game-name").unwrap();
        let game_img_selector = scraper::Selector::parse(".game-img").unwrap();
        let game_percentage_selector = scraper::Selector::parse("[class^=\"percent\"]").unwrap();

        let game_name = item
            .select(&game_name_selector)
            .next()
            .and_then(|x| x.text().next())
            .unwrap_or_default(); // Use a default value if not found
        let game_img = item
            .select(&game_img_selector)
            .next()
            .and_then(|x| x.value().attr("src"))
            .unwrap_or_default(); // Use a default value if not found
        let game_percentage = item
            .select(&game_percentage_selector)
            .next()
            .and_then(|x| x.value().attr("class"))
            .unwrap_or_default()
            .split("percent")
            .last()
            .unwrap_or_default()
            .parse::<i32>()
            .unwrap_or(0); // Use a default value if parsing fails

        acc.push(Game {
            game: game_name.to_string(),
            image_url: format!("{url}/{game_img}"),
            percentage: game_percentage,
        });
        acc
    });

    games.sort_by(|a, b| b.percentage.cmp(&a.percentage));

    Response {
        games: games,
        best_times: vec![
            "14:04", "14:05", "14:07", "14:08", "14:09", "14:10", "14:11", "14:13", "14:14",
            "14:15", "14:16", "14:17", "14:19", "14:20", "14:21", "14:22", "14:23", "14:24",
            "14:25", "14:26", "14:28", "14:29", "14:30", "14:31", "14:32", "14:33", "14:35",
            "14:37", "14:39", "14:43", "14:44", "14:45", "14:46", "14:47", "14:48", "14:49",
            "14:51", "14:52", "14:54", "14:55", "14:58", "14:59",
        ],
    }
}

pub async fn ws_handler(ws: warp::ws::Ws) -> Result<impl Reply> {
    println!("ws_handler");
    Ok(ws.on_upgrade(move |socket| client_connection(socket)))
}

async fn handle_websocket(ws: WebSocket) {
    // Define a simple handler that echoes received messages back to the client.
    let (mut tx, _) = ws.split();

    let mut interval = tokio::time::interval(Duration::from_secs(30));
    loop {
        interval.tick().await;
        let response = fetch_games().await;
        let msg = serde_json::to_string(&response).unwrap();
        tx.send(Message::text(msg)).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    println!("Configuring websocket route");

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(handle_websocket));
    let routes = warp::path("api")
        .and(ws_route)
        .or(warp::fs::dir("assets"))
        .with(cors().allow_any_origin().build());

    println!("Starting server");

    warp::serve(routes).run(([0, 0, 0, 0], 4001)).await;
}
