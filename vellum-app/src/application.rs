use vellum_core::editor::Editor;
use crossterm::{cursor, execute};
use keybinds::{KeySeq, Keybinds, Keybind};
use crate::actions::Action;
use serde::Deserialize;
use std::path::Path;
use std::io::stdout;
use std::collections::HashMap;
use vellum_core::cursor::Direction;

#[derive(Deserialize, PartialEq, Eq, Hash)]
pub enum ApplicationSetting {
    WrapDoc,
    WrapLine,
}

#[derive(Deserialize)]
pub enum SettingValue {
    Bool(bool),
    String(String),
    U32(u32),
}

pub struct ApplicationSettings {
    settings: HashMap<ApplicationSetting, SettingValue>
}

impl Default for ApplicationSettings {
    fn default() -> Self {
        let mut settings: HashMap<ApplicationSetting, SettingValue> = HashMap::new();
        settings.insert(ApplicationSetting::WrapDoc, SettingValue::Bool(false));
        settings.insert(ApplicationSetting::WrapLine, SettingValue::Bool(false));
        Self {
            settings
        }
    }
}

impl ApplicationSettings {

    pub fn set_setting(&mut self, setting: ApplicationSetting, value: SettingValue) {
        if let Some(x) = self.settings.get_mut(&setting) {
            *x = value;
        } else {
            self.settings.insert(setting, value);
        }
    }

    pub fn get_setting(&self, setting: ApplicationSetting) -> Option<&SettingValue> {
        self.settings.get(&setting)
    }
}

#[derive(Deserialize)]
struct ApplicationConfig {
    keybindings: Keybinds<Action>,
}

impl ApplicationConfig {

    pub fn new() -> Self {
        use std::fs;
        let default_keybinds_path = Path::new("vellum-app/src/default_keybinds.toml");
        let toml_str = fs::read_to_string(default_keybinds_path)
            .expect("Failed to read default_keybinds.toml");
        let keybindings: Keybinds<Action> = toml::from_str(&toml_str)
            .expect("Failed to parse default_keybinds.toml");
        Self { keybindings }
    }

    pub fn set_keybind(&mut self, binding: KeySeq, action: Action) {
        self.keybindings.push(Keybind::new(binding, action));
    }
}

pub struct Application {
    editor: Editor, // TODO: Multiple editors
    config: ApplicationConfig,
    settings: ApplicationSettings,
}

impl Application {

    pub fn new(view_size: (usize, usize)) -> Self {
        Self {
            editor: Editor::new(view_size),
            config: ApplicationConfig::new(),
            settings: ApplicationSettings::default()
        }
    }

    pub fn execute_action(&mut self, a: Action) {
        match a {
            Action::Exit => {}
            Action::ShiftCursorUp => {
                self.editor.shift_cursor(Direction::Up, self.settings.g, self.settings.wrap_line);
                let new_cursor_pos = self.editor.get_cursor_pos().as_tuple();
                // TODO: If TUI...
                execute!(stdout(), cursor::MoveTo(new_cursor_pos.0 as u16, new_cursor_pos.1 as u16));
            }
            Action::ShiftCursorUp => {}
            Action::ShiftCursorUp => {}
            Action::ShiftCursorUp => {}
            _ => {}
        }
    }
}