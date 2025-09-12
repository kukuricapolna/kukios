# KukiOS GUI Documentation

## Overview

KukiOS now includes a graphical user interface (GUI) alongside its traditional command-line interface (CLI). This document explains how to use the new GUI features and how to switch between interfaces.

## Startup Options

When KukiOS boots, you will be presented with an interface selection prompt:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           INTERFACE SELECTION                          │
└─────────────────────────────────────────────────────────────────────────┘

KukiOS supports both graphical and command-line interfaces.

Available options:
  [Y] GUI Mode  - Graphical user interface with windows, desktop, and mouse support
  [N] CLI Mode  - Command-line interface (current default mode)

Do you want to start with the Graphical User Interface (GUI)?
Enter your choice (y/n):
```

### GUI Mode (Y/Yes)
- Starts the graphical desktop environment
- Features windows, taskbar, and visual applications
- Uses VGA Mode 13h (320x200, 256 colors)
- Includes a desktop with grid background and window management

### CLI Mode (N/No)
- Continues with the traditional command-line interface
- All existing KukiOS commands remain available
- Text-based interaction through terminal

## GUI Features

### Desktop Environment
- **Desktop Background**: Grid pattern with system information
- **Window Management**: Multiple windows with title bars, borders, and close buttons
- **Taskbar**: Bottom panel showing open applications and Start menu
- **System Info**: Display of KukiOS version in top-right corner

### Available Applications

#### Terminal Window
- Access to KukiOS command-line interface within the GUI
- Type commands just like in CLI mode
- Displays command prompt and help text

#### File Manager
- Browse filesystem contents
- View files in root directory
- Simple file listing interface

#### About Dialog
- System information display
- Shows KukiOS version and credits
- Information about the operating system

### GUI Controls

#### Windows
- **Move**: Click and drag title bar (not yet implemented)
- **Close**: Click the 'X' button in top-right corner
- **Activate**: Click anywhere on window to bring to front
- **Minimize/Maximize**: Planned for future versions

#### Taskbar
- **Application Buttons**: Click to switch between open applications
- **Start Menu**: Access to system functions (planned)

## Technical Specifications

### Graphics Mode
- **Resolution**: 320x200 pixels
- **Color Depth**: 8-bit (256 colors)
- **VGA Mode**: Mode 13h
- **Memory**: Direct framebuffer access at 0xA0000

### Color Palette
- **GUI_BACKGROUND**: Dark blue-gray (0x20, 0x20, 0x30)
- **GUI_FOREGROUND**: White (0xFF, 0xFF, 0xFF)
- **GUI_ACCENT**: Bright blue (0x40, 0x80, 0xFF)
- **GUI_SUCCESS**: Green (0x00, 0xFF, 0x00)
- **GUI_WARNING**: Yellow (0xFF, 0xFF, 0x00)
- **GUI_ERROR**: Red (0xFF, 0x00, 0x00)

### Font System
- **Type**: Bitmap font
- **Size**: 8x8 pixels per character
- **Characters**: Basic ASCII support
- **Style**: Monospace

## Architecture

### Module Structure
```
src/gui/
├── mod.rs          # Main GUI module and types
├── graphics.rs     # VGA graphics driver
├── desktop.rs      # Desktop environment and window management
└── widgets.rs      # UI components (buttons, dialogs, etc.)
```

### Key Components

#### VgaDisplay
- Low-level VGA graphics driver
- Pixel manipulation and drawing primitives
- Text rendering with bitmap font
- Color palette management

#### Desktop
- Window management system
- Application lifecycle
- Desktop background rendering
- Taskbar management

#### Widgets
- Reusable UI components
- Buttons, labels, dialogs
- Progress bars and checkboxes
- Event handling system

## CLI Commands for GUI

Even in CLI mode, you can access GUI-related commands:

### `gui` or `startgui`
Attempts to switch to GUI mode (currently shows information about restarting)

```bash
$ gui
Switching to GUI mode...
Note: GUI switching from CLI is not yet implemented.
Please restart the system to use GUI mode.
At startup, choose 'Y' when asked about GUI interface.
```

## Building and Running

The GUI is built automatically with KukiOS. No additional dependencies are required beyond the existing build system.

### Build Command
```bash
cargo build --target x86_64-kukios.json
```

### Run in QEMU
```bash
cargo run --target x86_64-kukios.json
```

## Future Enhancements

### Planned Features
- **Mouse Support**: PS/2 mouse integration for clicking and dragging
- **Window Dragging**: Move windows by dragging title bars
- **Menu System**: Start menu and right-click context menus
- **More Applications**: Text editor, calculator, games
- **Themes**: Customizable color schemes and appearance
- **Resolution Options**: Support for higher resolutions
- **Font Options**: Multiple font sizes and styles

### Possible Applications
- **Text Editor**: Full-featured text editing with syntax highlighting
- **Calculator**: Basic arithmetic operations
- **Games**: Simple games like Snake or Tetris
- **System Monitor**: Display CPU usage, memory, and system stats
- **File Browser**: Advanced file management with icons
- **Settings Panel**: System configuration interface

## Development

### Adding New Applications
To add a new application to the GUI:

1. Define the application content in `desktop.rs`:
```rust
pub enum WindowContent {
    Terminal,
    FileManager,
    About,
    YourNewApp,  // Add your application here
}
```

2. Implement rendering logic in the `render_content` method
3. Add application creation in `Desktop::init()`

### Creating Custom Widgets
New widgets can be added in `widgets.rs`:

1. Define the widget struct
2. Implement the `render` method
3. Add event handling if needed
4. Export the widget from the module

### Graphics Programming
Low-level graphics operations are available through `VgaDisplay`:

- `set_pixel()`: Set individual pixels
- `fill_rect()`: Draw filled rectangles
- `draw_rect()`: Draw rectangle outlines
- `draw_text()`: Render text with bitmap font
- `clear()`: Clear screen with solid color

## Troubleshooting

### Common Issues

#### GUI Won't Start
- Ensure you selected 'Y' at the startup prompt
- Check that VGA mode initialization succeeded
- Verify no compilation errors in GUI modules

#### Display Problems
- GUI uses VGA Mode 13h which requires compatible hardware/emulation
- QEMU should work correctly with default settings
- Real hardware may require specific VGA cards

#### Performance Issues
- GUI rendering is CPU-intensive without acceleration
- Consider reducing update frequency for better performance
- Complex graphics operations may cause slowdowns

### Debug Information
The system provides debug output during GUI initialization:
- VGA driver loading status
- Display mode setup progress
- Desktop environment initialization
- Component loading confirmations

## Contributing

To contribute to the GUI system:

1. Follow the existing code style and patterns
2. Add documentation for new features
3. Test thoroughly in QEMU before submitting
4. Consider backward compatibility with CLI mode
5. Update this documentation for significant changes

## Version History

### v0.1.0 (Current)
- Initial GUI implementation
- Basic window management
- VGA Mode 13h graphics driver
- Desktop environment with taskbar
- Terminal, File Manager, and About applications
- Startup prompt for interface selection

---

For questions or issues with the GUI system, refer to the main KukiOS documentation or examine the source code in the `src/gui/` directory.