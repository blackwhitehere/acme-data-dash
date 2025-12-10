#!/bin/bash

# Set up environment
export DATABASE_URL=sqlite:data_dash.db

# Reset DB
./scripts/reset_db.sh

# Function to kill background processes on exit
cleanup() {
    echo "Stopping services..."
    kill $(jobs -p) 2>/dev/null
}
trap cleanup EXIT

# Kill any processes using ports 3000 and 5173
echo "Checking for processes using ports 3000 and 5173..."
lsof -ti:3000 | xargs kill -9 2>/dev/null || true
lsof -ti:5173 | xargs kill -9 2>/dev/null || true
sleep 1

echo "Starting Acme Data Dash in DEV mode..."

# 1. Start Backend (Rust)
echo "[Backend] Starting on http://localhost:3000..."
cargo run &
BACKEND_PID=$!

# 2. Start Frontend (Vite)
echo "[Frontend] Starting on http://localhost:5173..."
cd ui && npm run dev &
FRONTEND_PID=$!

# Wait for both
wait $BACKEND_PID $FRONTEND_PID
