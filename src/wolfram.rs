use zed::serde_json::{Map, Value};
use zed::settings::{BinarySettings, LspSettings};
use zed::{Command, LanguageServerId, Worktree};
use zed_extension_api as zed;

const LANGUAGE_SERVER_NAME: &str = "wolfram-lsp";
const START_SERVER_CODE: &str = "Needs[\"LSPServer`\"];LSPServer`StartServer[]";
const KERNEL_CANDIDATES: [&str; 6] = [
    "WolframKernel",
    "WolframKernel.exe",
    "MathKernel",
    "MathKernel.exe",
    "wolframscript",
    "wolframscript.exe",
];
const SAFE_ENV_VARS: [&str; 11] = [
    "HOME",
    "PATH",
    "USER",
    "LANG",
    "LANGUAGE",
    "LC_ALL",
    "LC_CTYPE",
    "DISPLAY",
    "WAYLAND_DISPLAY",
    "XDG_RUNTIME_DIR",
    "DBUS_SESSION_BUS_ADDRESS",
];
const KERNEL_ARGS: [&str; 7] = [
    "-noinit",
    "-noprompt",
    "-nopaclet",
    "-noicon",
    "-nostartuppaclets",
    "-run",
    START_SERVER_CODE,
];
const WOLFRAMSCRIPT_ARGS: [&str; 3] = ["-local", "-code", START_SERVER_CODE];

struct WLExtension;

#[derive(Debug, Default, PartialEq, Eq)]
struct ExtensionLaunchSettings {
    kernel_path: Option<String>,
    arguments: Option<Vec<String>>,
    semantic_tokens: Option<bool>,
}

struct WLBinary {
    path: String,
    args: Vec<String>,
}

impl WLExtension {
    fn lsp_settings(worktree: &Worktree) -> Option<LspSettings> {
        LspSettings::for_worktree(LANGUAGE_SERVER_NAME, worktree).ok()
    }

    fn raw_initialization_options(worktree: &Worktree) -> Option<Value> {
        Self::lsp_settings(worktree).and_then(|lsp_settings| lsp_settings.initialization_options)
    }

    fn binary_settings(worktree: &Worktree) -> Option<BinarySettings> {
        Self::lsp_settings(worktree).and_then(|lsp_settings| lsp_settings.binary)
    }

    fn default_args(path: &str) -> Vec<String> {
        let executable_name = path.rsplit(['/', '\\']).next().unwrap_or(path);
        let args = if executable_name.eq_ignore_ascii_case("wolframscript")
            || executable_name.eq_ignore_ascii_case("wolframscript.exe")
        {
            WOLFRAMSCRIPT_ARGS.as_slice()
        } else {
            KERNEL_ARGS.as_slice()
        };

        args.iter().map(|arg| (*arg).to_string()).collect()
    }

    fn resolve_configured_path(path: String, worktree: &Worktree) -> String {
        if path.contains('/') || path.contains('\\') {
            return path;
        }

        worktree.which(&path).unwrap_or(path)
    }

    fn discovered_binary_path(worktree: &Worktree) -> Option<String> {
        KERNEL_CANDIDATES
            .iter()
            .find_map(|candidate| worktree.which(candidate))
    }

    fn resolved_args(path: &str, configured_args: Option<Vec<String>>) -> Vec<String> {
        configured_args.unwrap_or_else(|| Self::default_args(path))
    }

    fn extension_launch_settings_from_value(value: &Value) -> Option<ExtensionLaunchSettings> {
        let extension_settings = value.as_object()?.get("zed_extension")?.as_object()?;

        let kernel_path = extension_settings
            .get("kernel_path")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned);
        let arguments = extension_settings
            .get("arguments")
            .and_then(Value::as_array)
            .and_then(|arguments| {
                arguments
                    .iter()
                    .map(|argument| argument.as_str().map(ToOwned::to_owned))
                    .collect::<Option<Vec<_>>>()
            })
            .filter(|arguments| !arguments.is_empty());
        let semantic_tokens = extension_settings
            .get("semantic_tokens")
            .and_then(Value::as_bool);

        if kernel_path.is_none() && arguments.is_none() && semantic_tokens.is_none() {
            None
        } else {
            Some(ExtensionLaunchSettings {
                kernel_path,
                arguments,
                semantic_tokens,
            })
        }
    }

    fn extension_launch_settings(worktree: &Worktree) -> Option<ExtensionLaunchSettings> {
        let options = Self::raw_initialization_options(worktree)?;
        Self::extension_launch_settings_from_value(&options)
    }

    fn language_server_binary(&self, worktree: &Worktree) -> zed::Result<WLBinary> {
        let binary_settings = Self::binary_settings(worktree);
        let launch_settings = Self::extension_launch_settings(worktree).unwrap_or_default();
        let configured_path = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.path.clone())
            .map(|path| Self::resolve_configured_path(path, worktree))
            .or_else(|| {
                launch_settings
                    .kernel_path
                    .as_ref()
                    .map(|path| Self::resolve_configured_path(path.clone(), worktree))
            });
        let configured_args = binary_settings
            .and_then(|binary_settings| binary_settings.arguments)
            .filter(|arguments| !arguments.is_empty())
            .or(launch_settings.arguments);

        let path = configured_path
            .or_else(|| Self::discovered_binary_path(worktree))
            .ok_or_else(|| {
                "Unable to find a Wolfram kernel. Set `lsp.wolfram-lsp.binary.path` to a \
                 `WolframKernel`, `MathKernel`, or `wolframscript` executable."
                    .to_string()
            })?;

        let args = Self::resolved_args(&path, configured_args);

        Ok(WLBinary { path, args })
    }

    fn filtered_env(worktree: &Worktree) -> Vec<(String, String)> {
        worktree
            .shell_env()
            .into_iter()
            .filter(|(key, _)| SAFE_ENV_VARS.contains(&key.as_str()))
            .collect()
    }

    fn merged_initialization_options(worktree: &Worktree) -> Option<Value> {
        Self::merged_initialization_options_from_value(Self::raw_initialization_options(worktree))
    }

    fn merged_initialization_options_from_value(raw_options: Option<Value>) -> Option<Value> {
        let semantic_tokens = raw_options
            .as_ref()
            .and_then(Self::extension_launch_settings_from_value)
            .and_then(|settings| settings.semantic_tokens);
        let mut options = match raw_options {
            Some(Value::Object(mut options)) => {
                options.remove("zed_extension");
                options
            }
            Some(_) | None => Map::new(),
        };

        if let Some(semantic_tokens) = semantic_tokens {
            options.insert("semanticTokens".to_string(), Value::Bool(semantic_tokens));
        } else {
            options
                .entry("semanticTokens".to_string())
                .or_insert(Value::Bool(true));
        }

        if options.is_empty() {
            None
        } else {
            Some(Value::Object(options))
        }
    }
}

impl zed::Extension for WLExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Command> {
        let lsp_binary = self.language_server_binary(worktree)?;
        let env = Self::filtered_env(worktree);

        eprintln!(
            "wolfram-language-zed: starting Wolfram LSP with command={} args={:?}",
            lsp_binary.path, lsp_binary.args
        );

        Ok(Command {
            command: lsp_binary.path,
            args: lsp_binary.args,
            env,
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Option<Value>> {
        eprintln!("wolfram-language-zed: preparing Wolfram LSP initialization options");
        Ok(Self::merged_initialization_options(worktree))
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Option<Value>> {
        Ok(Self::lsp_settings(worktree).and_then(|lsp_settings| lsp_settings.settings))
    }
}

zed::register_extension!(WLExtension);

#[cfg(test)]
mod tests {
    use super::{ExtensionLaunchSettings, WLExtension};
    use zed_extension_api::serde_json::json;

    #[test]
    fn parses_extension_launch_settings() {
        let value = json!({
            "zed_extension": {
                "kernel_path": "/tmp/WolframKernel",
                "arguments": ["-run", "Needs[\"LSPServer`\"];LSPServer`StartServer[]"]
            },
            "semanticTokens": true
        });

        let launch_settings = WLExtension::extension_launch_settings_from_value(&value);

        assert_eq!(
            launch_settings,
            Some(ExtensionLaunchSettings {
                kernel_path: Some("/tmp/WolframKernel".to_string()),
                arguments: Some(vec![
                    "-run".to_string(),
                    "Needs[\"LSPServer`\"];LSPServer`StartServer[]".to_string()
                ]),
                semantic_tokens: None,
            })
        );
    }

    #[test]
    fn ignores_empty_extension_arguments() {
        let value = json!({
            "zed_extension": {
                "kernel_path": "/tmp/WolframKernel",
                "arguments": []
            }
        });

        let launch_settings = WLExtension::extension_launch_settings_from_value(&value);

        assert_eq!(
            launch_settings,
            Some(ExtensionLaunchSettings {
                kernel_path: Some("/tmp/WolframKernel".to_string()),
                arguments: None,
                semantic_tokens: None,
            })
        );
    }

    #[test]
    fn parses_semantic_tokens_opt_in() {
        let value = json!({
            "zed_extension": {
                "semantic_tokens": true
            }
        });

        let launch_settings = WLExtension::extension_launch_settings_from_value(&value);

        assert_eq!(
            launch_settings,
            Some(ExtensionLaunchSettings {
                kernel_path: None,
                arguments: None,
                semantic_tokens: Some(true),
            })
        );
    }

    #[test]
    fn enables_semantic_tokens_by_default() {
        let merged = WLExtension::merged_initialization_options_from_value(None);

        assert_eq!(merged, Some(json!({ "semanticTokens": true })));
    }

    #[test]
    fn preserves_top_level_semantic_tokens_setting() {
        let merged = WLExtension::merged_initialization_options_from_value(Some(json!({
            "semanticTokens": false
        })));

        assert_eq!(merged, Some(json!({ "semanticTokens": false })));
    }

    #[test]
    fn zed_extension_semantic_tokens_setting_overrides_top_level_setting() {
        let merged = WLExtension::merged_initialization_options_from_value(Some(json!({
            "semanticTokens": true,
            "zed_extension": {
                "semantic_tokens": false
            }
        })));

        assert_eq!(merged, Some(json!({ "semanticTokens": false })));
    }

    #[test]
    fn uses_default_kernel_args_when_arguments_are_missing() {
        let args = WLExtension::resolved_args("/tmp/WolframKernel", None);

        assert_eq!(args, WLExtension::default_args("/tmp/WolframKernel"));
    }

    #[test]
    fn uses_wolframscript_args_for_windows_executable() {
        let args =
            WLExtension::default_args(r"C:\\Program Files\\Wolfram Research\\wolframscript.exe");

        assert_eq!(args, super::WOLFRAMSCRIPT_ARGS);
    }

    #[test]
    fn uses_configured_args_when_present() {
        let args = WLExtension::resolved_args(
            "/tmp/WolframKernel",
            Some(vec!["-custom".to_string(), "-flag".to_string()]),
        );

        assert_eq!(args, vec!["-custom".to_string(), "-flag".to_string()]);
    }
}
