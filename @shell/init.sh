#!/usr/bin/env bash

SHELL_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"

# Funktionsmodule: Menüs und Projektbefehle.
# shellcheck source=/dev/null
source "$SHELL_DIR/commands.sh"
# shellcheck source=/dev/null
source "$SHELL_DIR/options.sh"

init_paths() {
	ROOT_DIR="$1"
	FRONTEND_DIR="$ROOT_DIR/frontend"
	APP_DIR="$ROOT_DIR/app"
	BACKEND_DIR="$ROOT_DIR/backend"
}

print_header() {
	printf '\n== simply-note-app ==\n\n'
}

command_exists() {
	command -v "$1" >/dev/null 2>&1
}

check_required_tools() {
	local missing_tools=()

	for required_command in node npm cargo rustc; do
		if ! command_exists "$required_command"; then
			missing_tools+=("$required_command")
		fi
	done

	if ((${#missing_tools[@]} > 0)); then
		printf 'Folgende benötigte Werkzeuge fehlen: %s\n' "${missing_tools[*]}"
		printf 'Installiere Node.js/npm sowie Rust/Cargo und starte das Skript erneut.\n'
		return 1
	fi
}

install_npm_packages() {
	if [[ -x "$FRONTEND_DIR/node_modules/.bin/ng" && -x "$FRONTEND_DIR/node_modules/.bin/tauri" ]]; then
		printf 'npm-Pakete sind bereits installiert.\n'
		return
	fi

	printf 'Installiere npm-Pakete im frontend-Verzeichnis ...\n'
	(
		cd "$FRONTEND_DIR"
		npm install
	)
}

main() {
	init_paths "$1"
	print_header
	check_required_tools
	install_npm_packages
	select_ide
	select_action
}
