CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    stars INTEGER NOT NULL,
    url TEXT NOT NULL,
    homepage TEXT
);

CREATE TABLE project_languages (
    project_id INTEGER NOT NULL,
    language TEXT NOT NULL,
    usage INTEGER NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

CREATE TABLE stats (
    id INTEGER PRIMARY KEY DEFAULT 1,
    name TEXT NOT NULL,
    bio TEXT,
    avatar_url TEXT NOT NULL,
    company TEXT,
    public_repos INTEGER NOT NULL,
    followers INTEGER NOT NULL,
    following INTEGER NOT NULL,
    location TEXT NOT NULL,
    hireable BOOLEAN NOT NULL
);
