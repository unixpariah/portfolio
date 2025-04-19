use crate::Project;
use actix_web::error;
use sqlx::{PgPool, error::Error as SqlxError};

pub enum Query {
    UpdateProjects(Vec<Project>),
    GetProjects,
}

#[derive(Debug)]
pub enum DbAction {
    ProjectsUpdated,
    ProjectsRetrieved(Vec<Project>),
}

pub async fn execute(pool: &PgPool, query: Query) -> Result<DbAction, actix_web::Error> {
    let result = match query {
        Query::UpdateProjects(projects) => update_projects(pool, projects).await,
        Query::GetProjects => get_projects(pool).await,
    };
    result.map_err(error::ErrorInternalServerError)
}

async fn get_projects(pool: &PgPool) -> Result<DbAction, SqlxError> {
    // First, fetch all projects
    let projects_without_languages = sqlx::query!(
        r#"
        SELECT id, name, description, stars
        FROM projects
        ORDER BY stars DESC
        "#
    )
    .fetch_all(pool)
    .await?;
    
    // Then create the vector for our final projects with languages
    let mut projects = Vec::with_capacity(projects_without_languages.len());
    
    // For each project, fetch its languages and build the complete Project struct
    for project_row in projects_without_languages {
        // Fetch languages for this project
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
        
        // Convert database rows to the expected format
        let language_tuples = languages
            .into_iter()
            .map(|lang| (lang.language, lang.usage as i64))
            .collect();
        
        // Construct the complete Project with languages
        let project = Project {
            id: project_row.id,
            name: project_row.name,
            description: project_row.description,
            stars: project_row.stars,
            languages: language_tuples,
        };
        
        projects.push(project);
    }
    
    Ok(DbAction::ProjectsRetrieved(projects))
}

async fn update_projects(pool: &PgPool, projects: Vec<Project>) -> Result<DbAction, SqlxError> {
    // Use a transaction to ensure atomicity
    let mut tx = pool.begin().await?;
    
    for project in projects {
        // Insert or update the project first
        sqlx::query!(
            r#"
            INSERT INTO projects (id, name, description, stars)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO UPDATE
            SET name = EXCLUDED.name,
                description = EXCLUDED.description,
                stars = EXCLUDED.stars
            "#,
            project.id,
            project.name,
            project.description,
            project.stars,
        )
        .execute(&mut *tx)
        .await?;
        
        // First, delete existing languages for this project to avoid duplicates
        sqlx::query!(
            r#"
            DELETE FROM project_languages 
            WHERE project_id = $1
            "#,
            project.id
        )
        .execute(&mut *tx)
        .await?;
        
        // Then insert all languages for this project
        for (language, usage) in project.languages {
            sqlx::query!(
                r#"
                INSERT INTO project_languages (project_id, language, usage)
                VALUES ($1, $2, $3)
                "#,
                project.id,
                language,
                usage as i32,  // Assuming your DB column is INTEGER
            )
            .execute(&mut *tx)
            .await?;
        }
    }
    
    // Commit the transaction
    tx.commit().await?;
    
    Ok(DbAction::ProjectsUpdated)
}
