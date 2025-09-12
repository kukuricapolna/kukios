//! GUI Widgets for KukiOS
//! Provides reusable UI components like buttons, labels, and dialogs

use crate::gui::{
    graphics::{Rgb888, VgaDisplay},
    Point, Rect, GUI_ACCENT, GUI_BACKGROUND, GUI_ERROR, GUI_FOREGROUND, GUI_SUCCESS, GUI_WARNING,
};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

/// Basic button widget
#[derive(Debug, Clone)]
pub struct Button {
    pub rect: Rect,
    pub text: String,
    pub is_pressed: bool,
    pub is_enabled: bool,
    pub color: ButtonColor,
}

#[derive(Debug, Clone)]
pub enum ButtonColor {
    Default,
    Primary,
    Success,
    Warning,
    Danger,
}

impl Button {
    pub fn new(rect: Rect, text: String) -> Self {
        Self {
            rect,
            text,
            is_pressed: false,
            is_enabled: true,
            color: ButtonColor::Default,
        }
    }

    pub fn with_color(mut self, color: ButtonColor) -> Self {
        self.color = color;
        self
    }

    pub fn render(&self, display: &mut VgaDisplay) {
        let bg_color = if !self.is_enabled {
            Rgb888::new(0x60, 0x60, 0x60)
        } else if self.is_pressed {
            match self.color {
                ButtonColor::Default => GUI_FOREGROUND,
                ButtonColor::Primary => Rgb888::new(0x20, 0x60, 0xD0),
                ButtonColor::Success => Rgb888::new(0x00, 0x80, 0x00),
                ButtonColor::Warning => Rgb888::new(0xCC, 0x99, 0x00),
                ButtonColor::Danger => Rgb888::new(0xCC, 0x00, 0x00),
            }
        } else {
            match self.color {
                ButtonColor::Default => GUI_BACKGROUND,
                ButtonColor::Primary => GUI_ACCENT,
                ButtonColor::Success => GUI_SUCCESS,
                ButtonColor::Warning => GUI_WARNING,
                ButtonColor::Danger => GUI_ERROR,
            }
        };

        let text_color = if self.is_pressed
            || matches!(
                self.color,
                ButtonColor::Primary
                    | ButtonColor::Success
                    | ButtonColor::Warning
                    | ButtonColor::Danger
            ) {
            GUI_BACKGROUND
        } else {
            GUI_FOREGROUND
        };

        // Button background
        display.fill_rect(
            self.rect.x,
            self.rect.y,
            self.rect.width,
            self.rect.height,
            bg_color,
        );

        // Button border
        let border_color = if self.is_enabled {
            GUI_FOREGROUND
        } else {
            Rgb888::new(0x80, 0x80, 0x80)
        };

        display.draw_rect(
            self.rect.x,
            self.rect.y,
            self.rect.width,
            self.rect.height,
            border_color,
        );

        // Button text (centered)
        let text_x =
            self.rect.x + (self.rect.width as i32 / 2) - ((self.text.len() as i32 * 8) / 2);
        let text_y = self.rect.y + (self.rect.height as i32 / 2) - 4;

        display.draw_text(text_x, text_y, &self.text, text_color);
    }

    pub fn contains_point(&self, point: Point) -> bool {
        self.rect.contains_point(point)
    }

    pub fn set_pressed(&mut self, pressed: bool) {
        if self.is_enabled {
            self.is_pressed = pressed;
        }
    }
}

/// Simple text label widget
#[derive(Debug, Clone)]
pub struct Label {
    pub position: Point,
    pub text: String,
    pub color: Rgb888,
}

impl Label {
    pub fn new(position: Point, text: String) -> Self {
        Self {
            position,
            text,
            color: GUI_FOREGROUND,
        }
    }

    pub fn with_color(mut self, color: Rgb888) -> Self {
        self.color = color;
        self
    }

    pub fn render(&self, display: &mut VgaDisplay) {
        display.draw_text(self.position.x, self.position.y, &self.text, self.color);
    }
}

/// Dialog box for messages and confirmations
#[derive(Debug, Clone)]
pub struct Dialog {
    pub rect: Rect,
    pub title: String,
    pub message: String,
    pub dialog_type: DialogType,
    pub buttons: Vec<Button>,
    pub is_visible: bool,
}

#[derive(Debug, Clone)]
pub enum DialogType {
    Info,
    Warning,
    Error,
    Confirmation,
}

impl Dialog {
    pub fn new(title: String, message: String, dialog_type: DialogType) -> Self {
        let rect = Rect::new(80, 60, 160, 80);
        let mut dialog = Self {
            rect,
            title,
            message,
            dialog_type,
            buttons: Vec::new(),
            is_visible: false,
        };

        // Add appropriate buttons based on dialog type
        match dialog.dialog_type {
            DialogType::Info | DialogType::Warning | DialogType::Error => {
                dialog.add_button("OK".to_string(), ButtonColor::Primary);
            }
            DialogType::Confirmation => {
                dialog.add_button("Yes".to_string(), ButtonColor::Success);
                dialog.add_button("No".to_string(), ButtonColor::Danger);
            }
        }

        dialog
    }

    pub fn add_button(&mut self, text: String, color: ButtonColor) {
        let button_width = 50;
        let button_height = 16;
        let button_spacing = 10;
        let total_buttons_width = (self.buttons.len() as u32 + 1) * button_width
            + (self.buttons.len() as u32) * button_spacing;

        let start_x = self.rect.x + (self.rect.width as i32 / 2) - (total_buttons_width as i32 / 2);
        let button_y = self.rect.y + self.rect.height as i32 - button_height as i32 - 8;
        let button_x =
            start_x + (self.buttons.len() as i32 * (button_width as i32 + button_spacing as i32));

        let button = Button::new(
            Rect::new(button_x, button_y, button_width, button_height),
            text,
        )
        .with_color(color);

        self.buttons.push(button);
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    pub fn render(&self, display: &mut VgaDisplay) {
        if !self.is_visible {
            return;
        }

        // Dialog background with shadow effect
        display.fill_rect(
            self.rect.x + 2,
            self.rect.y + 2,
            self.rect.width,
            self.rect.height,
            Rgb888::new(0x10, 0x10, 0x10),
        );

        // Dialog main background
        display.fill_rect(
            self.rect.x,
            self.rect.y,
            self.rect.width,
            self.rect.height,
            GUI_BACKGROUND,
        );

        // Dialog border
        display.draw_rect(
            self.rect.x,
            self.rect.y,
            self.rect.width,
            self.rect.height,
            GUI_FOREGROUND,
        );

        // Title bar
        let title_color = match self.dialog_type {
            DialogType::Info => GUI_ACCENT,
            DialogType::Warning => GUI_WARNING,
            DialogType::Error => GUI_ERROR,
            DialogType::Confirmation => GUI_SUCCESS,
        };

        display.fill_rect(
            self.rect.x + 1,
            self.rect.y + 1,
            self.rect.width - 2,
            16,
            title_color,
        );

        // Title text
        display.draw_text(
            self.rect.x + 4,
            self.rect.y + 4,
            &self.title,
            GUI_BACKGROUND,
        );

        // Message text (word wrapped)
        let message_lines = self.wrap_text(&self.message, 18);
        let mut y_offset = 24;

        for line in message_lines {
            display.draw_text(
                self.rect.x + 8,
                self.rect.y + y_offset,
                &line,
                GUI_FOREGROUND,
            );
            y_offset += 12;
        }

        // Render buttons
        for button in &self.buttons {
            button.render(display);
        }
    }

    fn wrap_text(&self, text: &str, max_chars_per_line: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in text.split_whitespace() {
            if current_line.len() + word.len() + 1 <= max_chars_per_line {
                if !current_line.is_empty() {
                    current_line.push(' ');
                }
                current_line.push_str(word);
            } else {
                if !current_line.is_empty() {
                    lines.push(current_line);
                    current_line = String::new();
                }
                current_line.push_str(word);
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        lines
    }

    pub fn handle_click(&mut self, point: Point) -> Option<usize> {
        if !self.is_visible {
            return None;
        }

        for (i, button) in self.buttons.iter_mut().enumerate() {
            if button.contains_point(point) {
                button.set_pressed(true);
                return Some(i);
            }
        }
        None
    }
}

/// Progress bar widget
#[derive(Debug, Clone)]
pub struct ProgressBar {
    pub rect: Rect,
    pub progress: f32, // 0.0 to 1.0
    pub color: Rgb888,
    pub show_text: bool,
}

impl ProgressBar {
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            progress: 0.0,
            color: GUI_ACCENT,
            show_text: false,
        }
    }

    pub fn with_color(mut self, color: Rgb888) -> Self {
        self.color = color;
        self
    }

    pub fn with_text(mut self, show_text: bool) -> Self {
        self.show_text = show_text;
        self
    }

    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
    }

    pub fn render(&self, display: &mut VgaDisplay) {
        // Background
        display.fill_rect(
            self.rect.x,
            self.rect.y,
            self.rect.width,
            self.rect.height,
            GUI_BACKGROUND,
        );

        // Border
        display.draw_rect(
            self.rect.x,
            self.rect.y,
            self.rect.width,
            self.rect.height,
            GUI_FOREGROUND,
        );

        // Progress fill
        let fill_width = ((self.rect.width - 2) as f32 * self.progress) as u32;
        if fill_width > 0 {
            display.fill_rect(
                self.rect.x + 1,
                self.rect.y + 1,
                fill_width,
                self.rect.height - 2,
                self.color,
            );
        }

        // Progress text
        if self.show_text {
            let percentage = (self.progress * 100.0) as u32;
            let text = alloc::format!("{}%", percentage);
            let text_x = self.rect.x + (self.rect.width as i32 / 2) - ((text.len() as i32 * 8) / 2);
            let text_y = self.rect.y + (self.rect.height as i32 / 2) - 4;

            display.draw_text(text_x, text_y, &text, GUI_FOREGROUND);
        }
    }
}

/// Simple checkbox widget
#[derive(Debug, Clone)]
pub struct Checkbox {
    pub rect: Rect,
    pub label: String,
    pub is_checked: bool,
    pub is_enabled: bool,
}

impl Checkbox {
    pub fn new(rect: Rect, label: String) -> Self {
        Self {
            rect,
            label,
            is_checked: false,
            is_enabled: true,
        }
    }

    pub fn render(&self, display: &mut VgaDisplay) {
        let checkbox_size = 12;
        let checkbox_rect = Rect::new(self.rect.x, self.rect.y, checkbox_size, checkbox_size);

        // Checkbox background
        display.fill_rect(
            checkbox_rect.x,
            checkbox_rect.y,
            checkbox_rect.width,
            checkbox_rect.height,
            GUI_BACKGROUND,
        );

        // Checkbox border
        let border_color = if self.is_enabled {
            GUI_FOREGROUND
        } else {
            Rgb888::new(0x80, 0x80, 0x80)
        };

        display.draw_rect(
            checkbox_rect.x,
            checkbox_rect.y,
            checkbox_rect.width,
            checkbox_rect.height,
            border_color,
        );

        // Check mark
        if self.is_checked {
            display.draw_text(
                checkbox_rect.x + 2,
                checkbox_rect.y + 2,
                "X",
                if self.is_enabled {
                    GUI_ACCENT
                } else {
                    border_color
                },
            );
        }

        // Label
        let label_color = if self.is_enabled {
            GUI_FOREGROUND
        } else {
            Rgb888::new(0x80, 0x80, 0x80)
        };

        display.draw_text(
            checkbox_rect.x + checkbox_size as i32 + 4,
            checkbox_rect.y + 2,
            &self.label,
            label_color,
        );
    }

    pub fn contains_point(&self, point: Point) -> bool {
        // Include both checkbox and label area
        let full_rect = Rect::new(
            self.rect.x,
            self.rect.y,
            12 + 4 + (self.label.len() as u32 * 8),
            12,
        );
        full_rect.contains_point(point)
    }

    pub fn toggle(&mut self) {
        if self.is_enabled {
            self.is_checked = !self.is_checked;
        }
    }
}
