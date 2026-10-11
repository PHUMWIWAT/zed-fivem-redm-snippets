use std::env;
use std::path::PathBuf;

use zed_extension_api::settings::LspSettings;
use zed_extension_api::{self as zed, Result};

struct FiveMRedMExtension;

fn game_folder(game: Option<&str>) -> &'static str {
    match game {
        Some(game) if game.eq_ignore_ascii_case("rdr3") => "RDR3",
        _ => "GTAV",
    }
}

fn slash_path(folder: PathBuf) -> String {
    folder.to_string_lossy().replace('\\', "/")
}

/// Zed's wasm process cwd is `extensions/work/<id>`, which only holds the build.
/// The vendored `plugin/` directory is in `extensions/installed/<id>`.
fn extension_dir_from_work(work: &std::path::Path) -> PathBuf {
    let installed = work.parent().and_then(|work_root| {
        if work_root.file_name().and_then(|name| name.to_str()) == Some("work") {
            let id = work.file_name()?;
            Some(work_root.parent()?.join("installed").join(id))
        } else {
            None
        }
    });
    installed.unwrap_or_else(|| work.to_path_buf())
}

fn lua_library(game: &str) -> Result<zed::serde_json::Value> {
    let plugin = extension_dir_from_work(&env::current_dir().map_err(|err| err.to_string())?)
        .join("plugin");
    let natives = plugin.join("library").join("natives");

    Ok(zed::serde_json::json!({
        "Lua": {
            "runtime": {
                "version": "Lua 5.4",
                "plugin": slash_path(plugin.join("plugin.lua")),
                "nonstandardSymbol": [
                    "/**/", "`", "+=", "-=", "*=", "/=", "<<=", ">>=", "&=", "|=", "^="
                ]
            },
            "workspace": {
                "library": [
                    slash_path(plugin.join("library").join("runtime")),
                    slash_path(natives.join("CFX-NATIVE")),
                    slash_path(natives.join(game))
                ],
                "ignoreDir": [".vscode", ".git", ".github", "node_modules", "\\[cfx\\]"]
            }
        }
    }))
}

impl zed::Extension for FiveMRedMExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // Zed starts a server listed for Lua even when it is opt-in. This id is
        // not attached to Lua, but refuse a launch if it is started anyway.
        Err("FiveM RedM only adds natives to lua-language-server".into())
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

        let game = LspSettings::for_worktree("fivem-redm", worktree)
            .ok()
            .and_then(|settings| settings.settings)
            .and_then(|settings| {
                settings
                    .get("game")
                    .and_then(|value| value.as_str())
                    .map(str::to_owned)
            });

        Ok(Some(lua_library(game_folder(game.as_deref()))?))
    }
}

zed::register_extension!(FiveMRedMExtension);

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{extension_dir_from_work, game_folder};

    #[test]
    fn game_defaults_to_gtav_and_accepts_rdr3() {
        assert_eq!(game_folder(None), "GTAV");
        assert_eq!(game_folder(Some("gtav")), "GTAV");
        assert_eq!(game_folder(Some("RDR3")), "RDR3");
        assert_eq!(game_folder(Some("nope")), "GTAV");
    }

    #[test]
    fn library_path_uses_the_installed_extension_dir() {
        let work = Path::new(r"C:\Users\pummi\AppData\Local\Zed\extensions\work\fivem-redm-native-snippets");
        let installed = extension_dir_from_work(work);
        assert_eq!(
            installed,
            Path::new(r"C:\Users\pummi\AppData\Local\Zed\extensions\installed\fivem-redm-native-snippets")
        );
    }
}
