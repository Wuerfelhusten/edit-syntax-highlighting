// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Color themes for syntax highlighting.

use crate::oklab::StraightRgba;
use crate::syntax::TokenKind;

/// A complete color theme for syntax highlighting.
#[derive(Clone)]
pub struct Theme {
    styles: Vec<TokenStyle>,
}

/// The visual style for a token.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TokenStyle {
    /// Foreground color
    pub fg: StraightRgba,
    /// Background color (optional)
    pub bg: Option<StraightRgba>,
    /// Bold text
    pub bold: bool,
    /// Italic text
    pub italic: bool,
    /// Underline text
    pub underline: bool,
}

impl TokenStyle {
    /// Create a new style with just a foreground color.
    pub const fn new(fg: StraightRgba) -> Self {
        Self {
            fg,
            bg: None,
            bold: false,
            italic: false,
            underline: false,
        }
    }

    /// Set the bold flag.
    pub const fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Set the italic flag.
    pub const fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Set the underline flag.
    pub const fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    /// Set the background color.
    pub const fn bg(mut self, bg: StraightRgba) -> Self {
        self.bg = Some(bg);
        self
    }
}

impl Theme {
    /// Create a new theme with default dark colors (inspired by VS Code Dark+).
    pub fn default_dark() -> Self {
        let mut styles = vec![TokenStyle::new(rgb(0xD4D4D4)); 256];

        // Comments - green
        styles[TokenKind::Comment as usize] = TokenStyle::new(rgb(0x6A9955)).italic();

        // Strings - orange/brown
        styles[TokenKind::String as usize] = TokenStyle::new(rgb(0xCE9178));
        styles[TokenKind::Char as usize] = TokenStyle::new(rgb(0xCE9178));
        styles[TokenKind::Escape as usize] = TokenStyle::new(rgb(0xD7BA7D));

        // Numbers - light green
        styles[TokenKind::Number as usize] = TokenStyle::new(rgb(0xB5CEA8));

        // Booleans and null - blue
        styles[TokenKind::Boolean as usize] = TokenStyle::new(rgb(0x569CD6)).bold();
        styles[TokenKind::Null as usize] = TokenStyle::new(rgb(0x569CD6)).bold();

        // Keywords - purple/pink
        styles[TokenKind::Keyword as usize] = TokenStyle::new(rgb(0xC586C0));
        styles[TokenKind::KeywordControl as usize] = TokenStyle::new(rgb(0xC586C0)).bold();
        styles[TokenKind::KeywordFunction as usize] = TokenStyle::new(rgb(0xC586C0)).bold();
        styles[TokenKind::KeywordImport as usize] = TokenStyle::new(rgb(0xC586C0));
        styles[TokenKind::KeywordStorage as usize] = TokenStyle::new(rgb(0x569CD6));
        styles[TokenKind::KeywordType as usize] = TokenStyle::new(rgb(0x4EC9B0));
        styles[TokenKind::KeywordOperator as usize] = TokenStyle::new(rgb(0xC586C0));

        // Identifiers - light gray/white
        styles[TokenKind::Identifier as usize] = TokenStyle::new(rgb(0xD4D4D4));
        styles[TokenKind::TypeName as usize] = TokenStyle::new(rgb(0x4EC9B0));
        styles[TokenKind::FunctionName as usize] = TokenStyle::new(rgb(0xDCDCAA));
        styles[TokenKind::VariableName as usize] = TokenStyle::new(rgb(0x9CDCFE));
        styles[TokenKind::PropertyName as usize] = TokenStyle::new(rgb(0x9CDCFE));
        styles[TokenKind::ParameterName as usize] = TokenStyle::new(rgb(0x9CDCFE));

        // Operators and punctuation - light gray
        styles[TokenKind::Operator as usize] = TokenStyle::new(rgb(0xD4D4D4));
        styles[TokenKind::Punctuation as usize] = TokenStyle::new(rgb(0xD4D4D4));
        styles[TokenKind::Delimiter as usize] = TokenStyle::new(rgb(0xD4D4D4));
        styles[TokenKind::Separator as usize] = TokenStyle::new(rgb(0xD4D4D4));

        // Special
        styles[TokenKind::Attribute as usize] = TokenStyle::new(rgb(0x4EC9B0));
        styles[TokenKind::Macro as usize] = TokenStyle::new(rgb(0x4EC9B0));
        styles[TokenKind::Label as usize] = TokenStyle::new(rgb(0xDCDCAA));

        // JSON specific
        styles[TokenKind::JsonKey as usize] = TokenStyle::new(rgb(0x9CDCFE));
        styles[TokenKind::JsonBrace as usize] = TokenStyle::new(rgb(0xD4D4D4));
        styles[TokenKind::JsonBracket as usize] = TokenStyle::new(rgb(0xD4D4D4));
        styles[TokenKind::JsonColon as usize] = TokenStyle::new(rgb(0xD4D4D4));
        styles[TokenKind::JsonComma as usize] = TokenStyle::new(rgb(0xD4D4D4));

        // Rust specific
        styles[TokenKind::RustLifetime as usize] = TokenStyle::new(rgb(0x4EC9B0)).italic();
        styles[TokenKind::RustMacro as usize] = TokenStyle::new(rgb(0x4EC9B0));
        styles[TokenKind::RustAttribute as usize] = TokenStyle::new(rgb(0x4EC9B0));

        // Markdown specific
        styles[TokenKind::MarkdownHeading as usize] = TokenStyle::new(rgb(0x569CD6)).bold();
        styles[TokenKind::MarkdownBold as usize] = TokenStyle::new(rgb(0xD4D4D4)).bold();
        styles[TokenKind::MarkdownItalic as usize] = TokenStyle::new(rgb(0xD4D4D4)).italic();
        styles[TokenKind::MarkdownCode as usize] = TokenStyle::new(rgb(0xCE9178));
        styles[TokenKind::MarkdownLink as usize] = TokenStyle::new(rgb(0x4EC9B0)).underline();

        // Errors - red
        styles[TokenKind::Error as usize] = TokenStyle::new(rgb(0xF44747)).underline();

        Self { styles }
    }

    /// Create a new theme with default light colors (inspired by VS Code Light+).
    pub fn default_light() -> Self {
        let mut styles = vec![TokenStyle::new(rgb(0x000000)); 256];

        // Comments - green
        styles[TokenKind::Comment as usize] = TokenStyle::new(rgb(0x008000)).italic();

        // Strings - brown/red
        styles[TokenKind::String as usize] = TokenStyle::new(rgb(0xA31515));
        styles[TokenKind::Char as usize] = TokenStyle::new(rgb(0xA31515));

        // Numbers - green
        styles[TokenKind::Number as usize] = TokenStyle::new(rgb(0x098658));

        // Booleans and null - blue
        styles[TokenKind::Boolean as usize] = TokenStyle::new(rgb(0x0000FF)).bold();
        styles[TokenKind::Null as usize] = TokenStyle::new(rgb(0x0000FF)).bold();

        // Keywords - blue
        styles[TokenKind::Keyword as usize] = TokenStyle::new(rgb(0x0000FF));
        styles[TokenKind::KeywordControl as usize] = TokenStyle::new(rgb(0xAF00DB)).bold();
        styles[TokenKind::KeywordFunction as usize] = TokenStyle::new(rgb(0x0000FF)).bold();
        styles[TokenKind::KeywordImport as usize] = TokenStyle::new(rgb(0xAF00DB));
        styles[TokenKind::KeywordStorage as usize] = TokenStyle::new(rgb(0x0000FF));
        styles[TokenKind::KeywordType as usize] = TokenStyle::new(rgb(0x267F99));
        styles[TokenKind::KeywordOperator as usize] = TokenStyle::new(rgb(0x0000FF));

        // Identifiers
        styles[TokenKind::Identifier as usize] = TokenStyle::new(rgb(0x000000));
        styles[TokenKind::TypeName as usize] = TokenStyle::new(rgb(0x267F99));
        styles[TokenKind::FunctionName as usize] = TokenStyle::new(rgb(0x795E26));
        styles[TokenKind::VariableName as usize] = TokenStyle::new(rgb(0x001080));

        // Errors - red
        styles[TokenKind::Error as usize] = TokenStyle::new(rgb(0xFF0000)).underline();

        Self { styles }
    }

    /// Get the style for a given token kind.
    pub fn get_style(&self, kind: TokenKind) -> TokenStyle {
        self.styles.get(kind as usize).copied().unwrap_or(TokenStyle::new(rgb(0xD4D4D4)))
    }

    /// Set the style for a given token kind.
    pub fn set_style(&mut self, kind: TokenKind, style: TokenStyle) {
        if (kind as usize) < self.styles.len() {
            self.styles[kind as usize] = style;
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::default_dark()
    }
}

/// Helper to create an RGB color from a hex value.
const fn rgb(hex: u32) -> StraightRgba {
    // StraightRgba stores colors as 0xAABBGGRR (little-endian)
    let r = (hex >> 16) & 0xFF;
    let g = (hex >> 8) & 0xFF;
    let b = hex & 0xFF;
    let a = 0xFF;
    StraightRgba::from_le(r | (g << 8) | (b << 16) | (a << 24))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_default() {
        let theme = Theme::default();
        let style = theme.get_style(TokenKind::Keyword);
        assert!(style.fg.red() > 0 || style.fg.green() > 0 || style.fg.blue() > 0);
    }

    #[test]
    fn test_rgb_helper() {
        let color = rgb(0xFF0000);
        assert_eq!(color.red(), 0xFF);
        assert_eq!(color.green(), 0);
        assert_eq!(color.blue(), 0);
    }
}
