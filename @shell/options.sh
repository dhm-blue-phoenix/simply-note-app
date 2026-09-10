#!/usr/bin/env bash

open_in_vscode() {
	if ! command_exists code; then
		printf 'Der VS-Code-Befehl "code" wurde nicht gefunden.\n'
		printf 'Aktiviere in VS Code den Shell-Befehl "code" und starte das Skript erneut.\n'
		return 1
	fi

	printf '\nWas soll in VS Code geöffnet werden?\n'
	printf '  1) Gesamtes Verzeichnis\n'
	printf '  2) app (Tauri)\n'
	printf '  3) backend (Rust)\n'
	printf '  4) frontend (Angular)\n'
	read -r -p 'Auswahl [1-4]: ' vscode_choice

	case "$vscode_choice" in
		1) code "$ROOT_DIR" ;;
		2) code "$APP_DIR" ;;
		3) code "$BACKEND_DIR" ;;
		4) code "$FRONTEND_DIR" ;;
		*) printf 'Ungültige Auswahl. VS Code wird nicht geöffnet.\n' ;;
	esac
}

select_ide() {
	printf 'Soll VS Code geöffnet werden?\n'
	printf '  1) Ja\n'
	printf '  2) Nein\n'
	read -r -p 'Auswahl [1-2]: ' ide_choice

	case "$ide_choice" in
		1) open_in_vscode || true ;;
		2) ;;
		*) printf 'Ungültige Auswahl. Es wird ohne IDE fortgesetzt.\n' ;;
	esac
}

select_action() {
	while true; do
		printf '\nWas soll ausgeführt werden?\n'
		printf '  1) ng serve\n'
		printf '  2) tauri dev\n'
		printf '  3) backend run\n'
		printf '  4) backend watch\n'
		printf '  5) Beenden\n'
		read -r -p 'Auswahl [1-5]: ' action_choice

		case "$action_choice" in
			1) run_frontend; break ;;
			2) run_tauri; break ;;
			3) run_backend; break ;;
			4) watch_backend; break ;;
			5) break ;;
			*) printf 'Ungültige Auswahl. Bitte 1 bis 5 eingeben.\n' ;;
		esac
	done
}
