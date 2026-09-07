# simply-note-app (In Entwicklung. Genauere angaben folgen. Es können Fehler aufträten!)

Eine Desktop-Notiz-App mit Angular als Frontend und Tauri als Desktop-Anwendung. Das Projekt enthält außerdem ein separates Rust-Backend-Verzeichnis.

## Plattform-Kompatibilität

Das Projekt ist grundsätzlich für macOS, Linux und Windows vorbereitet:

| Plattform | Startskript | Hinweis |
| --- | --- | --- |
| macOS | `./worspace.sh` | Bash ist normalerweise vorhanden. |
| Linux | `./worspace.sh` | Bash und die Tauri-Systemabhängigkeiten müssen installiert sein. |
| Windows | `worspace.cmd` | Startet die PowerShell-Module automatisch. |

Die Projektdateien und Startskripte sind damit plattformübergreifend angelegt. Für einen erfolgreichen Tauri-Build müssen zusätzlich die jeweiligen Systemvoraussetzungen des Betriebssystems erfüllt sein.

## Voraussetzungen

Vor dem ersten Start müssen folgende Werkzeuge installiert sein:

| Werkzeug | Zweck | Download |
| --- | --- | --- |
| Node.js inklusive npm | Angular, npm-Pakete und Tauri-CLI | [Node.js herunterladen](https://nodejs.org/en/download) |
| Rust inklusive Cargo und rustc | Tauri-Anwendung und Rust-Code kompilieren | [Rust installieren](https://www.rust-lang.org/tools/install) |

Das Startskript prüft `node`, `npm`, `cargo` und `rustc` automatisch. Fehlen Werkzeuge, wird der Start mit einer entsprechenden Meldung beendet.

### Optionale Werkzeuge

| Werkzeug | Zweck | Download |
| --- | --- | --- |
| Visual Studio Code | Projekt oder einzelne Ordner öffnen | [VS Code herunterladen](https://code.visualstudio.com/download) |
| PowerShell | Windows-Skript ausführen; unter aktuellen Windows-Versionen meist vorhanden | [PowerShell installieren](https://learn.microsoft.com/powershell/scripting/install/installing-powershell) |

Für Tauri können je nach Betriebssystem weitere Systemkomponenten erforderlich sein:

- [Tauri-Voraussetzungen für Windows, macOS und Linux](https://v2.tauri.app/start/prerequisites/)
- Windows: [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) und [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)
- macOS: [Xcode und Command Line Tools](https://developer.apple.com/xcode/)

## Installation

1. Repository herunterladen oder klonen.
2. In das Projektverzeichnis wechseln.
3. Das Startskript ausführen.

Die npm-Pakete werden beim ersten Start automatisch im Ordner `frontend` installiert. Eine manuelle Installation ist ebenfalls möglich:

```bash
cd frontend
npm install
```

## Starten

### macOS und Linux

```bash
./worspace.sh
```

Falls die Datei noch nicht ausführbar ist:

```bash
chmod +x worspace.sh
./worspace.sh
```

### Windows

Doppelklick auf:

```text
worspace.cmd
```

Oder in `cmd` beziehungsweise PowerShell aus dem Projektverzeichnis:

```powershell
.\worspace.cmd
```

Der Windows-Starter ruft das PowerShell-Modul `@shell/windows/start.ps1` auf. Die Ausführung erfolgt für diesen Aufruf mit einer temporären `ExecutionPolicy`, damit keine separate Richtlinienanpassung notwendig ist.

## Menü

Nach der Werkzeug- und Paketprüfung bietet das Skript an:

1. Visual Studio Code zu öffnen oder ohne IDE fortzufahren.
2. Das gesamte Projekt oder einen einzelnen Bereich in VS Code zu öffnen:
   - gesamtes Verzeichnis
   - `app` für Tauri
   - `backend` für Rust
   - `frontend` für Angular
3. Einen Entwicklungsbefehl auszufuehren:
   - `ng serve`
   - `tauri dev`
   - Beenden

`tauri dev` startet den Angular-Dev-Server über die vorhandene Tauri-Konfiguration automatisch. Deshalb sollte nicht gleichzeitig ein separates `ng serve` gestartet werden.

## Manuelle Befehle

Angular-Entwicklung:

```bash
cd frontend
npm run start
```

Angular-Build:

```bash
cd frontend
npm run build
```

Tauri-Entwicklung:

```bash
cd app
npm run tauri -- dev
```

Rust-Backend separat prüfen:

```bash
cd backend
cargo check
```

## Projektstruktur

```text
.
|-- @shell/
|   |-- init.sh
|   |-- commands.sh
|   |-- options.sh
|   `-- windows/
|       |-- init.ps1
|       |-- commands.ps1
|       |-- options.ps1
|       `-- start.ps1
|-- app/              Tauri-Anwendung
|-- backend/          separates Rust-Backend
|-- frontend/         Angular-Anwendung
|-- worspace.sh       Startskript für macOS und Linux
|-- worspace.cmd      Startskript für Windows
`-- README.md
```

## Aktueller Projektstand

- Frontend: Angular 22
- Desktop-App: Tauri 2
- Frontend-Abhängigkeiten: `frontend/package.json` und `frontend/package-lock.json`  
- Rust-Code: Tauri-Code unter `app/src` sowie ein separates Backend unter `backend`
