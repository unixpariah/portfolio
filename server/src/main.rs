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
struct Stats {
    name: String,
    bio: Option<String>,
    avatar_url: String,
    company: Option<String>,
    public_repos: i32,
    followers: i32,
    following: i32,
    location: String,
    hireable: bool,
}

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
    pub line_count: i32,
}

static SECRET: LazyLock<String> = LazyLock::new(|| {
    let secret = std::fs::read_to_string("/run/secrets/github_api").unwrap();
    let secret = secret.trim();
    assert!(!secret.is_empty(), "Github api token must not be empty");
    secret.to_string()
});

#[get("/stats")]
async fn get_stats(pool: web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    let db::DbAction::StatsRetrieved(stats) = db::execute(&pool, db::Query::GetStats).await? else {
        return HttpResponse::Ok().await;
    };

    Ok(HttpResponse::Ok().json(stats))
}

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
            let mut interval = interval(Duration::from_secs(86400));
            let client = Client::new();

            loop {
                interval.tick().await;

                let (repos_response, stats_response) = tokio::join!(
                    client
                        .get("https://api.github.com/users/unixpariah/repos")
                        .header("Accept", "application/vnd.github+json")
                        .header("Authorization", format!("Bearer {}", SECRET.as_str()))
                        .header("User-Agent", "portfolio")
                        .send(),
                    client
                        .get("https://api.github.com/users/unixpariah")
                        .header("Authorization", format!("Bearer {}", SECRET.as_str()))
                        .header("User-Agent", "portfolio")
                        .send()
                );

                if let (Ok(repos_res), Ok(stats_res)) = (repos_response, stats_response) {
                    match repos_res.json::<Value>().await {
                        Ok(Value::Array(repos)) => {
                            let project_futures = repos.iter().filter_map(|repo| {
                                if repo
                                    .get("fork")
                                    .and_then(|v| v.as_bool())
                                    .unwrap_or_default()
                                {
                                    return None;
                                }

                                let client = client.clone();
                                let secret = SECRET.clone();

                                Some(async move {
                                    let (id, name, desc, stars, url, homepage) = (
                                        repo.get("id")
                                            .and_then(|v| v.as_i64())
                                            .map(|v| v as i32)?,
                                        repo.get("name").and_then(|v| v.as_str())?,
                                        repo.get("description").and_then(|v| v.as_str())?,
                                        repo.get("stargazers_count")
                                            .and_then(|v| v.as_i64())
                                            .map(|v| v as i32)?,
                                        repo.get("html_url").and_then(|v| v.as_str())?,
                                        repo.get("homepage").and_then(|v| v.as_str()),
                                    );

                                    let lang_res = client
                                        .get(format!(
                                            "https://api.github.com/repos/unixpariah/{}/languages",
                                            name
                                        ))
                                        .header("Authorization", format!("Bearer {}", secret))
                                        .header("User-Agent", "portfolio")
                                        .send()
                                        .await
                                        .ok()?;

                                    let mut lang_json = lang_res.json::<Value>().await.ok()?;
                                    let languages = if let Value::Object(map) = lang_json.take() {
                                        map.into_iter()
                                            .map(|(k, v)| Language {
                                                name: k,
                                                line_count: v
                                                    .as_i64()
                                                    .map(|v| v as i32)
                                                    .unwrap_or_default(),
                                            })
                                            .collect()
                                    } else {
                                        Vec::new()
                                    };

                                    Some(Project {
                                        id,
                                        name: name.to_string(),
                                        description: desc.to_string(),
                                        stars,
                                        languages,
                                        url: url.to_string(),
                                        homepage: homepage
                                            .filter(|s| !s.is_empty())
                                            .map(String::from),
                                    })
                                })
                            });

                            let projects = future::join_all(project_futures)
                                .await
                                .into_iter()
                                .flatten()
                                .collect::<Vec<_>>();

                            if let Err(e) =
                                db::execute(&pool, db::Query::UpdateProjects(projects)).await
                            {
                                log::error!("Failed to update projects: {}", e);
                            }
                        }
                        Ok(_) => log::warn!("Unexpected repositories response format"),
                        Err(e) => log::error!("Failed to parse repositories: {}", e),
                    }

                    match stats_res.json::<Value>().await {
                        Ok(stats_json) => {
                            let stats = Stats {
                                name: stats_json
                                    .get("name")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or_default()
                                    .to_string(),
                                bio: stats_json
                                    .get("bio")
                                    .and_then(|v| v.as_str())
                                    .map(String::from),
                                avatar_url: stats_json
                                    .get("avatar_url")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or_default()
                                    .to_string(),
                                company: stats_json
                                    .get("company")
                                    .and_then(|v| v.as_str())
                                    .map(String::from),
                                public_repos: stats_json
                                    .get("public_repos")
                                    .and_then(|v| v.as_i64())
                                    .map(|v| v as i32)
                                    .unwrap_or_default(),
                                followers: stats_json
                                    .get("followers")
                                    .and_then(|v| v.as_i64())
                                    .map(|v| v as i32)
                                    .unwrap_or_default(),
                                following: stats_json
                                    .get("following")
                                    .and_then(|v| v.as_i64())
                                    .map(|v| v as i32)
                                    .unwrap_or_default(),
                                location: stats_json
                                    .get("location")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or_default()
                                    .to_string(),
                                hireable: stats_json
                                    .get("hireable")
                                    .and_then(|v| v.as_bool())
                                    .unwrap_or_default(),
                            };

                            if let Err(e) = db::execute(&pool, db::Query::UpdateStats(stats)).await
                            {
                                log::error!("Failed to update stats: {}", e);
                            }
                        }
                        Err(e) => log::error!("Failed to parse stats: {}", e),
                    }
                } else {
                    log::error!("Failed to fetch initial data");
                }
            }
        });
    }

    HttpServer::new(move || {
        let cors = Cors::default()
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
            .service(get_stats)
    })
    .bind(("0.0.0.0", 8000))?
    .workers(2)
    .run()
    .await
}
