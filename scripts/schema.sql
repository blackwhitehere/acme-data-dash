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
    secret_ref TEXT
);

CREATE TABLE IF NOT EXISTS secrets (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
