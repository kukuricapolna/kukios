//! VGA Graphics Driver for KukiOS GUI
//! Provides low-level VGA graphics mode support

use core::ptr;

// VGA mode 13h (320x200, 256 colors)
const VGA_WIDTH: usize = 320;
const VGA_HEIGHT: usize = 200;
const VGA_MEMORY: *mut u8 = 0xA0000 as *mut u8;

// VGA port addresses
const VGA_SEQUENCER_INDEX: u16 = 0x3C4;
const VGA_SEQUENCER_DATA: u16 = 0x3C5;
const VGA_CRT_INDEX: u16 = 0x3D4;
const VGA_CRT_DATA: u16 = 0x3D5;
const VGA_GRAPHICS_INDEX: u16 = 0x3CE;
const VGA_GRAPHICS_DATA: u16 = 0x3CF;
const VGA_ATTRIBUTE_INDEX: u16 = 0x3C0;
const VGA_ATTRIBUTE_DATA: u16 = 0x3C1;
const VGA_MISC_WRITE: u16 = 0x3C2;
const VGA_INPUT_STATUS: u16 = 0x3DA;

/// VGA Graphics Display Driver
pub struct VgaDisplay {
    framebuffer: *mut u8,
    palette: [u8; 768], // RGB palette, 256 colors * 3 bytes
}

impl VgaDisplay {
    pub fn new() -> Self {
        Self {
            framebuffer: VGA_MEMORY,
            palette: [0; 768],
        }
    }

    /// Initialize VGA mode 13h (320x200x256)
    pub fn init(&mut self) {
        unsafe {
            // Switch to VGA mode 13h
            self.set_mode_13h();
            self.init_palette();
        }
    }

    /// Set VGA mode 13h registers
    unsafe fn set_mode_13h(&self) {
        // Disable interrupts during mode switch
        x86_64::instructions::interrupts::disable();

        // Set misc register
        self.write_port(VGA_MISC_WRITE, 0x63);

        // Sequencer registers
        let seq_regs = [0x03, 0x01, 0x0F, 0x00, 0x0E];
        for (i, &reg) in seq_regs.iter().enumerate() {
            self.write_port(VGA_SEQUENCER_INDEX, i as u8);
            self.write_port(VGA_SEQUENCER_DATA, reg);
        }

        // CRT registers
        let crt_regs = [
            0x5F, 0x4F, 0x50, 0x82, 0x54, 0x80, 0xBF, 0x1F, 0x00, 0x41, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x9C, 0x0E, 0x8F, 0x28, 0x40, 0x96, 0xB9, 0xA3, 0xFF,
        ];

        // Unlock CRT registers
        self.write_port(VGA_CRT_INDEX, 0x03);
        self.write_port(VGA_CRT_DATA, self.read_port(VGA_CRT_DATA) | 0x80);
        self.write_port(VGA_CRT_INDEX, 0x11);
        self.write_port(VGA_CRT_DATA, self.read_port(VGA_CRT_DATA) & 0x7F);

        for (i, &reg) in crt_regs.iter().enumerate() {
            self.write_port(VGA_CRT_INDEX, i as u8);
            self.write_port(VGA_CRT_DATA, reg);
        }

        // Graphics registers
        let gfx_regs = [0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x05, 0x0F, 0xFF];
        for (i, &reg) in gfx_regs.iter().enumerate() {
            self.write_port(VGA_GRAPHICS_INDEX, i as u8);
            self.write_port(VGA_GRAPHICS_DATA, reg);
        }

        // Attribute registers
        let attr_regs = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F, 0x41, 0x00, 0x0F, 0x00, 0x00,
        ];

        self.read_port(VGA_INPUT_STATUS); // Reset attribute controller
        for (i, &reg) in attr_regs.iter().enumerate() {
            self.write_port(VGA_ATTRIBUTE_INDEX, i as u8);
            self.write_port(VGA_ATTRIBUTE_INDEX, reg);
        }
        self.write_port(VGA_ATTRIBUTE_INDEX, 0x20); // Enable palette

        // Re-enable interrupts
        x86_64::instructions::interrupts::enable();
    }

    /// Initialize a basic 256-color palette
    fn init_palette(&mut self) {
        unsafe {
            // Set up a basic palette
            for i in 0..256 {
                let r = ((i >> 5) & 0x07) * 255 / 7;
                let g = ((i >> 2) & 0x07) * 255 / 7;
                let b = (i & 0x03) * 255 / 3;

                self.palette[i * 3] = (r >> 2) as u8; // Red
                self.palette[i * 3 + 1] = (g >> 2) as u8; // Green
                self.palette[i * 3 + 2] = (b >> 2) as u8; // Blue
            }

            // Write palette to VGA
            self.write_port(0x3C8, 0); // Start at color 0
            for &color in &self.palette {
                self.write_port(0x3C9, color);
            }
        }
    }

    unsafe fn write_port(&self, port: u16, value: u8) {
        x86_64::instructions::port::Port::new(port).write(value);
    }

    unsafe fn read_port(&self, port: u16) -> u8 {
        x86_64::instructions::port::Port::new(port).read()
    }

    /// Clear the entire screen with a color
    pub fn clear(&mut self, color: Rgb888) {
        let color_index = self.rgb_to_palette_index(color);
        unsafe {
            ptr::write_bytes(self.framebuffer, color_index, VGA_WIDTH * VGA_HEIGHT);
        }
    }

    /// Set a pixel at the given coordinates
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Rgb888) {
        if x >= VGA_WIDTH as u32 || y >= VGA_HEIGHT as u32 {
            return;
        }

        let color_index = self.rgb_to_palette_index(color);
        let offset = (y as usize * VGA_WIDTH) + x as usize;

        unsafe {
            ptr::write_volatile(self.framebuffer.add(offset), color_index);
        }
    }

    /// Get pixel color at coordinates
    pub fn get_pixel(&self, x: u32, y: u32) -> u8 {
        if x >= VGA_WIDTH as u32 || y >= VGA_HEIGHT as u32 {
            return 0;
        }

        let offset = (y as usize * VGA_WIDTH) + x as usize;
        unsafe { ptr::read_volatile(self.framebuffer.add(offset)) }
    }

    /// Draw a filled rectangle
    pub fn fill_rect(&mut self, x: i32, y: i32, width: u32, height: u32, color: Rgb888) {
        let color_index = self.rgb_to_palette_index(color);

        for dy in 0..height {
            for dx in 0..width {
                let px = x + dx as i32;
                let py = y + dy as i32;

                if px >= 0 && px < VGA_WIDTH as i32 && py >= 0 && py < VGA_HEIGHT as i32 {
                    let offset = (py as usize * VGA_WIDTH) + px as usize;
                    unsafe {
                        ptr::write_volatile(self.framebuffer.add(offset), color_index);
                    }
                }
            }
        }
    }

    /// Draw a rectangle outline
    pub fn draw_rect(&mut self, x: i32, y: i32, width: u32, height: u32, color: Rgb888) {
        // Top edge
        self.draw_horizontal_line(x, y, width, color);
        // Bottom edge
        self.draw_horizontal_line(x, y + height as i32 - 1, width, color);
        // Left edge
        self.draw_vertical_line(x, y, height, color);
        // Right edge
        self.draw_vertical_line(x + width as i32 - 1, y, height, color);
    }

    /// Draw a horizontal line
    pub fn draw_horizontal_line(&mut self, x: i32, y: i32, width: u32, color: Rgb888) {
        let color_index = self.rgb_to_palette_index(color);

        for dx in 0..width {
            let px = x + dx as i32;
            if px >= 0 && px < VGA_WIDTH as i32 && y >= 0 && y < VGA_HEIGHT as i32 {
                let offset = (y as usize * VGA_WIDTH) + px as usize;
                unsafe {
                    ptr::write_volatile(self.framebuffer.add(offset), color_index);
                }
            }
        }
    }

    /// Draw a vertical line
    pub fn draw_vertical_line(&mut self, x: i32, y: i32, height: u32, color: Rgb888) {
        let color_index = self.rgb_to_palette_index(color);

        for dy in 0..height {
            let py = y + dy as i32;
            if x >= 0 && x < VGA_WIDTH as i32 && py >= 0 && py < VGA_HEIGHT as i32 {
                let offset = (py as usize * VGA_WIDTH) + x as usize;
                unsafe {
                    ptr::write_volatile(self.framebuffer.add(offset), color_index);
                }
            }
        }
    }

    /// Convert RGB color to nearest palette index
    fn rgb_to_palette_index(&self, color: Rgb888) -> u8 {
        // Simple mapping to 8-bit color palette
        let r = (color.r() >> 5) & 0x07; // 3 bits for red
        let g = (color.g() >> 5) & 0x07; // 3 bits for green
        let b = (color.b() >> 6) & 0x03; // 2 bits for blue

        (r << 5) | (g << 2) | b
    }

    /// Present/flush the current frame (no-op for direct framebuffer)
    pub fn present(&self) {
        // VGA mode 13h writes directly to video memory, so no presentation needed
        // This is here for API compatibility
    }

    /// Get screen dimensions
    pub fn dimensions(&self) -> (u32, u32) {
        (VGA_WIDTH as u32, VGA_HEIGHT as u32)
    }

    /// Reset VGA to text mode (mode 3) for CLI
    pub fn reset_to_text_mode(&self) {
        unsafe {
            // Disable interrupts during mode switch
            x86_64::instructions::interrupts::disable();

            // Reset to VGA text mode 3 (80x25 color text)
            self.write_port(VGA_MISC_WRITE, 0x67);

            // Sequencer registers for text mode
            let seq_regs = [0x03, 0x00, 0x03, 0x00, 0x02];
            for (i, &reg) in seq_regs.iter().enumerate() {
                self.write_port(VGA_SEQUENCER_INDEX, i as u8);
                self.write_port(VGA_SEQUENCER_DATA, reg);
            }

            // CRT registers for text mode
            let crt_regs = [
                0x5F, 0x4F, 0x50, 0x82, 0x55, 0x81, 0xBF, 0x1F, 0x00, 0x4F, 0x0D, 0x0E, 0x00, 0x00,
                0x00, 0x50, 0x9C, 0x0E, 0x8F, 0x28, 0x1F, 0x96, 0xB9, 0xA3, 0xFF,
            ];

            // Unlock CRT registers
            self.write_port(VGA_CRT_INDEX, 0x03);
            self.write_port(VGA_CRT_DATA, self.read_port(VGA_CRT_DATA) | 0x80);
            self.write_port(VGA_CRT_INDEX, 0x11);
            self.write_port(VGA_CRT_DATA, self.read_port(VGA_CRT_DATA) & 0x7F);

            for (i, &reg) in crt_regs.iter().enumerate() {
                self.write_port(VGA_CRT_INDEX, i as u8);
                self.write_port(VGA_CRT_DATA, reg);
            }

            // Graphics registers for text mode
            let gfx_regs = [0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x0E, 0x00, 0xFF];
            for (i, &reg) in gfx_regs.iter().enumerate() {
                self.write_port(VGA_GRAPHICS_INDEX, i as u8);
                self.write_port(VGA_GRAPHICS_DATA, reg);
            }

            // Attribute registers for text mode
            let attr_regs = [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x14, 0x07, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D,
                0x3E, 0x3F, 0x0C, 0x00, 0x0F, 0x08, 0x00,
            ];

            self.read_port(VGA_INPUT_STATUS); // Reset attribute controller
            for (i, &reg) in attr_regs.iter().enumerate() {
                self.write_port(VGA_ATTRIBUTE_INDEX, i as u8);
                self.write_port(VGA_ATTRIBUTE_INDEX, reg);
            }
            self.write_port(VGA_ATTRIBUTE_INDEX, 0x20); // Enable palette

            // Re-enable interrupts
            x86_64::instructions::interrupts::enable();
        }
    }

    /// Draw text with cursor support
    pub fn draw_text(&mut self, x: i32, y: i32, text: &str, color: Rgb888) {
        let color_index = self.rgb_to_palette_index(color);
        let mut cursor_x = x;
        let mut cursor_y = y;

        for ch in text.chars() {
            if ch == '\n' {
                cursor_x = x;
                cursor_y += 12; // Move to next line
                continue;
            }

            if cursor_x + 8 > VGA_WIDTH as i32 {
                cursor_x = x;
                cursor_y += 12; // Wrap to next line
            }

            if cursor_y + 8 > VGA_HEIGHT as i32 {
                break; // Would go off screen
            }

            self.draw_char(cursor_x, cursor_y, ch, color_index);
            cursor_x += 8;
        }
    }

    /// Draw text with visible cursor
    pub fn draw_text_with_cursor(
        &mut self,
        x: i32,
        y: i32,
        text: &str,
        color: Rgb888,
        cursor_pos: usize,
    ) {
        let color_index = self.rgb_to_palette_index(color);
        let cursor_color_index = self.rgb_to_palette_index(Rgb888::WHITE);
        let mut cursor_x = x;
        let mut cursor_y = y;
        let mut char_index = 0;

        for ch in text.chars() {
            if ch == '\n' {
                cursor_x = x;
                cursor_y += 12;
                char_index += 1;
                continue;
            }

            if cursor_x + 8 > VGA_WIDTH as i32 {
                cursor_x = x;
                cursor_y += 12;
            }

            if cursor_y + 8 > VGA_HEIGHT as i32 {
                break;
            }

            self.draw_char(cursor_x, cursor_y, ch, color_index);

            // Draw cursor if this is the cursor position
            if char_index == cursor_pos {
                self.draw_cursor(cursor_x + 8, cursor_y, cursor_color_index);
            }

            cursor_x += 8;
            char_index += 1;
        }

        // Draw cursor at end if cursor_pos is at end of text
        if char_index == cursor_pos {
            self.draw_cursor(cursor_x, cursor_y, cursor_color_index);
        }
    }

    /// Draw a blinking cursor
    fn draw_cursor(&mut self, x: i32, y: i32, color_index: u8) {
        // Simple vertical line cursor
        for dy in 0..8 {
            let px = x;
            let py = y + dy;
            if px >= 0 && px < VGA_WIDTH as i32 && py >= 0 && py < VGA_HEIGHT as i32 {
                let offset = (py as usize * VGA_WIDTH) + px as usize;
                unsafe {
                    ptr::write_volatile(self.framebuffer.add(offset), color_index);
                }
            }
        }
    }

    /// Draw a single character using a simple bitmap font
    fn draw_char(&mut self, x: i32, y: i32, ch: char, color_index: u8) {
        // Complete 8x8 bitmap font for ASCII characters
        let font_data = match ch {
            // Letters A-Z
            'A' => [0x18, 0x24, 0x42, 0x42, 0x7E, 0x42, 0x42, 0x00],
            'B' => [0x7C, 0x42, 0x42, 0x7C, 0x42, 0x42, 0x7C, 0x00],
            'C' => [0x3C, 0x42, 0x40, 0x40, 0x40, 0x42, 0x3C, 0x00],
            'D' => [0x78, 0x44, 0x42, 0x42, 0x42, 0x44, 0x78, 0x00],
            'E' => [0x7E, 0x40, 0x40, 0x7C, 0x40, 0x40, 0x7E, 0x00],
            'F' => [0x7E, 0x40, 0x40, 0x7C, 0x40, 0x40, 0x40, 0x00],
            'G' => [0x3C, 0x42, 0x40, 0x4E, 0x42, 0x42, 0x3C, 0x00],
            'H' => [0x42, 0x42, 0x42, 0x7E, 0x42, 0x42, 0x42, 0x00],
            'I' => [0x3E, 0x08, 0x08, 0x08, 0x08, 0x08, 0x3E, 0x00],
            'J' => [0x02, 0x02, 0x02, 0x02, 0x42, 0x42, 0x3C, 0x00],
            'K' => [0x44, 0x48, 0x50, 0x60, 0x50, 0x48, 0x44, 0x00],
            'L' => [0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x7E, 0x00],
            'M' => [0x42, 0x66, 0x5A, 0x42, 0x42, 0x42, 0x42, 0x00],
            'N' => [0x42, 0x62, 0x52, 0x4A, 0x46, 0x42, 0x42, 0x00],
            'O' => [0x3C, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3C, 0x00],
            'P' => [0x7C, 0x42, 0x42, 0x7C, 0x40, 0x40, 0x40, 0x00],
            'Q' => [0x3C, 0x42, 0x42, 0x42, 0x52, 0x4A, 0x3C, 0x00],
            'R' => [0x7C, 0x42, 0x42, 0x7C, 0x48, 0x44, 0x42, 0x00],
            'S' => [0x3C, 0x42, 0x40, 0x3C, 0x02, 0x42, 0x3C, 0x00],
            'T' => [0x7F, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x00],
            'U' => [0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3C, 0x00],
            'V' => [0x42, 0x42, 0x42, 0x42, 0x24, 0x18, 0x18, 0x00],
            'W' => [0x42, 0x42, 0x42, 0x42, 0x5A, 0x66, 0x42, 0x00],
            'X' => [0x42, 0x24, 0x18, 0x18, 0x24, 0x42, 0x42, 0x00],
            'Y' => [0x41, 0x22, 0x14, 0x08, 0x08, 0x08, 0x08, 0x00],
            'Z' => [0x7E, 0x02, 0x04, 0x08, 0x10, 0x20, 0x7E, 0x00],

            // Numbers 0-9
            '0' => [0x3C, 0x42, 0x46, 0x4A, 0x52, 0x62, 0x3C, 0x00],
            '1' => [0x08, 0x18, 0x08, 0x08, 0x08, 0x08, 0x1C, 0x00],
            '2' => [0x3C, 0x42, 0x02, 0x04, 0x18, 0x20, 0x7E, 0x00],
            '3' => [0x3C, 0x42, 0x02, 0x1C, 0x02, 0x42, 0x3C, 0x00],
            '4' => [0x04, 0x0C, 0x14, 0x24, 0x7E, 0x04, 0x04, 0x00],
            '5' => [0x7E, 0x40, 0x7C, 0x02, 0x02, 0x42, 0x3C, 0x00],
            '6' => [0x1C, 0x20, 0x40, 0x7C, 0x42, 0x42, 0x3C, 0x00],
            '7' => [0x7E, 0x02, 0x04, 0x08, 0x10, 0x10, 0x10, 0x00],
            '8' => [0x3C, 0x42, 0x42, 0x3C, 0x42, 0x42, 0x3C, 0x00],
            '9' => [0x3C, 0x42, 0x42, 0x3E, 0x02, 0x04, 0x38, 0x00],

            // Special characters
            ' ' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            '!' => [0x08, 0x08, 0x08, 0x08, 0x08, 0x00, 0x08, 0x00],
            '"' => [0x14, 0x14, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00],
            '#' => [0x14, 0x14, 0x3E, 0x14, 0x3E, 0x14, 0x14, 0x00],
            '$' => [0x08, 0x3E, 0x40, 0x3C, 0x02, 0x7C, 0x08, 0x00],
            '%' => [0x60, 0x92, 0x94, 0x08, 0x29, 0x49, 0x06, 0x00],
            '&' => [0x30, 0x48, 0x30, 0x56, 0x88, 0x89, 0x76, 0x00],
            '\'' => [0x08, 0x08, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00],
            '(' => [0x04, 0x08, 0x10, 0x10, 0x10, 0x08, 0x04, 0x00],
            ')' => [0x20, 0x10, 0x08, 0x08, 0x08, 0x10, 0x20, 0x00],
            '*' => [0x08, 0x49, 0x2A, 0x1C, 0x2A, 0x49, 0x08, 0x00],
            '+' => [0x00, 0x08, 0x08, 0x3E, 0x08, 0x08, 0x00, 0x00],
            ',' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x10, 0x00],
            '-' => [0x00, 0x00, 0x00, 0x3E, 0x00, 0x00, 0x00, 0x00],
            '.' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00],
            '/' => [0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x00],
            ':' => [0x00, 0x00, 0x08, 0x00, 0x00, 0x08, 0x00, 0x00],
            ';' => [0x00, 0x00, 0x08, 0x00, 0x00, 0x08, 0x10, 0x00],
            '<' => [0x04, 0x08, 0x10, 0x20, 0x10, 0x08, 0x04, 0x00],
            '=' => [0x00, 0x00, 0x3E, 0x00, 0x3E, 0x00, 0x00, 0x00],
            '>' => [0x20, 0x10, 0x08, 0x04, 0x08, 0x10, 0x20, 0x00],
            '?' => [0x3C, 0x42, 0x04, 0x08, 0x08, 0x00, 0x08, 0x00],
            '@' => [0x3C, 0x42, 0x9A, 0xAA, 0x9E, 0x40, 0x3C, 0x00],
            '[' => [0x1C, 0x10, 0x10, 0x10, 0x10, 0x10, 0x1C, 0x00],
            '\\' => [0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01, 0x00],
            ']' => [0x38, 0x08, 0x08, 0x08, 0x08, 0x08, 0x38, 0x00],
            '^' => [0x08, 0x14, 0x22, 0x00, 0x00, 0x00, 0x00, 0x00],
            '_' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7F, 0x00],
            '`' => [0x10, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            '{' => [0x0C, 0x10, 0x10, 0x20, 0x10, 0x10, 0x0C, 0x00],
            '|' => [0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x00],
            '}' => [0x30, 0x08, 0x08, 0x04, 0x08, 0x08, 0x30, 0x00],
            '~' => [0x00, 0x00, 0x31, 0x4E, 0x00, 0x00, 0x00, 0x00],

            // Lowercase letters
            'a' => [0x00, 0x00, 0x3C, 0x02, 0x3E, 0x42, 0x3E, 0x00],
            'b' => [0x40, 0x40, 0x5C, 0x62, 0x42, 0x62, 0x5C, 0x00],
            'c' => [0x00, 0x00, 0x3C, 0x40, 0x40, 0x42, 0x3C, 0x00],
            'd' => [0x02, 0x02, 0x3A, 0x46, 0x42, 0x46, 0x3A, 0x00],
            'e' => [0x00, 0x00, 0x3C, 0x42, 0x7E, 0x40, 0x3C, 0x00],
            'f' => [0x0C, 0x10, 0x10, 0x7C, 0x10, 0x10, 0x10, 0x00],
            'g' => [0x00, 0x00, 0x3A, 0x46, 0x46, 0x3A, 0x02, 0x3C],
            'h' => [0x40, 0x40, 0x5C, 0x62, 0x42, 0x42, 0x42, 0x00],
            'i' => [0x08, 0x00, 0x18, 0x08, 0x08, 0x08, 0x1C, 0x00],
            'j' => [0x04, 0x00, 0x0C, 0x04, 0x04, 0x04, 0x44, 0x38],
            'k' => [0x20, 0x20, 0x24, 0x28, 0x30, 0x28, 0x24, 0x00],
            'l' => [0x18, 0x08, 0x08, 0x08, 0x08, 0x08, 0x1C, 0x00],
            'm' => [0x00, 0x00, 0x76, 0x49, 0x49, 0x49, 0x49, 0x00],
            'n' => [0x00, 0x00, 0x5C, 0x62, 0x42, 0x42, 0x42, 0x00],
            'o' => [0x00, 0x00, 0x3C, 0x42, 0x42, 0x42, 0x3C, 0x00],
            'p' => [0x00, 0x00, 0x5C, 0x62, 0x62, 0x5C, 0x40, 0x40],
            'q' => [0x00, 0x00, 0x3A, 0x46, 0x46, 0x3A, 0x02, 0x02],
            'r' => [0x00, 0x00, 0x5C, 0x62, 0x40, 0x40, 0x40, 0x00],
            's' => [0x00, 0x00, 0x3E, 0x40, 0x3C, 0x02, 0x7C, 0x00],
            't' => [0x10, 0x10, 0x7C, 0x10, 0x10, 0x12, 0x0C, 0x00],
            'u' => [0x00, 0x00, 0x42, 0x42, 0x42, 0x46, 0x3A, 0x00],
            'v' => [0x00, 0x00, 0x42, 0x42, 0x42, 0x24, 0x18, 0x00],
            'w' => [0x00, 0x00, 0x41, 0x49, 0x49, 0x49, 0x36, 0x00],
            'x' => [0x00, 0x00, 0x42, 0x24, 0x18, 0x24, 0x42, 0x00],
            'y' => [0x00, 0x00, 0x42, 0x42, 0x46, 0x3A, 0x02, 0x3C],
            'z' => [0x00, 0x00, 0x7E, 0x04, 0x18, 0x20, 0x7E, 0x00],

            _ => [0x7E, 0x81, 0xA5, 0x81, 0xBD, 0x99, 0x81, 0x7E], // Question mark box for unknown characters
        };

        // Draw the 8x8 character
        for (row, &byte) in font_data.iter().enumerate() {
            for col in 0..8 {
                if (byte >> (7 - col)) & 1 != 0 {
                    let px = x + col;
                    let py = y + row as i32;

                    if px >= 0 && px < VGA_WIDTH as i32 && py >= 0 && py < VGA_HEIGHT as i32 {
                        let offset = (py as usize * VGA_WIDTH) + px as usize;
                        unsafe {
                            ptr::write_volatile(self.framebuffer.add(offset), color_index);
                        }
                    }
                }
            }
        }
    }
}

// Simple RGB888 color type
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb888 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb888 {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const BLACK: Rgb888 = Rgb888::new(0, 0, 0);
    pub const WHITE: Rgb888 = Rgb888::new(255, 255, 255);
    pub const RED: Rgb888 = Rgb888::new(255, 0, 0);
    pub const GREEN: Rgb888 = Rgb888::new(0, 255, 0);
    pub const BLUE: Rgb888 = Rgb888::new(0, 0, 255);
    pub const YELLOW: Rgb888 = Rgb888::new(255, 255, 0);

    pub fn r(&self) -> u8 {
        self.r
    }
    pub fn g(&self) -> u8 {
        self.g
    }
    pub fn b(&self) -> u8 {
        self.b
    }
}
