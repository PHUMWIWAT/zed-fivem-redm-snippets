use std::env;
use std::path::PathBuf;

use zed_extension_api::settings::LspSettings;
use zed_extension_api::{self as zed, serde_json::json, Result};

struct FiveMRedMExtension;

fn install_dir() -> PathBuf {
    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn game_folder(worktree: &zed::Worktree) -> &'static str {
    let game = LspSettings::for_worktree("fivem-redm", worktree)
        .ok()
        .and_then(|settings| settings.settings)
        .and_then(|settings| settings.get("game")?.as_str().map(str::to_owned))
        .unwrap_or_else(|| "gtav".to_string());

    if game.eq_ignore_ascii_case("rdr3") {
        "RDR3"
    } else {
        "GTAV"
    }
}

impl zed::Extension for FiveMRedMExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_additional_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        target_language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        if target_language_server_id.as_ref() != "lua-language-server" {
            return Ok(None);
        }

        let root = install_dir().join("plugin");
        let library = root.join("library");
        let game = game_folder(worktree);

        Ok(Some(json!({
            "Lua": {
                "runtime": {
                    "version": "Lua 5.4",
                    "plugin": root.join("plugin.lua").to_string_lossy(),
                    "nonstandardSymbol": [
                        "/**/", "`", "+=", "-=", "*=", "/=", "<<=", ">>=", "&=", "|=", "^="
                    ]
                },
                "workspace": {
                    "library": [
                        library.join("runtime").to_string_lossy(),
                        library.join("natives").join("CFX-NATIVE").to_string_lossy(),
                        library.join("natives").join(game).to_string_lossy()
                    ],
                    "ignoreDir": [".vscode", ".git", ".github", "node_modules", "\\[cfx\\]"]
                }
            }
        })))
    }
}

zed::register_extension!(FiveMRedMExtension);
