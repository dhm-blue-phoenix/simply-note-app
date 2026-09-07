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