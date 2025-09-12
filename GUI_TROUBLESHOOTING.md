# KukiOS GUI Troubleshooting Guide

## Overview

This guide helps resolve common issues with the KukiOS graphical user interface, particularly the flickering problem you may encounter.

## Common Issues and Solutions

### 1. Screen Flickering

**Symptoms:**
- Desktop appears but flickers rapidly
- Windows and text are barely visible due to constant redrawing
- Display appears unstable or jittery

**Root Causes:**
- Continuous screen refreshing in the main GUI loop
- VGA Mode 13h direct framebuffer writes without proper timing
- Lack of double buffering or VSync synchronization
- Excessive rendering calls

**Solutions Applied:**

#### A. Static Rendering Mode
The GUI now uses a "render-once" approach instead of continuous redrawing:
```rust
// Instead of continuous loop redrawing:
while self.is_running {
    self.render();  // This caused flickering
}

// Now uses static mode:
self.render();  // Render once
loop {
    crate::sleep(100000000);  // Just wait, don't redraw
}
```

#### B. Simplified Graphics Operations
- Removed complex grid patterns that required many draw calls
- Simplified window rendering with fewer fill operations
- Eliminated unnecessary taskbar button rendering
- Reduced text rendering complexity

#### C. Increased Frame Delays
Extended sleep intervals to prevent rapid screen updates:
```rust
crate::sleep(100000000);  // 100ms delay instead of 10ms
```

### 2. VGA Mode Issues

**Symptoms:**
- GUI fails to initialize
- Black screen after selecting GUI mode
- Display corruption

**Solutions:**
1. **Check QEMU Configuration:**
   ```bash
   qemu-system-x86_64 -vga std -drive format=raw,file=bootimage.bin
   ```

2. **Verify VGA Mode 13h Support:**
   - Ensure emulator supports 320x200x256 color mode
   - Real hardware may need specific VGA cards

3. **Memory Address Verification:**
   - VGA framebuffer at 0xA0000 should be accessible
   - Check memory mapping in bootloader configuration

### 3. Performance Issues

**Symptoms:**
- Slow GUI responsiveness
- High CPU usage
- System appears frozen

**Current Mitigations:**
1. **Timeout-Based Exit:**
   - GUI automatically returns to CLI after ~30 seconds
   - Prevents infinite loops that consume resources

2. **Minimal Rendering:**
   - Only essential UI elements are drawn
   - Complex graphics operations avoided

3. **Extended Sleep Intervals:**
   - Longer delays between operations reduce CPU load

## Debugging Steps

### Step 1: Verify Basic GUI Loading
1. Boot KukiOS
2. Select 'Y' when prompted for GUI
3. Look for these messages:
   ```
   Starting KukiOS GUI...
   Initializing graphics subsystem...
   Starting desktop environment...
   GUI Mode Active - Press ESC key to return to CLI
   ```

### Step 2: Check VGA Initialization
If the screen goes black immediately:
1. VGA mode switch may have failed
2. Try running in QEMU with `-vga std`
3. Check for hardware compatibility

### Step 3: Address Flickering
If you see flickering:
1. The current build should have reduced this significantly
2. If still present, try these QEMU options:
   ```bash
   qemu-system-x86_64 -vga std -display gtk,gl=off
   ```

### Step 4: Monitor Resource Usage
In QEMU monitor (Ctrl+Alt+2):
```
info registers
info mem
```

## Current Limitations

### Known Issues:
1. **No Mouse Support:** GUI is display-only currently
2. **Limited Interaction:** No clicking or window manipulation
3. **Automatic Timeout:** GUI exits after 30 seconds
4. **Basic Graphics:** Simple shapes and text only
5. **No Dynamic Updates:** Static display without live content

### Planned Improvements:
1. **Proper Event Loop:** Replace timeout with keyboard input detection
2. **Double Buffering:** Implement off-screen rendering
3. **VSync Support:** Synchronize with display refresh
4. **Mouse Integration:** Add PS/2 mouse driver
5. **Interactive Elements:** Enable clicking and dragging

## Alternative Approaches

### If GUI Still Flickers:

#### Option 1: Use CLI Mode
- Select 'N' at startup prompt
- All KukiOS functionality available via command line
- Use `gui` command to get information about GUI mode

#### Option 2: QEMU Settings
Try different QEMU display options:
```bash
# Option A: Different VGA emulation
qemu-system-x86_64 -vga cirrus -drive format=raw,file=bootimage.bin

# Option B: Disable acceleration
qemu-system-x86_64 -vga std -no-accel -drive format=raw,file=bootimage.bin

# Option C: Use SDL display
qemu-system-x86_64 -vga std -display sdl -drive format=raw,file=bootimage.bin
```

#### Option 3: Build Configuration
Modify Cargo.toml to disable GUI features:
```toml
# Comment out GUI-related dependencies if needed
# embedded-graphics = "0.7.0"
```

## Code Structure for Debugging

### Key Files:
- `src/gui/mod.rs` - Main GUI loop and timing
- `src/gui/graphics.rs` - VGA driver and low-level operations  
- `src/gui/desktop.rs` - Window rendering and desktop management
- `src/startup_prompt.rs` - Interface selection logic

### Debug Points:
1. **VGA Initialization:** Check `VgaDisplay::init()` success
2. **Render Timing:** Monitor sleep intervals in main loop
3. **Memory Access:** Verify framebuffer writes at 0xA0000
4. **Mode Switching:** Confirm VGA registers set correctly

## Testing Recommendations

### Test Environment:
- **Primary:** QEMU with `-vga std`
- **Secondary:** QEMU with `-vga cirrus`  
- **Advanced:** Real hardware with compatible VGA

### Test Sequence:
1. Boot to interface prompt
2. Select GUI mode
3. Observe initialization messages
4. Wait for static display
5. Allow automatic timeout to CLI
6. Repeat with different QEMU settings if issues persist

## Getting Help

### Information to Provide:
1. **Host System:** OS and QEMU version
2. **Command Used:** Exact qemu-system-x86_64 command
3. **Symptoms:** Detailed description of visual issues
4. **Timing:** When issues occur (immediately, after delay, etc.)
5. **Error Messages:** Any console output or error messages

### Log Collection:
Run with serial output to capture debug information:
```bash
qemu-system-x86_64 -vga std -serial stdio -drive format=raw,file=bootimage.bin 2>&1 | tee kukios-debug.log
```

## Version History

### Current Version (v0.1.0):
- Basic GUI with static rendering
- Flickering mitigation implemented
- Automatic timeout for stability
- Simplified graphics to reduce complexity

### Future Versions:
- Dynamic rendering with proper timing
- Interactive GUI elements
- Hardware acceleration support
- Multi-resolution display modes