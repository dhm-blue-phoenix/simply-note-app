-- Categories Table
CREATE TABLE IF NOT EXISTS categories (
    id TEXT PRIMARY KEY NOT NULL,
    category TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL,
    updated_at TEXT
);

-- Notes Table
CREATE TABLE IF NOT EXISTS notes (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT not null,
    content TEXT,
    status TEXT DEFAULT 'notes' CHECK ( status IN ( 'notes', 'trash' ) ),
    category_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT,
    FOREIGN KEY (category_id) REFERENCES categories(id)
);