// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::fs;
use std::path::PathBuf;

use edit::oklab::StraightRgba;
use edit::json;
use stdext::arena::scratch_arena;

use crate::apperr;

/// Application settings structure
#[derive(Debug, Clone)]
pub struct Settings {
    /// Custom color for the title bar/menu bar background (RGBA format)
    /// If None, the default calculated color will be used
    pub titlebar_color: Option<StraightRgba>,
    /// Custom color for selected items background (RGBA format)
    /// If None, the default green color will be used
    pub selection_color: Option<StraightRgba>,
    /// Custom color for line numbers in the editor (RGBA format)
    /// If None, the default color will be used
    pub line_number_color: Option<StraightRgba>,
    /// Custom color for the separator between line numbers and text (RGBA format)
    /// If None, the default color will be used
    pub line_separator_color: Option<StraightRgba>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            titlebar_color: None,
            selection_color: None,
            line_number_color: None,
            line_separator_color: None,
        }
    }
}

impl Settings {
    /// Load settings from the config file
    pub fn load() -> apperr::Result<Self> {
        let config_path = Self::config_path()?;
        
        if !config_path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(&config_path)?;

        Self::parse(&contents)
    }

    /// Parse settings from JSON string
    fn parse(json_str: &str) -> apperr::Result<Self> {
        let mut settings = Self::default();
        let arena = scratch_arena(None);

        match json::parse(&arena, json_str) {
            Ok(root) => {
                if let Some(obj) = root.as_object() {
                    // Parse titlebar_color if present
                    if let Some(color_str) = obj.get_str("titlebar_color") {
                        settings.titlebar_color = Self::parse_color(color_str);
                    }
                    // Parse selection_color if present
                    if let Some(color_str) = obj.get_str("selection_color") {
                        settings.selection_color = Self::parse_color(color_str);
                    }
                    // Parse line_number_color if present
                    if let Some(color_str) = obj.get_str("line_number_color") {
                        settings.line_number_color = Self::parse_color(color_str);
                    }
                    // Parse line_separator_color if present
                    if let Some(color_str) = obj.get_str("line_separator_color") {
                        settings.line_separator_color = Self::parse_color(color_str);
                    }
                }
            }
            Err(_err) => {
                // Ignore parse errors and return default settings
            }
        }

        Ok(settings)
    }

    /// Parse color from hex string (e.g., "#RRGGBB" or "#RRGGBBAA")
    fn parse_color(s: &str) -> Option<StraightRgba> {
        let s = s.trim();
        if !s.starts_with('#') {
            return None;
        }

        let hex = &s[1..];
        let len = hex.len();

        if len != 6 && len != 8 {
            return None;
        }

        let r = u32::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u32::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u32::from_str_radix(&hex[4..6], 16).ok()?;
        let a = if len == 8 {
            u32::from_str_radix(&hex[6..8], 16).ok()?
        } else {
            255
        };

        // StraightRgba stores colors as 0xAABBGGRR (little-endian)
        Some(StraightRgba::from_le(r | (g << 8) | (b << 16) | (a << 24)))
    }

    /// Save settings to the config file
    pub fn save(&self) -> apperr::Result<()> {
        let config_path = Self::config_path()?;
        
        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = self.to_json();
        fs::write(&config_path, json)?;

        Ok(())
    }

    /// Convert settings to JSON string
    fn to_json(&self) -> String {
        let mut json = String::from("{\n");
        let mut first = true;

        if let Some(color) = self.titlebar_color {
            let color_str = Self::color_to_hex(color);
            json.push_str(&format!("  \"titlebar_color\": \"{}\"", color_str));
            first = false;
        }

        if let Some(color) = self.selection_color {
            if !first {
                json.push_str(",\n");
            }
            let color_str = Self::color_to_hex(color);
            json.push_str(&format!("  \"selection_color\": \"{}\"", color_str));
            first = false;
        }

        if let Some(color) = self.line_number_color {
            if !first {
                json.push_str(",\n");
            }
            let color_str = Self::color_to_hex(color);
            json.push_str(&format!("  \"line_number_color\": \"{}\"", color_str));
            first = false;
        }

        if let Some(color) = self.line_separator_color {
            if !first {
                json.push_str(",\n");
            }
            let color_str = Self::color_to_hex(color);
            json.push_str(&format!("  \"line_separator_color\": \"{}\"", color_str));
            first = false;
        }

        if !first {
            json.push('\n');
        }
        json.push_str("}\n");
        json
    }

    /// Convert color to hex string
    fn color_to_hex(color: StraightRgba) -> String {
        let r = color.red();
        let g = color.green();
        let b = color.blue();
        let a = color.alpha();
        
        if a == 255 {
            format!("#{:02X}{:02X}{:02X}", r, g, b)
        } else {
            format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
        }
    }

    /// Public wrapper for color_to_hex
    pub fn color_to_hex_pub(color: StraightRgba) -> String {
        Self::color_to_hex(color)
    }

    /// Public wrapper for parse_color
    pub fn parse_color_pub(s: &str) -> Option<StraightRgba> {
        Self::parse_color(s)
    }

    /// Get the path to the config file
    fn config_path() -> apperr::Result<PathBuf> {
        #[cfg(target_os = "windows")]
        {
            if let Ok(appdata) = std::env::var("APPDATA") {
                let mut path = PathBuf::from(appdata);
                path.push("edit");
                path.push("settings.json");
                return Ok(path);
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            if let Ok(home) = std::env::var("HOME") {
                let mut path = PathBuf::from(home);
                path.push(".config");
                path.push("edit");
                path.push("settings.json");
                return Ok(path);
            }
        }

        // Fallback to current directory
        Ok(PathBuf::from("settings.json"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color() {
        let color = Settings::parse_color("#FF0000").unwrap();
        assert_eq!(color.red(), 255);
        assert_eq!(color.green(), 0);
        assert_eq!(color.blue(), 0);
        assert_eq!(color.alpha(), 255);

        let color = Settings::parse_color("#00FF00FF").unwrap();
        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 0);
        assert_eq!(color.alpha(), 255);

        let color = Settings::parse_color("#0000FF80").unwrap();
        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 0);
        assert_eq!(color.blue(), 255);
        assert_eq!(color.alpha(), 128);

        assert!(Settings::parse_color("FF0000").is_none());
        assert!(Settings::parse_color("#FF").is_none());
    }

    #[test]
    fn test_color_to_hex() {
        let color = StraightRgba::from_le(255 | (0 << 8) | (0 << 16) | (255 << 24));
        assert_eq!(Settings::color_to_hex(color), "#FF0000");

        let color = StraightRgba::from_le(0 | (255 << 8) | (0 << 16) | (128 << 24));
        assert_eq!(Settings::color_to_hex(color), "#00FF0080");
    }
}
