# Zed Wolfram Language Extension
This extension adds Wolfram Language support to the Zed IDE.
This includes syntax highlighting from [LumaKernel's tree-sitter parser] and integration with Wolfram's LSPServer.


# Setup
Default's are currently boken so you need to add the following to your zed settings:
```json
{
	"lsp": {
		"wolfram-lsp": {
			"binary": {
				"path": "/path/to/your/MathKernel",
				"arguments": [
					"-noinit",
					"-noprompt",
					"-nopaclet",
					"-noicon",
					"-nostartuppaclets",
					"-run",
					"Needs[\"LSPServer`\"];LSPServer`StartServer[]"
				]
			}
		}
	}
}
```

It is also highly recommended to disable format-on-save:
```json
{
	"languages": {
		"Wolfram Language": {
			"format_on_save": "off"
		}
	},
}
```
