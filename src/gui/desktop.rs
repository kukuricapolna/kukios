//! Desktop Environment for KukiOS GUI
//! Provides window management, taskbar, and desktop functionality

use crate::gui::{graphics::VgaDisplay, Rect, GUI_ACCENT, GUI_BACKGROUND, GUI_FOREGROUND};
use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};

/// Represents a window in the desktop environment
#[derive(Debug, Clone)]
pub struct Window {
    pub id: usize,
    pub title: String,
    pub rect: Rect,
    pub is_active: bool,
    pub is_minimized: bool,
    pub is_maximized: bool,
    pub content: WindowContent,
    pub terminal_state: TerminalState,
}

/// Different types of window content
#[derive(Debug, Clone)]
pub enum WindowContent {
    Terminal,
    FileManager,
    About,
    Custom(String),
}

/// Terminal state for interactive terminal windows
#[derive(Debug, Clone)]
pub struct TerminalState {
    pub output_lines: Vec<String>,
    pub current_input: String,
    pub cursor_position: usize,
    pub show_cursor: bool,
    pub prompt: String,
}

impl TerminalState {
    pub fn new() -> Self {
        Self {
            output_lines: vec![
                "KukiOS Terminal v1.0".to_string(),
                "Type 'help' for commands".to_string(),
                "".to_string(),
            ],
            current_input: String::new(),
            cursor_position: 0,
            show_cursor: true,
            prompt: "$ ".to_string(),
        }
    }

    pub fn add_output(&mut self, line: String) {
        self.output_lines.push(line);
        // Keep only last 10 lines to fit in window
        if self.output_lines.len() > 10 {
            self.output_lines.remove(0);
        }
    }

    pub fn execute_command(&mut self, command: &str) {
        // Add the command to output
        self.add_output(format!("{}{}", self.prompt, command));

        // Process the command
        match command.trim() {
            "help" => {
                self.add_output("Available commands:".to_string());
                self.add_output("  help - Show this help".to_string());
                self.add_output("  clear - Clear terminal".to_string());
                self.add_output("  about - System info".to_string());
                self.add_output("  exit - Close terminal".to_string());
            }
            "clear" => {
                self.output_lines.clear();
                self.add_output("Terminal cleared".to_string());
            }
            "about" => {
                self.add_output("KukiOS Terminal".to_string());
                self.add_output("Version: 0.1.0".to_string());
                self.add_output("GUI Mode Active".to_string());
            }
            "exit" => {
                self.add_output("Terminal session ended".to_string());
            }
            "easteregg" => {
                self.add_output(
                    "Hey, you proved yourself to be a big fan of Kukiweb ;)".to_string(),
                );
            }
            "" => {
                // Empty command, just show new prompt
            }
            _ => {
                self.add_output(format!("Unknown command: {}", command));
                self.add_output("Type 'help' for available commands".to_string());
            }
        }

        self.current_input.clear();
        self.cursor_position = 0;
    }

    pub fn handle_char(&mut self, ch: char) {
        if ch == '\n' || ch == '\r' {
            let command = self.current_input.clone();
            self.execute_command(&command);
        } else if ch == '\x08' {
            if self.cursor_position > 0 {
                self.current_input.remove(self.cursor_position - 1);
                self.cursor_position -= 1;
            }
        } else if ch.is_ascii_graphic() || ch == ' ' {
            self.current_input.insert(self.cursor_position, ch);
            self.cursor_position += 1;
        }
    }
}

impl Window {
    pub fn new(id: usize, title: String, rect: Rect, content: WindowContent) -> Self {
        let terminal_state = if matches!(content, WindowContent::Terminal) {
            TerminalState::new()
        } else {
            TerminalState::new()
        };

        Self {
            id,
            title,
            rect,
            is_active: false,
            is_minimized: false,
            is_maximized: false,
            content,
            terminal_state,
        }
    }

    pub fn handle_input(&mut self, ch: char) {
        if matches!(self.content, WindowContent::Terminal) {
            self.terminal_state.handle_char(ch);
        }
    }

    pub fn render(&self, display: &mut VgaDisplay) {
        if self.is_minimized {
            return;
        }

        display.fill_rect(
            self.rect.x,
            self.rect.y,
            self.rect.width,
            self.rect.height,
            GUI_BACKGROUND,
        );

        display.draw_rect(
            self.rect.x,
            self.rect.y,
            self.rect.width,
            self.rect.height,
            GUI_FOREGROUND,
        );

        display.fill_rect(
            self.rect.x + 1,
            self.rect.y + 1,
            self.rect.width - 2,
            16,
            GUI_FOREGROUND,
        );

        display.draw_text(
            self.rect.x + 4,
            self.rect.y + 4,
            &self.title,
            GUI_BACKGROUND,
        );

        self.render_content_with_cursor(display);
    }

    fn render_content_with_cursor(&self, display: &mut VgaDisplay) {
        let content_x = self.rect.x + 4;
        let content_y = self.rect.y + 20;

        match &self.content {
            WindowContent::Terminal => {
                let mut y_offset = 0;

                // Render terminal output lines
                for (i, line) in self.terminal_state.output_lines.iter().enumerate() {
                    if y_offset + 12 > (self.rect.height as i32 - 40) {
                        break; // Don't render lines that would go outside window
                    }
                    display.draw_text(content_x, content_y + y_offset, line, GUI_FOREGROUND);
                    y_offset += 12;
                }

                // Add some space before input line
                y_offset += 4;

                // Render current input line with prompt
                let input_line = format!(
                    "{}{}",
                    self.terminal_state.prompt, self.terminal_state.current_input
                );
                display.draw_text(content_x, content_y + y_offset, &input_line, GUI_FOREGROUND);

                // Draw cursor at current position
                if self.terminal_state.show_cursor {
                    let cursor_x = content_x
                        + ((self.terminal_state.prompt.len() + self.terminal_state.cursor_position)
                            as i32
                            * 8);
                    display.draw_text(cursor_x, content_y + y_offset, "_", GUI_ACCENT);
                }
            }
            WindowContent::FileManager => {
                display.draw_text(content_x, content_y, "File Manager", GUI_FOREGROUND);
                display.draw_text(content_x, content_y + 12, "Root directory:", GUI_FOREGROUND);
                display.draw_text(content_x + 8, content_y + 24, "test.txt", GUI_FOREGROUND);
                display.draw_text(content_x + 8, content_y + 36, "debug.txt", GUI_FOREGROUND);
                display.draw_text(content_x + 8, content_y + 48, "boot.bin", GUI_FOREGROUND);
            }
            WindowContent::About => {
                display.draw_text(content_x, content_y, "About KukiOS", GUI_FOREGROUND);
                display.draw_text(content_x, content_y + 12, "Version: 0.1.0", GUI_FOREGROUND);
                display.draw_text(content_x, content_y + 24, "Language: Rust", GUI_FOREGROUND);
                display.draw_text(
                    content_x,
                    content_y + 36,
                    "GUI Mode: VGA 13h",
                    GUI_FOREGROUND,
                );
                display.draw_text(content_x, content_y + 48, "By: Kukiweb.cz", GUI_FOREGROUND);
            }
            WindowContent::Custom(text) => {
                if text == "GUI Help Window" {
                    // Special help window content
                    display.draw_text(content_x, content_y, "GUI Help & Commands", GUI_FOREGROUND);
                    display.draw_text(
                        content_x,
                        content_y + 12,
                        "Available commands:",
                        GUI_FOREGROUND,
                    );
                    display.draw_text(
                        content_x,
                        content_y + 24,
                        "- help: Show this help",
                        GUI_FOREGROUND,
                    );
                    display.draw_text(
                        content_x,
                        content_y + 36,
                        "- about: System info",
                        GUI_FOREGROUND,
                    );
                    display.draw_text(
                        content_x,
                        content_y + 48,
                        "- clear: Refresh screen",
                        GUI_FOREGROUND,
                    );
                    display.draw_text(
                        content_x,
                        content_y + 60,
                        "- exit/quit: Return to CLI",
                        GUI_FOREGROUND,
                    );
                    display.draw_text(
                        content_x,
                        content_y + 72,
                        "- refresh: Redraw GUI",
                        GUI_FOREGROUND,
                    );
                    display.draw_text(
                        content_x,
                        content_y + 84,
                        "Type commands in console",
                        GUI_FOREGROUND,
                    );
                    display.draw_text(
                        content_x,
                        content_y + 96,
                        "below this GUI display",
                        GUI_FOREGROUND,
                    );
                } else {
                    display.draw_text(content_x, content_y, text, GUI_FOREGROUND);
                }
            }
        }
    }
}

/// Main desktop environment
use crate::gui::{Button, GuiCursor, Point};

pub struct Desktop {
    pub windows: Vec<Window>,
    pub active_window: Option<usize>,
    pub next_window_id: usize,
    pub taskbar: Taskbar,
    pub cursor: GuiCursor,
}

impl Desktop {
    pub fn new() -> Self {
        Self {
            windows: vec![Window::new(
                10,
                "Kukiweb".to_string(),
                Rect {
                    x: 10,
                    y: 10,
                    width: 100,
                    height: 100,
                },
                WindowContent::Custom("Vitejte na kukiweb.cz".to_string()),
            )],
            active_window: None,
            next_window_id: 1,
            taskbar: Taskbar::new(),
            cursor: GuiCursor::new(160, 100), // Start at center
        }
    }

    pub fn init(&mut self, _display: &mut VgaDisplay) {
        // Create initial windows
        self.create_window(
            "Terminal".to_string(),
            Rect::new(20, 30, 200, 120),
            WindowContent::Terminal,
        );
        self.create_window(
            "About KukiOS".to_string(),
            Rect::new(50, 60, 180, 100),
            WindowContent::About,
        );
        self.create_window(
            "Help".to_string(),
            Rect::new(80, 40, 220, 130),
            WindowContent::Custom("GUI Help Window".to_string()),
        );

        // Set About window as modal and active at startup
        if let Some(pos) = self
            .windows
            .iter()
            .position(|w| matches!(w.content, WindowContent::About))
        {
            self.active_window = Some(pos);
            self.windows[pos].is_active = true;
        }
    }

    pub fn create_window(&mut self, title: String, rect: Rect, content: WindowContent) -> usize {
        let id = self.next_window_id;
        self.next_window_id += 1;

        let window = Window::new(id, title, rect, content);
        self.windows.push(window);

        // Update taskbar
        self.taskbar
            .add_window(id, &self.windows.last().unwrap().title);

        id
    }

    pub fn close_window(&mut self, window_id: usize) {
        if let Some(pos) = self.windows.iter().position(|w| w.id == window_id) {
            let _window = self.windows.remove(pos);
            self.taskbar.remove_window(window_id);

            // Update active window if needed
            if self.active_window == Some(pos) {
                self.active_window = if self.windows.is_empty() {
                    None
                } else if pos >= self.windows.len() {
                    Some(self.windows.len() - 1)
                } else {
                    Some(pos)
                };

                // Set new active window
                if let Some(active_idx) = self.active_window {
                    for (i, window) in self.windows.iter_mut().enumerate() {
                        window.is_active = i == active_idx;
                    }
                }
            }
        }
    }

    pub fn activate_window(&mut self, window_id: usize) {
        // Deactivate all windows
        for window in &mut self.windows {
            window.is_active = false;
        }

        // Find and activate the target window
        if let Some(pos) = self.windows.iter().position(|w| w.id == window_id) {
            self.windows[pos].is_active = true;
            self.active_window = Some(pos);
        }
    }

    pub fn handle_keyboard_input(&mut self, ch: char) {
        // Arrow keys for cursor movement (using WASD for demo)
        match ch {
            'w' => self.cursor.move_by(0, -5),
            'a' => self.cursor.move_by(-5, 0),
            's' => self.cursor.move_by(0, 5),
            'd' => self.cursor.move_by(5, 0),
            '\n' | ' ' => {
                // "Click" at cursor position
                self.handle_cursor_click();
            }
            _ => {
                // Send input to active terminal window
                if let Some(active_idx) = self.active_window {
                    if active_idx < self.windows.len() {
                        self.windows[active_idx].handle_input(ch);
                    }
                }
            }
        }
    }

    /// Handles a click at the cursor position (activates window or button)
    pub fn handle_cursor_click(&mut self) {
        let point = self.cursor.position;
        // Find window under cursor, defer activation to avoid overlapping mutable borrows
        let mut window_to_activate = None;
        for window in self.windows.iter() {
            if window.rect.contains_point(point) {
                window_to_activate = Some(window.id);
                break;
            }
        }
        if let Some(window_id) = window_to_activate {
            self.activate_window(window_id);
            // If window has buttons, check for button click
            // (Extend this for your actual button logic)
        }
        // You can extend this to check for buttons, icons, etc.
    }

    pub fn update_terminal_cursors(&mut self) {
        // Toggle cursor visibility for blinking effect
        for window in &mut self.windows {
            if matches!(window.content, WindowContent::Terminal) {
                window.terminal_state.show_cursor = !window.terminal_state.show_cursor;
            }
        }
    }

    pub fn render(&mut self, display: &mut VgaDisplay) {
        // Don't clear screen here - it's done in the main GUI loop

        // Draw desktop background pattern (simplified)
        self.draw_desktop_background(display);

        // If About window is active, render only it (modal behavior)
        if let Some(active_idx) = self.active_window {
            if matches!(self.windows[active_idx].content, WindowContent::About) {
                self.windows[active_idx].render(display);
            } else {
                // Render windows (inactive first, then active on top)
                let mut active_window: Option<Window> = None;

                for window in &self.windows {
                    if window.is_active {
                        active_window = Some(window.clone());
                    } else {
                        window.render(display);
                    }
                }

                // Render active window on top
                if let Some(ref window) = active_window {
                    window.render(display);
                }
            }
        } else {
            // Fallback: render all windows
            for window in &self.windows {
                window.render(display);
            }
        }

        // Render taskbar
        self.taskbar.render(display);

        // Draw desktop info
        self.draw_desktop_info(display);

        // Draw GUI cursor (mouse-like)
        self.render_cursor(display);
    }

    /// Draws the GUI cursor as a small rectangle or crosshair
    fn render_cursor(&self, display: &mut VgaDisplay) {
        if self.cursor.is_visible {
            let x = self.cursor.position.x;
            let y = self.cursor.position.y;
            // Draw a small crosshair cursor (3x3)
            display.fill_rect(x - 1, y, 3, 1, crate::gui::GUI_ACCENT);
            display.fill_rect(x, y - 1, 1, 3, crate::gui::GUI_ACCENT);
        }
    }

    fn draw_desktop_background(&self, _display: &mut VgaDisplay) {
        // Skip drawing background pattern to eliminate flicker completely
        // The clear operation already provides a solid background
    }

    fn draw_desktop_info(&self, display: &mut VgaDisplay) {
        // Draw KukiOS info in top-right corner
        display.draw_text(250, 5, "KukiOS", GUI_ACCENT);
        display.draw_text(250, 15, "v0.1.0", GUI_FOREGROUND);

        // Add interaction hints
        display.draw_text(5, 5, "Interactive GUI Mode", GUI_FOREGROUND);
        display.draw_text(5, 15, "Commands: help, about, exit", GUI_FOREGROUND);
    }
}

/// Simple taskbar at the bottom of the screen
pub struct Taskbar {
    pub height: u32,
    pub buttons: Vec<TaskbarButton>,
}

#[derive(Debug, Clone)]
pub struct TaskbarButton {
    pub window_id: usize,
    pub title: String,
    pub rect: Rect,
    pub is_active: bool,
}

impl Taskbar {
    pub fn new() -> Self {
        Self {
            height: 20,
            buttons: Vec::new(),
        }
    }

    pub fn add_window(&mut self, window_id: usize, title: &str) {
        let button_width = 80;
        let x = (self.buttons.len() * button_width as usize) as i32;
        let y = 200 - self.height as i32;

        let button = TaskbarButton {
            window_id,
            title: title.to_string(),
            rect: Rect::new(x, y, button_width, self.height),
            is_active: false,
        };

        self.buttons.push(button);
    }

    pub fn remove_window(&mut self, window_id: usize) {
        self.buttons.retain(|b| b.window_id != window_id);
        self.reposition_buttons();
    }

    fn reposition_buttons(&mut self) {
        let button_width = 80;
        let y = 200 - self.height as i32;

        for (i, button) in self.buttons.iter_mut().enumerate() {
            let x = (i * button_width as usize) as i32;
            button.rect = Rect::new(x, y, button_width, self.height);
        }
    }

    pub fn render(&self, display: &mut VgaDisplay) {
        // Simple taskbar - just a line at the bottom
        display.draw_horizontal_line(0, 199, 320, GUI_FOREGROUND);

        // Draw taskbar with system info
        display.draw_text(5, 185, "KukiOS GUI", GUI_FOREGROUND);
        display.draw_text(80, 185, "320x200", GUI_FOREGROUND);
        display.draw_text(150, 185, "VGA Mode", GUI_FOREGROUND);
        display.draw_text(220, 185, "Ready", GUI_FOREGROUND);
    }
}

impl TaskbarButton {
    pub fn render(&self, _display: &mut VgaDisplay) {
        // Skip rendering taskbar buttons to reduce complexity
        // This eliminates potential flickering from button rendering
    }
}
