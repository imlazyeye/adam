use crate::igor::BuildData;

pub fn build_macros(build_data: &BuildData) -> serde_json::Value {
    let macros_files = format!(
        r#"{{
    "Desktop": "{USER_DIR}/Desktop",
    "Programs": "",
    "MyDocuments": "{USER_DIR}",
    "Favorites": "{USER_DIR}/Library/Favorites",
    "Startup": "",
    "Recent": "",
    "SendTo": "",
    "StartMenu": "",
    "MyMusic": "{USER_DIR}/Music",
    "MyVideos": "{USER_DIR}/Videos",
    "DesktopDirectory": "{USER_DIR}/Desktop",
    "MyComputer": "",
    "NetworkShortcuts": "",
    "Fonts": "{USER_DIR}/Library/Fonts",
    "Templates": "{USER_DIR}/Templates",
    "CommonStartMenu": "",
    "CommonPrograms": "",
    "CommonStartup": "",
    "CommonDesktopDirectory": "",
    "ApplicationData": "{USER_DIR}/.config",
    "PrinterShortcuts": "",
    "LocalApplicationData": "{USER_DIR}/.local/share",
    "InternetCache": "{USER_DIR}/Library/Caches",
    "Cookies": "",
    "History": "",
    "CommonApplicationData": "/Users/Shared",
    "Windows": "",
    "System": "",
    "ProgramFiles": "/Applications",
    "MyPictures": "{USER_DIR}/Pictures",
    "UserProfile": "{USER_DIR}",
    "SystemX86": "",
    "ProgramFilesX86": "",
    "CommonProgramFiles": "",
    "CommonProgramFilesX86": "",
    "CommonTemplates": "/usr/share/templates",
    "CommonDocuments": "",
    "CommonAdminTools": "",
    "AdminTools": "",
    "CommonMusic": "",
    "CommonPictures": "",
    "CommonVideos": "",
    "Resources": "",
    "LocalizedResources": "",
    "CommonOemLinks": "",
    "CDBurning": "",
    "TempPath": "/var/folders/v_/r1l809l94_vbd75s98fbd6rr0000gn",
    "exe_path": "/Applications/GameMaker Studio 2.app/Contents/MonoBundle",
    "program_dir_name": "GameMakerStudio2",
    "program_name": "GameMakerStudio2",
    "program_name_pretty": "GameMaker Studio 2",
    "runtimeURI": "https://gms.yoyogames.com/Zeus-Runtime.rss",
    "updateURI": "https://gms.yoyogames.com/update-mac.rss",
    "releaseNotesURI": "https://gms.yoyogames.com/ReleaseNotes.html",
    "runtimeReleaseNotesURI": "https://gms.yoyogames.com/release-notes-runtime.html",
    "runtimeBaseLocation": "{GM_DATA}/Cache/runtimes",
    "runtimeLocation": "{RUNTIME}",
    "base_options_dir": "{RUNTIME}/BaseProject/options",
    "asset_compiler_path": "{RUNTIME}/bin/GMAssetCompiler.exe",
    "igor_path": "{RUNTIME}/bin/Igor.exe",
    "lib_compatibility_path": "{RUNTIME}/lib/compatibility.zip",
    "runner_path": "{RUNTIME}/windows/Runner.exe",
    "html5_runner_path": "{RUNTIME}/html5/scripts.html5.zip",
    "webserver_path": "{RUNTIME}/bin/GMWebServer.exe",
    "licenses_path": "/Applications/GameMaker Studio 2.app/Contents/MonoBundle/Licenses",
    "java_exe_path": "bin/java",
    "adb_exe_path": "platform-tools/adb",
    "keytool_exe_path": "bin/keytool",
    "openssl_exe_path": "bin/openssl",
    "skin_path": "/Applications/GameMaker Studio 2.app/Contents/MonoBundle/GUI/Skins",
    "user_skin_path": "{GM_DATA}/Skins",
    "user_override_directory": "{GM_DATA}/User",
    "default_skin": "/Applications/GameMaker Studio 2.app/Contents/MonoBundle/GUI/Skins/Dark",
    "current_skin": "{GM_DATA}/Skins/Dracula",
    "system_directory": "{GM_DATA}",
    "system_cache_directory": "{GM_DATA}/Cache",
    "local_directory": "{USER_DIR}/.config/GameMakerStudio2",
    "local_cache_directory": "{BUILD_DIR}",
    "temp_directory": "{BUILD_DIR}",
    "asset_compiler_cache_directory": "{BUILD_DIR}",
    "ide_cache_directory": "{USER_DIR}/.config/GameMakerStudio2/Cache/GMS2IDE",
    "my_projects_directory": "{USER_DIR}/GameMakerStudio2",
    "base_project": "{RUNTIME}/BaseProject/BaseProject.yyp",
    "default_font": "Open Sans",
    "default_style": "Regular",
    "default_font_size": "9",
    "loginURI": "https://api.yoyogames.com",
    "accountsURI": "https://accounts.yoyogames.com",
    "marketplaceURI": "https://marketplace.yoyogames.com",
    "marketplaceAPIURI": "https://api.yoyogames.com",
    "carouselSlidesURI": "https://api.yoyogames.com/api/2/slideshow.json",
    "user_directory": "{BUILD_DIR}",
    "user_cache_directory": "{BUILD_DIR}",
    "project_full_filename": "{PROJECT_DIR}/{PROJECT_NAME}.yyp",
    "project_dir": "{PROJECT_DIR}",
    "project_name": "{PROJECT_NAME}",
    "project_cache_directory_name": "cache",
    "options_dir": "{PROJECT_DIR}/options"
}}"#,
        USER_DIR = build_data.user_dir.display(),
        RUNTIME = build_data.runtime_location.display(),
        GM_DATA = "Users/Shared/GameMakerStudio2",
        PROJECT_DIR = build_data.current_directory.display(),
        PROJECT_NAME = build_data.project_name,
        BUILD_DIR = build_data
            .output_folder
            .join(build_data.output_kind.to_string())
            .display()
    );

    serde_json::from_str(&macros_files).unwrap()
}
