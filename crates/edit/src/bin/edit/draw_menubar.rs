// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use edit::helpers::*;
use edit::input::{kbmod, vk};
use edit::tui::*;
use edit::framebuffer::IndexedColor;
use stdext::arena_format;

use crate::localization::*;
use crate::state::*;

pub fn draw_menubar(ctx: &mut Context, state: &mut State) {
    ctx.menubar_begin();
    ctx.attr_background_rgba(state.menubar_color_bg);
    ctx.attr_foreground_rgba(state.menubar_color_fg);
    {
        let contains_focus = ctx.contains_focus();

        if ctx.menubar_menu_begin(loc(LocId::File), 'F') {
            draw_menu_file(ctx, state);
        }
        if !contains_focus && ctx.consume_shortcut(vk::F10) {
            ctx.steal_focus();
        }
        if state.documents.active().is_some() {
            if ctx.menubar_menu_begin(loc(LocId::Edit), 'E') {
                draw_menu_edit(ctx, state);
            }
            if ctx.menubar_menu_begin(loc(LocId::View), 'V') {
                draw_menu_view(ctx, state);
            }
        }
        if ctx.menubar_menu_begin(loc(LocId::Help), 'H') {
            draw_menu_help(ctx, state);
        }
    }
    ctx.menubar_end();
}

fn draw_menu_file(ctx: &mut Context, state: &mut State) {
    if ctx.menubar_menu_button(loc(LocId::FileNew), 'N', kbmod::CTRL | vk::N) {
        draw_add_untitled_document(ctx, state);
    }
    if ctx.menubar_menu_button(loc(LocId::FileOpen), 'O', kbmod::CTRL | vk::O) {
        state.wants_file_picker = StateFilePicker::Open;
    }
    if state.documents.active().is_some() {
        if ctx.menubar_menu_button(loc(LocId::FileSave), 'S', kbmod::CTRL | vk::S) {
            state.wants_save = true;
        }
        if ctx.menubar_menu_button(loc(LocId::FileSaveAs), 'A', vk::NULL) {
            state.wants_file_picker = StateFilePicker::SaveAs;
        }
        if ctx.menubar_menu_button(loc(LocId::FileClose), 'C', kbmod::CTRL | vk::W) {
            state.wants_close = true;
        }
    }
    if ctx.menubar_menu_button(loc(LocId::FileExit), 'X', kbmod::CTRL | vk::Q) {
        state.wants_exit = true;
    }
    ctx.menubar_menu_end();
}

fn draw_menu_edit(ctx: &mut Context, state: &mut State) {
    let doc = state.documents.active().unwrap();
    let mut tb = doc.buffer.borrow_mut();

    if ctx.menubar_menu_button(loc(LocId::EditUndo), 'U', kbmod::CTRL | vk::Z) {
        tb.undo();
        ctx.needs_rerender();
    }
    if ctx.menubar_menu_button(loc(LocId::EditRedo), 'R', kbmod::CTRL | vk::Y) {
        tb.redo();
        ctx.needs_rerender();
    }
    if ctx.menubar_menu_button(loc(LocId::EditCut), 'T', kbmod::CTRL | vk::X) {
        tb.cut(ctx.clipboard_mut());
        ctx.needs_rerender();
    }
    if ctx.menubar_menu_button(loc(LocId::EditCopy), 'C', kbmod::CTRL | vk::C) {
        tb.copy(ctx.clipboard_mut());
        ctx.needs_rerender();
    }
    if ctx.menubar_menu_button(loc(LocId::EditPaste), 'P', kbmod::CTRL | vk::V) {
        tb.paste(ctx.clipboard_ref());
        ctx.needs_rerender();
    }
    if state.wants_search.kind != StateSearchKind::Disabled {
        if ctx.menubar_menu_button(loc(LocId::EditFind), 'F', kbmod::CTRL | vk::F) {
            state.wants_search.kind = StateSearchKind::Search;
            state.wants_search.focus = true;
        }
        if ctx.menubar_menu_button(loc(LocId::EditReplace), 'L', kbmod::CTRL | vk::R) {
            state.wants_search.kind = StateSearchKind::Replace;
            state.wants_search.focus = true;
        }
    }
    if ctx.menubar_menu_button(loc(LocId::EditSelectAll), 'A', kbmod::CTRL | vk::A) {
        tb.select_all();
        ctx.needs_rerender();
    }
    ctx.menubar_menu_end();
}

fn draw_menu_view(ctx: &mut Context, state: &mut State) {
    if let Some(doc) = state.documents.active() {
        let mut tb = doc.buffer.borrow_mut();
        let word_wrap = tb.is_word_wrap_enabled();

        // All values on the statusbar are currently document specific.
        if ctx.menubar_menu_button(loc(LocId::ViewFocusStatusbar), 'S', vk::NULL) {
            state.wants_statusbar_focus = true;
        }
        if ctx.menubar_menu_button(loc(LocId::ViewGoToFile), 'F', kbmod::CTRL | vk::P) {
            state.wants_go_to_file = true;
        }
        if ctx.menubar_menu_button(loc(LocId::FileGoto), 'G', kbmod::CTRL | vk::G) {
            state.wants_goto = true;
        }
        if ctx.menubar_menu_checkbox(loc(LocId::ViewWordWrap), 'W', kbmod::ALT | vk::Z, word_wrap) {
            tb.set_word_wrap(!word_wrap);
            ctx.needs_rerender();
        }
    }

    ctx.menubar_menu_end();
}

fn draw_menu_help(ctx: &mut Context, state: &mut State) {
    if ctx.menubar_menu_button(loc(LocId::HelpSettings), 'S', vk::NULL) {
        state.wants_settings = true;
    }
    if ctx.menubar_menu_button(loc(LocId::HelpAbout), 'A', vk::NULL) {
        state.wants_about = true;
    }
    ctx.menubar_menu_end();
}

pub fn draw_dialog_about(ctx: &mut Context, state: &mut State) {
    ctx.modal_begin("about", loc(LocId::AboutDialogTitle));
    {
        ctx.block_begin("content");
        ctx.inherit_focus();
        ctx.attr_padding(Rect::three(1, 2, 1));
        {
            ctx.label("description", "Microsoft Edit");
            ctx.attr_overflow(Overflow::TruncateTail);
            ctx.attr_position(Position::Center);

            ctx.label(
                "version",
                &arena_format!(
                    ctx.arena(),
                    "{}{}",
                    loc(LocId::AboutDialogVersion),
                    env!("CARGO_PKG_VERSION")
                ),
            );
            ctx.attr_overflow(Overflow::TruncateHead);
            ctx.attr_position(Position::Center);

            ctx.label("copyright", "Copyright (c) Microsoft Corp 2025");
            ctx.attr_overflow(Overflow::TruncateTail);
            ctx.attr_position(Position::Center);

            ctx.block_begin("choices");
            ctx.inherit_focus();
            ctx.attr_padding(Rect::three(1, 2, 0));
            ctx.attr_position(Position::Center);
            {
                if ctx.button("ok", loc(LocId::Ok), ButtonStyle::default()) {
                    state.wants_about = false;
                }
                ctx.inherit_focus();
            }
            ctx.block_end();
        }
        ctx.block_end();
    }
    if ctx.modal_end() {
        state.wants_about = false;
    }
}

pub fn draw_dialog_settings(ctx: &mut Context, state: &mut State) {
    ctx.modal_begin("settings", loc(LocId::SettingsDialogTitle));
    {
        // Initialize input fields only once when dialog is opened
        if !state.settings_dialog_initialized {
            state.settings_dialog_initialized = true;
            
            // Initialize with current colors if set, otherwise leave empty
            if let Some(color) = state.settings.titlebar_color {
                state.settings_titlebar_color_input = crate::settings::Settings::color_to_hex_pub(color);
            }
            if let Some(color) = state.settings.selection_color {
                state.settings_selection_color_input = crate::settings::Settings::color_to_hex_pub(color);
            }
            if let Some(color) = state.settings.line_number_color {
                state.settings_line_number_color_input = crate::settings::Settings::color_to_hex_pub(color);
            }
            if let Some(color) = state.settings.line_separator_color {
                state.settings_line_separator_color_input = crate::settings::Settings::color_to_hex_pub(color);
            }
        }
        
        ctx.block_begin("content");
        ctx.inherit_focus();
        ctx.attr_padding(Rect::three(1, 2, 1));
        {
            ctx.label("titlebar-label", loc(LocId::SettingsTitlebarColor));
            ctx.attr_overflow(Overflow::TruncateTail);

            // Color input field
            ctx.editline("titlebar-color-input", &mut state.settings_titlebar_color_input);
            ctx.inherit_focus();
            ctx.attr_intrinsic_size(Size { width: 200, height: 1 });

            ctx.label("hint", loc(LocId::SettingsTitlebarColorHint));
            ctx.attr_overflow(Overflow::TruncateTail);
            ctx.attr_foreground_rgba(ctx.indexed(IndexedColor::BrightBlack));

            // Selection color section
            ctx.label("selection-label", loc(LocId::SettingsSelectionColor));
            ctx.attr_overflow(Overflow::TruncateTail);

            // Selection color input field
            ctx.editline("selection-color-input", &mut state.settings_selection_color_input);
            ctx.inherit_focus();
            ctx.attr_intrinsic_size(Size { width: 200, height: 1 });

            ctx.label("hint2", loc(LocId::SettingsSelectionColorHint));
            ctx.attr_overflow(Overflow::TruncateTail);
            ctx.attr_foreground_rgba(ctx.indexed(IndexedColor::BrightBlack));

            // Line number color section
            ctx.label("line-number-label", loc(LocId::SettingsLineNumberColor));
            ctx.attr_overflow(Overflow::TruncateTail);

            // Line number color input field
            ctx.editline("line-number-color-input", &mut state.settings_line_number_color_input);
            ctx.inherit_focus();
            ctx.attr_intrinsic_size(Size { width: 200, height: 1 });

            ctx.label("hint3", loc(LocId::SettingsLineNumberColorHint));
            ctx.attr_overflow(Overflow::TruncateTail);
            ctx.attr_foreground_rgba(ctx.indexed(IndexedColor::BrightBlack));

            // Line separator color section
            ctx.label("line-separator-label", loc(LocId::SettingsLineSeparatorColor));
            ctx.attr_overflow(Overflow::TruncateTail);

            // Line separator color input field
            ctx.editline("line-separator-color-input", &mut state.settings_line_separator_color_input);
            ctx.inherit_focus();
            ctx.attr_intrinsic_size(Size { width: 200, height: 1 });

            ctx.label("hint4", loc(LocId::SettingsLineSeparatorColorHint));
            ctx.attr_overflow(Overflow::TruncateTail);
            ctx.attr_foreground_rgba(ctx.indexed(IndexedColor::BrightBlack));

            ctx.block_begin("choices");
            ctx.inherit_focus();
            ctx.attr_padding(Rect::three(1, 2, 0));
            ctx.attr_position(Position::Center);
            {
                if ctx.button("save", loc(LocId::Save), ButtonStyle::default()) {
                    // Track old values for restart warning
                    let old_selection_color = state.settings.selection_color;
                    let old_line_number_color = state.settings.line_number_color;
                    let old_line_separator_color = state.settings.line_separator_color;
                    
                    // Try to parse and save titlebar color (empty or "#" = default)
                    let titlebar_input = state.settings_titlebar_color_input.trim();
                    if titlebar_input.is_empty() || titlebar_input == "#" {
                        state.settings.titlebar_color = None;
                    } else if let Some(color) = crate::settings::Settings::parse_color_pub(titlebar_input) {
                        state.settings.titlebar_color = Some(color);
                    }
                    
                    // Try to parse and save selection color (empty or "#" = default)
                    let selection_input = state.settings_selection_color_input.trim();
                    if selection_input.is_empty() || selection_input == "#" {
                        state.settings.selection_color = None;
                    } else if let Some(color) = crate::settings::Settings::parse_color_pub(selection_input) {
                        state.settings.selection_color = Some(color);
                    }
                    
                    // Try to parse and save line number color (empty or "#" = default)
                    let line_number_input = state.settings_line_number_color_input.trim();
                    if line_number_input.is_empty() || line_number_input == "#" {
                        state.settings.line_number_color = None;
                    } else if let Some(color) = crate::settings::Settings::parse_color_pub(line_number_input) {
                        state.settings.line_number_color = Some(color);
                    }
                    
                    // Try to parse and save line separator color (empty or "#" = default)
                    let line_separator_input = state.settings_line_separator_color_input.trim();
                    if line_separator_input.is_empty() || line_separator_input == "#" {
                        state.settings.line_separator_color = None;
                    } else if let Some(color) = crate::settings::Settings::parse_color_pub(line_separator_input) {
                        state.settings.line_separator_color = Some(color);
                    }
                    
                    let _ = state.settings.save();
                    
                    // Apply the colors immediately
                    state.menubar_color_bg = state.settings.titlebar_color.unwrap_or_else(|| {
                        ctx.indexed(IndexedColor::Background).oklab_blend(ctx.indexed_alpha(
                            IndexedColor::BrightBlue,
                            1,
                            2,
                        ))
                    });
                    state.menubar_color_fg = ctx.contrasted(state.menubar_color_bg);
                    
                    state.selection_color_bg = state.settings.selection_color.unwrap_or_else(|| {
                        ctx.indexed(IndexedColor::Green)
                    });
                    
                    state.line_number_color = state.settings.line_number_color;
                    state.line_separator_color = state.settings.line_separator_color;
                    
                    // Check if any colors requiring restart changed
                    let needs_restart = old_selection_color != state.settings.selection_color
                        || old_line_number_color != state.settings.line_number_color
                        || old_line_separator_color != state.settings.line_separator_color;
                    
                    if needs_restart {
                        state.wants_restart_warning = true;
                    }
                    
                    state.wants_settings = false;
                    state.settings_dialog_initialized = false;
                    state.settings_titlebar_color_input.clear();
                    state.settings_selection_color_input.clear();
                    state.settings_line_number_color_input.clear();
                    state.settings_line_separator_color_input.clear();
                    ctx.needs_rerender();
                }
                ctx.inherit_focus();

                if ctx.button("cancel", loc(LocId::Cancel), ButtonStyle::default()) {
                    state.wants_settings = false;
                    state.settings_dialog_initialized = false;
                    state.settings_titlebar_color_input.clear();
                    state.settings_selection_color_input.clear();
                    state.settings_line_number_color_input.clear();
                    state.settings_line_separator_color_input.clear();
                }
            }
            ctx.block_end();
        }
        ctx.block_end();
    }
    if ctx.modal_end() {
        state.wants_settings = false;
        state.settings_dialog_initialized = false;
        state.settings_titlebar_color_input.clear();
        state.settings_selection_color_input.clear();
        state.settings_line_number_color_input.clear();
        state.settings_line_separator_color_input.clear();
    }
}

pub fn draw_dialog_restart_warning(ctx: &mut Context, state: &mut State) {
    ctx.modal_begin("restart-warning", loc(LocId::SettingsDialogTitle));
    {
        ctx.block_begin("content");
        ctx.inherit_focus();
        ctx.attr_padding(Rect::three(1, 2, 1));
        {
            ctx.label("message", loc(LocId::SettingsRestartRequired));
            ctx.attr_overflow(Overflow::TruncateTail);

            ctx.block_begin("choices");
            ctx.inherit_focus();
            ctx.attr_padding(Rect::three(1, 2, 0));
            ctx.attr_position(Position::Center);
            {
                if ctx.button("ok", loc(LocId::Ok), ButtonStyle::default()) {
                    state.wants_restart_warning = false;
                }
                ctx.inherit_focus();
            }
            ctx.block_end();
        }
        ctx.block_end();
    }
    if ctx.modal_end() {
        state.wants_restart_warning = false;
    }
}
