function Initialize-Paths {
    param([Parameter(Mandatory = $true)][string]$RootDir)

    $script:RootDir = $RootDir
    $script:FrontendDir = Join-Path $RootDir 'frontend'
    $script:AppDir = Join-Path $RootDir 'app'
    $script:BackendDir = Join-Path $RootDir 'backend'
}

function Test-RequiredTools {
    $missingTools = @('node', 'npm', 'cargo', 'rustc') | Where-Object {
        -not (Get-Command $_ -ErrorAction SilentlyContinue)
    }

    if ($missingTools.Count -gt 0) {
        Write-Host "Folgende benoetigte Werkzeuge fehlen: $($missingTools -join ', ')"
        Write-Host 'Installiere Node.js/npm sowie Rust/Cargo und starte das Skript erneut.'
        return $false
    }

    return $true
}

function Install-NpmPackages {
    $ngBinary = Join-Path $script:FrontendDir 'node_modules\.bin\ng.cmd'
    $tauriBinary = Join-Path $script:FrontendDir 'node_modules\.bin\tauri.cmd'

    if ((Test-Path $ngBinary) -and (Test-Path $tauriBinary)) {
        Write-Host 'npm-Pakete sind bereits installiert.'
        return
    }

    Write-Host 'Installiere npm-Pakete im frontend-Verzeichnis ...'
    Push-Location $script:FrontendDir
    try {
        npm install
        if ($LASTEXITCODE -ne 0) {
            throw 'npm install ist fehlgeschlagen.'
        }
    }
    finally {
        Pop-Location
    }
}

function Start-Workspace {
    param([Parameter(Mandatory = $true)][string]$RootDir)

    Initialize-Paths $RootDir
    . (Join-Path $PSScriptRoot 'commands.ps1')
    . (Join-Path $PSScriptRoot 'options.ps1')

    Write-Host ''
    Write-Host '== simply-note-app =='
    Write-Host ''

    if (-not (Test-RequiredTools)) {
        exit 1
    }

    Install-NpmPackages
    Select-IDE
    Select-Action
}
