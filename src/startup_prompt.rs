use crate::println;
use crate::vga_buffer::{Color, WRITER};
use kukios::{
    interrupts::{input, Helper},
    sleep,
};

pub fn ask_for_gui() -> bool {
    clear_screen();
    show_banner();
    show_gui_prompt();
    loop {
        println!("Enter your choice (y/n): ");
        let input = input(Helper::Empty).trim().to_lowercase();

        match input.as_str() {
            "y" | "yes" | "gui" => {
                println!("Starting KukiOS GUI...");
                return true;
            }
            "n" | "no" | "cli" | "terminal" => {
                println!("Continuing in CLI mode...");
                return false;
            }
            _ => {
                println!("Invalid input. Please enter 'y' for GUI or 'n' for CLI.");
                continue;
            }
        }
    }
}

/// Clear the screen
fn clear_screen() {
    for _ in 0..50 {
        println!();
    }
}

/// Show the KukiOS startup banner
fn show_banner() {
    // Show welcome message
    WRITER.lock().change_fg_bg(Color::Cyan, Color::Black);
    println!("                        Welcome to KukiOS - A Rust-based Operating System!");
    println!("                              Kukiweb + Intelligence = KukiOS!");
    println!();

    // Reset colors
    WRITER.lock().change_fg_bg(Color::White, Color::Black);

    // Show system info
    println!("System Information:");
    println!("  Version: 0.1.0");
    println!("  Architecture: x86_64");
    println!("  Language: Rust");
    println!("  Build: Development-1.0");
    println!();
}

/// Show the GUI selection prompt
fn show_gui_prompt() {
    WRITER.lock().change_fg_bg(Color::LightBlue, Color::Black);
    println!("INTERFACE SELECTION");
    WRITER.lock().change_fg_bg(Color::White, Color::Black);
    println!("KukiOS supports both graphical and command-line interfaces.");
    println!("Available options:");
    WRITER.lock().change_fg_bg(Color::Green, Color::Black);
    println!("[Y] GUI Mode  - Graphical user interface with windows, desktop, and mouse support [IN UNSTABLE DEVELOPMENT]");
    WRITER.lock().change_fg_bg(Color::Yellow, Color::Black);
    println!(
        "[N] CLI Mode  - Command-line interface (current default mode) [IN STABLE DEVELOPMENT]"
    );
    WRITER.lock().change_fg_bg(Color::White, Color::Black);
    println!(
        "NOTE: To switch interfaces, restart the system using 'shutdown' in CLI or 'exit' in GUI"
    );
    WRITER.lock().change_fg_bg(Color::LightRed, Color::Black);
    println!("Do you want to start with the Graphical User Interface (GUI)?");
    WRITER.lock().change_fg_bg(Color::White, Color::Black);
}

/// Show a loading animation for GUI startup
pub fn show_gui_loading() {
    println!();
    println!("Initializing GUI components...");

    let loading_steps = [
        "Loading VGA graphics driver....",
        "Setting up display modes....",
        "Initializing desktop environment....",
        "Loading GUI widgets....",
        "Preparing window manager....",
        "Starting GUI subsystem....",
        "Welcome, KukiOS user!",
    ];

    for (i, step) in loading_steps.iter().enumerate() {
        println!("[{}/{}] {}", i + 1, loading_steps.len(), step);

        // Simple loading animation
        for j in 0..3 {
            sleep(25000000); // Short delay
            match j {
                0 => println!("."),
                1 => println!(".."),
                2 => println!(".. Done!"),
                _ => {}
            }
        }
    }

    println!();
    WRITER.lock().change_fg_bg(Color::Green, Color::Black);
    println!("[OK] GUI initialization complete!");
    WRITER.lock().change_fg_bg(Color::White, Color::Black);
    println!();
}

pub fn show_cli_continuation() {
    println!();
    WRITER.lock().change_fg_bg(Color::Green, Color::Black);
    println!("[STATUS] Continuing in CLI mode...");
    WRITER.lock().change_fg_bg(Color::White, Color::Black);
    println!();
    println!("Tip: Type 'help' to see available commands");
    println!();
}
