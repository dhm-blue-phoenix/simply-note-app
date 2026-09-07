function Invoke-Frontend {
    Push-Location $script:FrontendDir
    try {
        npm run start
    }
    finally {
        Pop-Location
    }
}

function Invoke-Tauri {
    Push-Location $script:AppDir
    try {
        npm run tauri -- dev
    }
    finally {
        Pop-Location
    }
}
