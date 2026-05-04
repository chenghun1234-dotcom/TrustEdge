-- TrustEdge D1 Database Schema
CREATE TABLE IF NOT EXISTS audit_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp INTEGER NOT NULL,
    assets REAL NOT NULL,
    liabilities REAL NOT NULL,
    ratio REAL NOT NULL,
    proof TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS publishers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    bank_api_key TEXT,
    wallet_address TEXT,
    created_at INTEGER
);
