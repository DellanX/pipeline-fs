#!/bin/bash

# Script to run both UI and Rust service
echo "Starting Pipeline-FS services..."

# Start the Rust service in background
echo "Starting Rust service..."
cd /workspaces/pipeline-fs
cargo run -- --mount-point /tmp/test-mount --stages /tmp/stage0 /tmp/stage1 --port 8080 &
RUST_PID=$!

# Give the service a moment to start
sleep 2

# Start the UI
echo "Starting UI..."
cd /workspaces/pipeline-fs/ui
npm run dev & 
UI_PID=$!

echo "Services started!"
echo "Rust service PID: $RUST_PID"
echo "UI PID: $UI_PID"

# Wait for both processes
wait $RUST_PID $UI_PID