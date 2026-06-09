CREATE TABLE IF NOT EXISTS weight_entries (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    weight_kg   REAL    NOT NULL,
    date        TEXT    NOT NULL,
    notes       TEXT,
    created_at  TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS meditation_sessions (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    date          TEXT    NOT NULL,
    time          TEXT    NOT NULL,
    duration_min  INTEGER NOT NULL,
    notes         TEXT,
    created_at    TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at    TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS feeling_entries (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    date        TEXT    NOT NULL,
    content     TEXT    NOT NULL,
    mood_score  INTEGER,
    created_at  TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS ai_tips (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    data_hash       TEXT    NOT NULL,
    response        TEXT    NOT NULL,
    entries_count   INTEGER NOT NULL DEFAULT 0,
    created_at      TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_weight_date ON weight_entries(date);
CREATE INDEX IF NOT EXISTS idx_meditation_date ON meditation_sessions(date);
CREATE INDEX IF NOT EXISTS idx_feeling_date ON feeling_entries(date);
