#!/usr/bin/env bash

run_frontend() {
	(
		cd "$FRONTEND_DIR"
		npm run start
	)
}

run_tauri() {
	(
		cd "$APP_DIR"
		npm run tauri -- dev
	)
}

run_backend() {
  (
    cd "$BACKEND_DIR"
    cargo run
  )
}

watch_backend() {
  (
    cd "$BACKEND_DIR"
    cargo watch -w src -x "run RUST_BACKTRACE=1"
  )
}