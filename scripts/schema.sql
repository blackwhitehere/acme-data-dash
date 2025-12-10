CREATE TABLE IF NOT EXISTS check_results (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    check_id TEXT NOT NULL,
    status TEXT NOT NULL,
    message TEXT,
    details TEXT,
    executed_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS connection_profiles (
    name TEXT PRIMARY KEY,
    driver TEXT NOT NULL,
    connection_string_template TEXT NOT NULL,
    connection_type TEXT DEFAULT 'database',
    secret_ref TEXT
);

CREATE TABLE IF NOT EXISTS secrets (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS data_sources (
    name TEXT PRIMARY KEY,
    connection_name TEXT NOT NULL,
    secret_key TEXT NOT NULL,
    is_valid BOOLEAN DEFAULT 1,
    FOREIGN KEY (connection_name) REFERENCES connection_profiles(name),
    FOREIGN KEY (secret_key) REFERENCES secrets(key)
);

CREATE TABLE IF NOT EXISTS unix_groups (
    group_name TEXT PRIMARY KEY,
    file_path TEXT NOT NULL,
    permissions TEXT NOT NULL
);
