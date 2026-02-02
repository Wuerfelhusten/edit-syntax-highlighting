# ![Application Icon for Edit](./assets/edit.svg) Edit - Extended Syntax Highlighting Fork

A simple terminal-based text editor with extensive syntax highlighting support.

This is a fork of Microsoft's [Edit](https://github.com/microsoft/edit) that adds modern syntax highlighting for 19 programming languages. The editor pays homage to the classic [MS-DOS Editor](https://en.wikipedia.org/wiki/MS-DOS_Editor), but with a modern interface and input controls similar to VS Code

This is a mirrored version of my fork present on [Modding Forge - Code](https://code.moddingforge.com/Wuerfelhusten/edit-syntax-highlighting/). If you want the binaries you can download them **[here](https://code.moddingforge.com/Wuerfelhusten/edit-syntax-highlighting/-/releases/v1.3.1)**

![Screenshot of Edit with the About dialog in the foreground](./assets/edit_hero_image.png)

## What's New in This Fork

This fork extends the original Edit with comprehensive syntax highlighting support for additional programming languages and markup formats.

### Supported Languages (19 Total)

| Language             | File Extensions                                     | Features                                                 |
| -------------------- | --------------------------------------------------- | -------------------------------------------------------- |
| **JSON/JSONC** | `.json`, `.jsonc`                               | Object/array literals, strings, numbers, booleans        |
| **Rust**       | `.rs`                                             | Keywords, macros, attributes, lifetimes, raw strings     |
| **Python**     | `.py`, `.pyw`, `.pyi`                         | Keywords, decorators, f-strings, triple-quoted strings   |
| **JavaScript** | `.js`, `.mjs`, `.cjs`                         | ES6+ features, template literals, regex                  |
| **TypeScript** | `.ts`, `.mts`, `.cts`                         | TypeScript-specific syntax, generics                     |
| **Markdown**   | `.md`, `.markdown`                              | Headers, lists, code blocks, links, emphasis             |
| **TOML**       | `.toml`                                           | Tables, arrays, key-value pairs                          |
| **YAML**       | `.yaml`, `.yml`                                 | Keys, values, sequences, mappings                        |
| **C**          | `.c`, `.h`                                      | Preprocessor directives, C23 features, standard types    |
| **C++**        | `.cpp`, `.cc`, `.cxx`, `.hpp`, `.hxx`     | C++20 features, raw strings, templates, STL              |
| **C#**         | `.cs`                                             | C# 9.0+ features, LINQ, interpolated strings, attributes |
| **Go**         | `.go`                                             | Goroutines, channels, defer, built-in functions          |
| **HTML**       | `.html`, `.htm`                                 | Tags, attributes, DOCTYPE, comments                      |
| **CSS**        | `.css`                                            | Selectors, at-rules, properties, colors                  |
| **Java**       | `.java`                                           | Java 17 features, annotations, Javadoc, text blocks      |
| **XML**        | `.xml`, `.svg`, `.xhtml`, `.xsd`, `.wsdl` | Tags, CDATA, processing instructions, entities           |
| **Shell**      | `.sh`, `.bash`, `.zsh`                        | Variables, builtins, redirections, command substitution  |
| **SQL**        | `.sql`                                            | DDL/DML/DCL keywords, data types, functions              |
| **AsciiDoc**   | `.adoc`, `.asciidoc`, `.asc`                  | Headers, lists, macros, attributes, formatting           |

### Lexer Architecture

All lexers implement high-performance byte-level parsing with:

- **Zero-copy tokenization** - Direct byte slice processing
- **Context-aware highlighting** - Language-specific features
- **Multi-line support** - Block comments, strings, heredocs
- **Escape sequences** - Proper handling of string escapes

## Installation

You can download binaries from the [Releases page](../../releases/latest) or build from source.

### Build from Source

Requirements:

- [Rust](https://www.rust-lang.org/tools/install) (nightly toolchain recommended)
- Git

```bash
# Clone the repository
git clone <repository-url>
cd edit

# Build release version
cargo build --release --package edit

# Binary will be in target/release/edit
```

### Build Configuration

| Environment variable   | Description                                                                     |
| ---------------------- | ------------------------------------------------------------------------------- |
| `EDIT_CFG_ICU*`      | See[ICU library name (SONAME)](#icu-library-name-soname) for details.              |
| `EDIT_CFG_LANGUAGES` | Comma-separated list of languages to include. See[i18n/edit.toml](i18n/edit.toml). |

## Usage

```bash
# Open a file
edit filename.rs

# Create new file
edit newfile.py
```

**Keyboard Shortcuts:**

- Standard shortcuts similar to VS Code and modern editors
- Full terminal compatibility with support for mouse input
- Context menus and dialog boxes for common operations

## Original Project

This fork is based on [Microsoft Edit](https://github.com/microsoft/edit) v1.2.1.

Original features include:

- Simple, accessible terminal-based editor
- Modern UI inspired by MS-DOS Editor
- Cross-platform support (Windows, Linux, macOS)
- Optional ICU integration for advanced text operations

## ICU Library Name (SONAME)

This project _optionally_ depends on the ICU library for Search and Replace functionality.
By default, the project looks for a SONAME without version suffix:

* Windows: `icuuc.dll`
* macOS: `libicuuc.dylib`
* UNIX: `libicuuc.so`

If your installation uses a different SONAME, set these environment variables at build time:

* `EDIT_CFG_ICUUC_SONAME`: e.g., `libicuuc.so.76`
* `EDIT_CFG_ICUI18N_SONAME`: e.g., `libicui18n.so.76`

For versioned exports:

* `EDIT_CFG_ICU_CPP_EXPORTS`: Set to `true` for C++ symbols (default on macOS)
* `EDIT_CFG_ICU_RENAMING_VERSION`: Version number for suffixed symbols
* `EDIT_CFG_ICU_RENAMING_AUTO_DETECT`: Auto-detect version at runtime (UNIX only)

Test your ICU configuration:

```sh
cargo test -- --ignored
```

## Contributing

Contributions are welcome! Areas for improvement:

- Additional language lexers
- Performance optimizations
- Bug fixes and testing
- Documentation improvements

## License

MIT License - see [LICENSE](LICENSE) file for details.

Original project © Microsoft Corporation
