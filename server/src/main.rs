mod db;

use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{
    App, HttpResponse, HttpServer, get,
    middleware::{self, DefaultHeaders},
    rt::time::interval,
    web,
};
use futures::future;
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{sync::LazyLock, time::Duration};

#[derive(Debug, Serialize)]
struct Project {
    id: i32,
    name: String,
    description: String,
    stars: i32,
    url: String,
    homepage: Option<String>,
    languages: Vec<Language>,
}

#[derive(Debug, Serialize)]
pub struct Language {
    pub name: String,
    pub line_count: i64,
}

static SECRET: LazyLock<String> = LazyLock::new(|| {
    let secret = std::fs::read_to_string("/home/unixpariah/.config/sops-nix/secrets/github-api")
        .unwrap_or_default();
    let secret = secret.trim();
    assert!(!secret.is_empty(), "Github api token must not be empty");
    secret.to_string()
});

#[get("/projects")]
async fn get_projects(pool: web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    let db::DbAction::ProjectsRetrieved(projects) =
        db::execute(&pool, db::Query::GetProjects).await?
    else {
        return HttpResponse::Ok().await;
    };

    Ok(HttpResponse::Ok().json(projects))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env var not set");
    log::info!("Connecting to database at: {}", db_url);

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(300))
        .connect(&db_url)
        .await
        .unwrap();

    {
        let pool = pool.clone();
        tokio::spawn(async {
            let pool = pool;
            let mut interval = interval(Duration::from_secs(3600));
            let client = Client::new();

            loop {
                interval.tick().await;
                let res = client
                    .get("https://api.github.com/users/unixpariah/repos")
                    .header("Accept", "application/vnd.github+json")
                    .header("Authorization", format!("Bearer {}", SECRET.as_str()))
                    .header("User-Agent", "portfolio")
                    .send()
                    .await;

                if let Ok(response) = res {
                    if let Ok(json) = response.json::<Value>().await {
                        if let Some(repos) = json.as_array() {
                            let futures = repos.iter().filter_map(|repo| {
                                let client = client.clone();
                                let secret = SECRET.clone();

                                if let Some(fork) = repo.get("fork") {
                                    if fork.as_bool().unwrap() {
                                        return None;
                                    }
                                }

                                if let (Some(id), Some(name), Some(description), Some(stars), Some(url), Some(homepage)) = (
                                    repo.get("id").and_then(|v| v.as_i64().map(|v| v as i32)),
                                    repo.get("name"),
                                    repo.get("description"),
                                    repo.get("stargazers_count")
                                        .and_then(|v| v.as_i64().map(|v| v as i32)),
                                    repo.get("html_url"),
                                    repo.get("homepage")
                                ) {
                                    Some(async move {
                                        let name_str = name.as_str().unwrap_or_default();
                                        let Ok(res) = client
                                            .get(format!("https://api.github.com/repos/unixpariah/{}/languages", name_str))
                                            .header("Authorization", format!("Bearer {}", secret))
                                            .header("User-Agent", "portfolio")
                                            .send()
                                            .await else {
                                            return None;
                                        };

                                        let Ok(mut json) = res.json::<Value>().await else {
                                            return None;
                                        };

                                        let languages = match json.take() {
                                            Value::Object(map) => map
                                                .into_iter()
                                                .map(|(k, v)| Language {name: k, line_count: v.as_i64().unwrap_or_default()})
                                                .collect::<Vec<_>>(),
                                            _ => Vec::new(),
                                        };

                                        let homepage = match homepage {
                                                Value::String(s) if !s.trim().is_empty() => Some(s.clone()),
                                                _ => None
                                            };


                                        Some(Project {
                                            id,
                                            name: name.to_string(),
                                            description: description.to_string(),
                                            stars,
                                            languages,
                                            url: url.to_string(),
                                            homepage,
                                        })
                                    })
                                } else {
                                    None
                                }
                            });

                            let projects: Vec<Project> = future::join_all(futures)
                                .await
                                .into_iter()
                                .flatten()
                                .collect();

                            if let Err(e) =
                                db::execute(&pool, db::Query::UpdateProjects(projects)).await
                            {
                                log::error!("{e}");
                            }
                        }
                    }
                }
            }
        });
    }

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://app.localhost")
            .allowed_methods(vec!["POST", "GET"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
            ])
            .allowed_header(actix_web::http::header::CONTENT_TYPE)
            .max_age(3600);

        let governor_conf = GovernorConfigBuilder::default()
            .seconds_per_request(2)
            .burst_size(5)
            .finish()
            .unwrap();

        App::new()
            .wrap(Governor::new(&governor_conf))
            .wrap(
                DefaultHeaders::new()
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add(("X-Frame-Options", "DENY"))
                    .add(("X-XSS-Protection", "1; mode=block")),
            )
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(get_projects)
    })
    .bind(("0.0.0.0", 8000))?
    .workers(2)
    .run()
    .await
}
