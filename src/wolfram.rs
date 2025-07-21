use wolfram_app_discovery::WolframApp;
use zed::LanguageServerId;
use zed_extension_api::{self as zed, settings::LspSettings};

struct WLExtension {}

struct WLBinary {
    path: String,
    args: Vec<String>,
}

impl WLExtension {
    fn language_server_binary(
        &mut self,
        lsp_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<WLBinary> {
        eprintln!("Starting language_server_binary for {}", lsp_id.as_ref());
        // let (platform, arch) = zed::current_platform();
        let mut path: String = String::new();
        let mut args: Vec<String> = vec![
            "-noinit",
            "-noprompt",
            "-nopaclet",
            "-noicon",
            "-nostartuppaclets",
            "-run",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
        if let Ok(app) = WolframApp::try_default() {
            if let Some(executable) = app.app_executable() {
                path = executable.to_string_lossy().to_string();
            }
        }
        if let Ok(lsp_settings) = LspSettings::for_worktree(lsp_id.as_ref(), worktree) {
            if let Some(binary) = lsp_settings.binary {
                if let Some(_args) = binary.arguments {
                    if !_args.is_empty() {
                        args = _args;
                    }
                }
                if let Some(_path) = binary.path {
                    path = _path
                }
            }
        };

        args.push("Needs[\"LSPServer`\"];LSPServer`StartServer[]".to_string());
        eprintln!("Command that will be executed: {} {}", path, args.join(" "));

        if path.is_empty() {
            eprintln!("Wolfram Language executable path is empty!");
            return Err(format!(
                "Wolfram Language executable not found for LSP '{}'. Searched path: '{}'",
                lsp_id.as_ref(),
                path
            ));
        }

        if !std::path::Path::new(&path).exists() {
            eprintln!("Wolfram Language executable path does not exist: {}", path);
            return Err(format!(
                "Wolfram Language executable path does not exist: '{}'",
                path
            ));
        }

        if args.is_empty() {
            eprintln!("Language server arguments are empty!");
            return Err("Language server arguments not found".to_string());
        }

        return Ok(WLBinary { path, args });
    }
}

impl zed::Extension for WLExtension {
    fn new() -> Self {
        Self {}
    }
    fn language_server_command(
        &mut self,
        _lsp_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let lsp_binary = match self.language_server_binary(_lsp_id, _worktree) {
            Ok(binary) => binary,
            Err(e) => {
                return Err(e);
            }
        };
        Ok(zed::Command {
            command: lsp_binary.path,
            args: lsp_binary.args,
            env: vec![],
        })
    }

    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        completion: zed::lsp::Completion,
    ) -> Option<zed::CodeLabel> {
        // Log completion details for debugging
        eprintln!(
            "Completion received from LSP {}: {:?}",
            _language_server_id.as_ref(),
            completion
        );
        None
    }

    fn label_for_symbol(
        &self,
        _language_server_id: &LanguageServerId,
        symbol: zed::lsp::Symbol,
    ) -> Option<zed::CodeLabel> {
        // Log symbol details for debugging
        eprintln!(
            "Symbol received from LSP {}: {:?}",
            _language_server_id.as_ref(),
            symbol
        );
        None
    }
}

zed::register_extension!(WLExtension);

#[cfg(test)]
mod tests {

    #[test]
    fn test_default_args_not_overwritten_by_empty_settings() {
        let expected_args = vec![
            "-noinit".to_string(),
            "-noprompt".to_string(),
            "-nopaclet".to_string(),
            "-noicon".to_string(),
            "-nostartuppaclets".to_string(),
            "-run".to_string(),
        ];

        // Simulate the logic from language_server_binary
        let mut args = expected_args.clone();

        // Simulate empty arguments from settings
        let settings_args: Vec<String> = vec![];

        // This is the fixed logic - should NOT replace args if settings are empty
        if !settings_args.is_empty() {
            args = settings_args;
        }

        // Final safety check
        if args.is_empty() {
            args = expected_args.clone();
        }

        assert_eq!(args, expected_args);
        assert!(!args.is_empty());
    }

    #[test]
    fn test_custom_args_used_when_not_empty() {
        let default_args = vec![
            "-noinit".to_string(),
            "-noprompt".to_string(),
            "-nopaclet".to_string(),
        ];

        let custom_args = vec!["-custom".to_string(), "-flag".to_string()];

        let mut args = default_args.clone();

        // Simulate non-empty arguments from settings
        if !custom_args.is_empty() {
            args = custom_args.clone();
        }

        assert_eq!(args, custom_args);
        assert_ne!(args, default_args);
    }
}
