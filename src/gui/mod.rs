//! GUI Module for KukiOS
//! Provides graphical user interface functionality

use crate::gui::graphics::Rgb888;
use crate::interrupts::{input, Helper};
use crate::{print, println};
use alloc::string::{String, ToString};

pub mod desktop;
pub mod graphics;
pub mod widgets;

pub use desktop::*;
pub use graphics::*;
pub use widgets::*;

/// Represents a GUI cursor (mouse-like) for desktop interaction
#[derive(Debug, Clone)]
pub struct GuiCursor {
    pub position: Point,
    pub is_visible: bool,
}

impl GuiCursor {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            position: Point::new(x, y),
            is_visible: true,
        }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.position.x = (self.position.x + dx).clamp(0, SCREEN_WIDTH as i32 - 1);
        self.position.y = (self.position.y + dy).clamp(0, SCREEN_HEIGHT as i32 - 1);
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.position.x = x.clamp(0, SCREEN_WIDTH as i32 - 1);
        self.position.y = y.clamp(0, SCREEN_HEIGHT as i32 - 1);
    }
}

// Standard colors for the GUI
pub const GUI_BACKGROUND: Rgb888 = Rgb888::new(0x20, 0x20, 0x30);
pub const GUI_FOREGROUND: Rgb888 = Rgb888::WHITE;
pub const GUI_ACCENT: Rgb888 = Rgb888::new(0x40, 0x80, 0xFF);
pub const GUI_SUCCESS: Rgb888 = Rgb888::GREEN;
pub const GUI_WARNING: Rgb888 = Rgb888::YELLOW;
pub const GUI_ERROR: Rgb888 = Rgb888::RED;

// Screen dimensions for VGA mode
pub const SCREEN_WIDTH: u32 = 320;
pub const SCREEN_HEIGHT: u32 = 200;

/// Represents a point in 2D space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Represents a rectangular area
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn contains_point(&self, point: Point) -> bool {
        point.x >= self.x
            && point.x < self.x + self.width as i32
            && point.y >= self.y
            && point.y < self.y + self.height as i32
    }
}

/// Main GUI context that manages the graphical interface
pub struct GuiContext {
    pub display: VgaDisplay,
    pub desktop: Desktop,
    pub is_running: bool,
}

impl GuiContext {
    pub fn new() -> Self {
        let mut display = VgaDisplay::new();
        display.init();

        Self {
            display,
            desktop: Desktop::new(),
            is_running: true,
        }
    }

    pub fn run(&mut self) {
        // Clear screen with background color
        self.display.clear(GUI_BACKGROUND);

        // Initialize desktop
        self.desktop.init(&mut self.display);

        // Render once and then enter interactive mode
        self.render();

        // Interactive mode with keyboard input
        println!("GUI Mode Active - Terminal windows are now interactive!");
        println!("Type in the terminal window below:");
        println!("Commands: 'help', 'clear', 'about'");
        println!("To exit GUI: Type 'exit' in console or GUI terminal");

        let mut frame_counter = 0;
        let mut terminal_input = String::new();

        // Main GUI event loop with simplified input and cursor control
        loop {
            // Update cursor blinking every 60 frames (slower)
            frame_counter += 1;
            if frame_counter >= 60 {
                self.desktop.update_terminal_cursors();
                self.render(); // Re-render to show cursor blink
                frame_counter = 0;
            }

            // Get simple character input for terminal or cursor
            print!("> ");
            let user_input = input(Helper::Is("GUI-Terminal".to_string()));

            if !user_input.trim().is_empty() {
                match user_input.trim() {
                    "exit" | "quit" => {
                        self.is_running = false;
                        break;
                    }
                    // WASD for cursor movement, Space for click
                    "w" | "a" | "s" | "d" | "" => {
                        for ch in user_input.chars() {
                            self.desktop.handle_keyboard_input(ch);
                            self.render();
                        }
                    }
                    " " => {
                        self.desktop.handle_keyboard_input(' ');
                        self.render();
                    }
                    _ => {
                        // Send input to active terminal window
                        for ch in user_input.chars() {
                            self.desktop.handle_keyboard_input(ch);
                            self.render();
                        }
                        // Execute the command (Enter confirms command)
                        self.desktop.handle_keyboard_input('\n');
                        self.render(); // Re-render after terminal update
                        println!("Command sent to terminal: {}", user_input.trim());
                    }
                }
            }

            // Improved refresh rate: render every loop iteration
            self.render();

            // Small delay to prevent excessive CPU usage (tweak as needed)
            crate::sleep(10000);
        }

        // Reset VGA to text mode before returning to CLI
        self.display.reset_to_text_mode();

        // Re-enable cursor for CLI
        use crate::vga_buffer::WRITER;
        WRITER.lock().enable_cursor();

        // Return to CLI mode
        println!("Exiting GUI mode, returning to CLI...");
        println!("You are now back in CLI mode. Type commands normally.");
    }

    /// Stub for mouse input integration
    /// Call this from your event loop when mouse events are available
    pub fn update_cursor_from_mouse(&mut self, mouse_x: i32, mouse_y: i32) {
        self.desktop.cursor.set_position(mouse_x, mouse_y);
        self.render();
    }

    /// Call this to handle a mouse click at the current cursor position
    pub fn handle_mouse_click(&mut self) {
        self.desktop.handle_cursor_click();
        self.render();
    }

    fn update(&mut self) {
        // Handle input events here (keyboard, mouse)
        // For now, we'll add a simple escape to exit
        // This would be expanded with proper input handling
    }

    fn show_help(&self) {
        println!();
        println!("=== KukiOS GUI Help ===");
        println!("Available commands:");
        println!("  help    - Show this help message");
        println!("  about   - Show system information");
        println!("  clear   - Clear and redraw the screen");
        println!("  refresh - Refresh the display");
        println!("  exit    - Return to CLI mode");
        println!("  quit    - Return to CLI mode");
        println!("  cli     - Return to CLI mode");
        println!();
    }

    fn show_about(&self) {
        println!();
        println!("=== About KukiOS GUI ===");
        println!("KukiOS Graphical User Interface v0.1.0");
        println!("A Rust-based operating system with GUI support");
        println!("Resolution: 320x200, 256 colors (VGA Mode 13h)");
        println!("Created by Kukiweb.cz");
        println!();
    }

    fn clear_and_redraw(&mut self) {
        // Clear screen and redraw GUI
        self.display.clear(GUI_BACKGROUND);
        self.render();
        println!("GUI display refreshed.");
    }

    fn render(&mut self) {
        // Only render if something has changed (for now, render less frequently)
        // Clear screen first to prevent artifacts
        self.display.clear(GUI_BACKGROUND);

        // Render desktop and all windows
        self.desktop.render(&mut self.display);

        // Present the frame
        self.display.present();
    }

    pub fn shutdown(&mut self) {
        self.is_running = false;
    }
}

/// Initialize GUI system
pub fn init_gui() -> GuiContext {
    GuiContext::new()
}
