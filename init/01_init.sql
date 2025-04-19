CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    stars INTEGER NOT NULL
);

CREATE TABLE project_languages (
    project_id INTEGER NOT NULL,
    language TEXT NOT NULL,
    usage INTEGER NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);
