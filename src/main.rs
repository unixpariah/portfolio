use askama::Template;
use axum::{Router, response::Html, routing::get};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    repos: &'a [Repo],
}

struct Repo {
    name: &'static str,
    description: &'static str,
    github: &'static str,
    host: Option<&'static str>,
}

async fn root() -> Html<String> {
    let repos = &[
        Repo {
            name: "nixconf",
            description: "Modular config for my system and HA k3s cluster",
            github: "https://forgejo.r0chd.pl/r0chd/nixconf",
            host: Some("https://r0chd.pl/"),
        },
        Repo {
            name: "whydotool",
            description: "Wayland-native command-line automation tool ",
            github: "https://forgejo.r0chd.pl/r0chd/whydotool",
            host: None,
        },
        Repo {
            name: "moxpaper",
            description: "Wallpaper daemon with fully customizable animations",
            github: "https://forgejo.r0chd.pl/mox-desktop/moxpaper",
            host: None,
        },
        Repo {
            name: "moxnotify",
            description: "Feature-rich hardware-accelerated keyboard driven Wayland notification daemon.",
            github: "https://forgejo.r0chd.pl/mox-desktop/moxnotify",
            host: None,
        },
        Repo {
            name: "moxidle",
            description: "Idle daemon with conditional timeouts and built in audio inhibitor",
            github: "https://forgejo.r0chd.pl/mox-desktop/moxidle",
            host: None,
        },
        Repo {
            name: "moxui",
            description: "wip ui library for mox desktop environment",
            github: "https://forgejo.r0chd.pl/mox-desktop/moxui",
            host: None,
        },
    ];

    Html(IndexTemplate { repos }.render().unwrap())
}
