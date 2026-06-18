# Wolfram Language for Zed

Wolfram Language support for [Zed](https://zed.dev/), including Tree-sitter
syntax highlighting, Wolfram-themed editor themes, bracket pairing, comment
configuration, and Language Server Protocol integration through the Wolfram
`LSPServer` paclet.

## Features

- Syntax highlighting for Wolfram Language source files using
  [`zed-wolfram-treesitter`](https://github.com/mtirard/zed-wolfram-treesitter).
- File recognition for `.wl`, `.wlt`, `.m`, `.mt`, `.wls`, `.nb`, `.cdf`, and
  `.tr` files.
- Wolfram-style bracket pairs for lists, function calls, associations, strings,
  and comments.
- Wolfram LSP startup from `WolframKernel`, `MathKernel`, or `wolframscript`.
- Semantic token support from Wolfram LSP, enabled by default by the extension.
- Bundled themes:
  - `Wolfram Dark`
  - `Wolfram Light`
  - `Wolfram Dark Rainbow`
  - `Wolfram Default`

## Requirements

Syntax highlighting works as soon as the extension is installed.

Language server features require a local Wolfram installation that can run one of
these executables:

- `WolframKernel`
- `MathKernel`
- `wolframscript`

The Wolfram kernel must be able to load `LSPServer`:

```wl
Needs["LSPServer`"]
```

`LSPServer` and its dependencies are included with Wolfram System 13.0 and
newer. For older or custom installations, install the paclets from a Wolfram
Language session:

```wl
PacletInstall["CodeParser"]
PacletInstall["CodeInspector"]
PacletInstall["CodeFormatter"]
PacletInstall["LSPServer"]
```

See [WolframResearch/LSPServer](https://github.com/WolframResearch/LSPServer)
for upstream server details.

## Installation

After the extension is published, install it from Zed's Extensions view:

1. Open Extensions with `cmd-shift-x` on macOS or `ctrl-shift-x` on Linux and
   Windows.
2. Search for `Wolfram Language`.
3. Click Install.

For local development or manual testing:

1. Install Rust with [`rustup`](https://rustup.rs/).
2. In Zed, run `zed: install dev extension` from the command palette.
3. Select this repository directory, which contains `extension.toml`.

Zed documents the local extension workflow in
[Developing Extensions](https://zed.dev/docs/extensions/developing-extensions).

## Configuration

The extension registers the language server as `wolfram-lsp`. If a Wolfram
kernel executable is on `PATH`, no configuration is normally required.

Use `lsp.wolfram-lsp.binary.path` when the executable is not discoverable or
when you want to pin a specific Wolfram installation:

```json
{
  "lsp": {
    "wolfram-lsp": {
      "binary": {
        "path": "/Applications/Mathematica.app/Contents/MacOS/WolframKernel"
      }
    }
  }
}
```

On Linux, a Wolfram Engine install commonly looks like this:

```json
{
  "lsp": {
    "wolfram-lsp": {
      "binary": {
        "path": "/usr/local/Wolfram/WolframEngine/14.0/Executables/WolframKernel"
      }
    }
  }
}
```

On Windows, use the executable path for your installed version:

```json
{
  "lsp": {
    "wolfram-lsp": {
      "binary": {
        "path": "C:\\Program Files\\Wolfram Research\\Wolfram Engine\\14.0\\WolframKernel.exe"
      }
    }
  }
}
```

The extension supplies default arguments for kernel executables:

```text
-noinit -noprompt -nopaclet -noicon -nostartuppaclets -run "Needs[\"LSPServer`\"];LSPServer`StartServer[]"
```

For `wolframscript`, it supplies:

```text
-local -code "Needs[\"LSPServer`\"];LSPServer`StartServer[]"
```

Override the launch arguments only if you need custom `LSPServer` startup
behavior, such as server-side logging:

```json
{
  "lsp": {
    "wolfram-lsp": {
      "binary": {
        "path": "WolframKernel",
        "arguments": [
          "-noinit",
          "-noprompt",
          "-nopaclet",
          "-noicon",
          "-nostartuppaclets",
          "-run",
          "Needs[\"LSPServer`\"];LSPServer`StartServer[\"/tmp/wolfram-lsp-logs\"]"
        ]
      }
    }
  }
}
```

The extension also accepts extension-specific startup options inside
`initialization_options.zed_extension`. Prefer `binary` settings for new
configuration; this compatibility shape is useful when you want to keep all
extension-specific options together:

```json
{
  "lsp": {
    "wolfram-lsp": {
      "initialization_options": {
        "zed_extension": {
          "kernel_path": "WolframKernel",
          "semantic_tokens": true
        }
      }
    }
  }
}
```

`zed_extension` is removed before initialization options are sent to the Wolfram
language server. All other initialization options are passed through.

To control how Zed displays semantic tokens for Wolfram Language files, use
Zed's language-level semantic token setting:

```json
{
  "languages": {
    "Wolfram Language": {
      "semantic_tokens": "combined"
    }
  }
}
```

See Zed's
[Configuring Languages](https://zed.dev/docs/configuring-languages#semantic-tokens)
documentation for the available semantic token modes.

## Troubleshooting

If syntax highlighting works but LSP features do not:

- Confirm that `WolframKernel`, `MathKernel`, or `wolframscript` is available on
  `PATH`, or set `lsp.wolfram-lsp.binary.path`.
- Run `Needs["LSPServer`"]` in the same Wolfram installation used by Zed.
- Open Zed's log with `zed: open log` and search for
  `wolfram-language-zed`.
- Start Zed from a terminal with `zed --foreground` to see extension startup
  diagnostics.
- If the kernel starts but exits, run Wolfram's LSP diagnostic helper with the
  same command shown in the Zed log:

```wl
Needs["LSPServer`"]
LSPServer`RunServerDiagnostic[{
  "/path/to/WolframKernel",
  "-noinit",
  "-noprompt",
  "-nopaclet",
  "-noicon",
  "-nostartuppaclets",
  "-run",
  "Needs[\"LSPServer`\"];LSPServer`StartServer[]"
}]
```

## Development

Build and test the Rust extension code with Cargo:

```sh
cargo test
```

Install the extension locally in Zed with `zed: install dev extension`. Zed will
compile the Rust code to WebAssembly and load the local `extension.toml`.

The Tree-sitter grammar is not implemented or vendored by this extension. Zed
fetches the upstream grammar pinned in `extension.toml`:

```toml
[grammars.wolfram]
repository = "https://github.com/mtirard/zed-wolfram-treesitter.git"
rev = "e4ebb3fde50391550b2e234bc0333447ca2e48c3"
```

## Release Notes

Current extension manifest version: `0.4.0`.

Before publishing a release:

- Confirm `extension.toml` has the intended `version`.
- Run `cargo test`.
- Install the repo as a dev extension in Zed and open `test.wl`.
- Verify syntax highlighting, theme selection, LSP startup, diagnostics, hover,
  and semantic token styling.
- Ensure the repository has a release-compatible root license before submitting
  to the Zed extensions registry.

Publishing follows Zed's extension registry process: open a pull request against
[`zed-industries/extensions`](https://github.com/zed-industries/extensions),
add this repository as a submodule under the chosen extension ID, and add the
matching `extensions.toml` entry. If this is the first public registry
submission, confirm that the final extension ID satisfies Zed's current registry
requirements before publishing.
