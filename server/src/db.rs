use crate::{Project, Stats};
use actix_web::error;
use sqlx::{PgPool, error::Error as SqlxError};

pub enum Query {
    GetStats,
    UpdateStats(Stats),
    UpdateProjects(Vec<Project>),
    GetProjects,
}

#[derive(Debug)]
pub enum DbAction {
    StatsUpdated,
    StatsRetrieved(Stats),
    ProjectsUpdated,
    ProjectsRetrieved(Vec<Project>),
}

pub async fn execute(pool: &PgPool, query: Query) -> Result<DbAction, actix_web::Error> {
    let result = match query {
        Query::UpdateProjects(projects) => update_projects(pool, projects).await,
        Query::GetProjects => get_projects(pool).await,
        Query::UpdateStats(stats) => update_stats(pool, stats).await,
        Query::GetStats => get_stats(pool).await,
    };
    result.map_err(error::ErrorInternalServerError)
}

async fn get_stats(pool: &PgPool) -> Result<DbAction, SqlxError> {
    let stats = sqlx::query_as!(
        Stats,
        r#"
            SELECT name, bio, avatar_url, company, public_repos, followers, following, location, hireable
            FROM stats
            LIMIT 1
        "#
    )
    .fetch_one(pool)
    .await?;

    Ok(DbAction::StatsRetrieved(stats))
}

async fn update_stats(pool: &PgPool, stats: Stats) -> Result<DbAction, SqlxError> {
    sqlx::query!(
        r#"
        INSERT INTO stats (id, name, bio, avatar_url, company, public_repos, followers, following, location, hireable)
        VALUES (1, $1, $2, $3, $4, $5, $6, $7, $8, $9)
        ON CONFLICT (id) DO UPDATE SET
            name = EXCLUDED.name,
            bio = EXCLUDED.bio,
            avatar_url = EXCLUDED.avatar_url,
            company = EXCLUDED.company,
            public_repos = EXCLUDED.public_repos,
            followers = EXCLUDED.followers,
            following = EXCLUDED.following,
            location = EXCLUDED.location,
            hireable = EXCLUDED.hireable
        "#,
        stats.name,
        stats.bio,
        stats.avatar_url,
        stats.company,
        stats.public_repos,
        stats.followers,
        stats.following,
        stats.location,
        stats.hireable
    )
    .execute(pool)
    .await?;

    Ok(DbAction::StatsUpdated)
}

async fn get_projects(pool: &PgPool) -> Result<DbAction, SqlxError> {
    let projects_without_languages = sqlx::query!(
        r#"
        SELECT id, name, description, stars, url, homepage
        FROM projects
        ORDER BY stars DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut projects = Vec::with_capacity(projects_without_languages.len());

    for project_row in projects_without_languages {
        let languages = sqlx::query!(
            r#"
            SELECT language, usage
            FROM project_languages
            WHERE project_id = $1
            ORDER BY usage DESC
            "#,
            project_row.id
        )
        .fetch_all(pool)
        .await?;

        let language_tuples = languages
            .into_iter()
            .map(|lang| crate::Language {
                name: lang.language,
                line_count: lang.usage,
            })
            .collect();

        let project = Project {
            id: project_row.id,
            name: project_row.name,
            description: project_row.description,
            stars: project_row.stars,
            languages: language_tuples,
            url: project_row.url,
            homepage: project_row.homepage,
        };

        projects.push(project);
    }

    Ok(DbAction::ProjectsRetrieved(projects))
}

async fn update_projects(pool: &PgPool, projects: Vec<Project>) -> Result<DbAction, SqlxError> {
    let mut tx = pool.begin().await?;

    for project in projects {
        sqlx::query!(
            r#"
            INSERT INTO projects (id, name, description, stars, url, homepage)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE
            SET name = EXCLUDED.name,
                description = EXCLUDED.description,
                stars = EXCLUDED.stars,
                url = EXCLUDED.url,
                homepage = EXCLUDED.homepage
            "#,
            project.id,
            project.name,
            project.description,
            project.stars,
            project.url,
            project.homepage,
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
            DELETE FROM project_languages 
            WHERE project_id = $1
            "#,
            project.id
        )
        .execute(&mut *tx)
        .await?;

        for language in project.languages {
            sqlx::query!(
                r#"
                INSERT INTO project_languages (project_id, language, usage)
                VALUES ($1, $2, $3)
                "#,
                project.id,
                language.name,
                language.line_count as i32,
            )
            .execute(&mut *tx)
            .await?;
        }
    }

    tx.commit().await?;

    Ok(DbAction::ProjectsUpdated)
}
