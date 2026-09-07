function Open-InVSCode {
    if (-not (Get-Command code -ErrorAction SilentlyContinue)) {
        Write-Host 'Der VS-Code-Befehl "code" wurde nicht gefunden.'
        Write-Host 'Aktiviere in VS Code den Shell-Befehl "code" und starte das Skript erneut.'
        return
    }

    Write-Host ''
    Write-Host 'Was soll in VS Code geoeffnet werden?'
    Write-Host '  1) Gesamtes Verzeichnis'
    Write-Host '  2) app (Tauri)'
    Write-Host '  3) backend (Rust)'
    Write-Host '  4) frontend (Angular)'
    $choice = Read-Host 'Auswahl [1-4]'

    switch ($choice) {
        '1' { code $script:RootDir }
        '2' { code $script:AppDir }
        '3' { code $script:BackendDir }
        '4' { code $script:FrontendDir }
        default { Write-Host 'Ungueltige Auswahl. VS Code wird nicht geoeffnet.' }
    }
}

function Select-IDE {
    Write-Host 'Soll VS Code geoeffnet werden?'
    Write-Host '  1) Ja'
    Write-Host '  2) Nein'
    $choice = Read-Host 'Auswahl [1-2]'

    switch ($choice) {
        '1' { Open-InVSCode }
        '2' { }
        default { Write-Host 'Ungueltige Auswahl. Es wird ohne IDE fortgesetzt.' }
    }
}

function Select-Action {
    while ($true) {
        Write-Host ''
        Write-Host 'Was soll ausgefuehrt werden?'
        Write-Host '  1) ng serve'
        Write-Host '  2) tauri dev'
        Write-Host '  3) Beenden'
        $choice = Read-Host 'Auswahl [1-3]'

        switch ($choice) {
            '1' { Invoke-Frontend; return }
            '2' { Invoke-Tauri; return }
            '3' { return }
            default { Write-Host 'Ungueltige Auswahl. Bitte 1 bis 3 eingeben.' }
        }
    }
}
