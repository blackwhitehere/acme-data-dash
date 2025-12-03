#!/bin/bash
set -e

DB_FILE="data_dash.db"

echo "Resetting database..."
if [ -f "$DB_FILE" ]; then
    rm "$DB_FILE"
    echo "Removed existing $DB_FILE"
fi

# Ensure sqlite3 is installed
if ! command -v sqlite3 &> /dev/null; then
    echo "Error: sqlite3 is not installed."
    exit 1
fi

echo "Creating schema..."
sqlite3 "$DB_FILE" < scripts/schema.sql

echo "Seeding data..."
sqlite3 "$DB_FILE" < scripts/seed_data.sql

echo "Database reset complete."
