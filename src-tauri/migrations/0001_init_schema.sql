PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    root_path TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS notes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    uri TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    last_modified INTEGER NOT NULL,
    created_at INTEGER,
    tags TEXT NOT NULL,
    body TEXT NOT NULL,
    FOREIGN KEY(project_id) REFERENCES projects(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_notes_project_id ON notes(project_id);

CREATE VIRTUAL TABLE IF NOT EXISTS notes_fts USING fts5(
    name,
    body,
    content='notes',
    content_rowid='id',
    tokenize='unicode61'
);

CREATE TRIGGER IF NOT EXISTS notes_ai AFTER INSERT ON notes BEGIN
    INSERT INTO notes_fts(rowid, name, body) VALUES (new.id, new.name, new.body);
END;

CREATE TRIGGER IF NOT EXISTS notes_ad AFTER DELETE ON notes BEGIN
    INSERT INTO notes_fts(notes_fts, rowid, name, body) VALUES('delete', old.id, old.name, old.body);
END;

CREATE TRIGGER IF NOT EXISTS notes_au AFTER UPDATE ON notes BEGIN
    INSERT INTO notes_fts(notes_fts, rowid, name, body) VALUES('delete', old.id, old.name, old.body);
    INSERT INTO notes_fts(rowid, name, body) VALUES (new.id, new.name, new.body);
END;

CREATE TABLE IF NOT EXISTS link_previews (
    url TEXT PRIMARY KEY,
    title TEXT,
    description TEXT,
    image_url TEXT,
    fetched_at INTEGER NOT NULL
);
