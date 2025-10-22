#!/bin/bash

# KukiOS Run Script
# Easy way to build and run KukiOS with GUI support

set -e

echo "=================================="
echo "     KukiOS Build & Run Script"
echo "=================================="
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Please run this script from the KukiOS root directory"
    exit 1
fi

# Parse command line arguments
GUI_MODE="ask"
CLEAN_BUILD=false
RELEASE_MODE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --gui)
            GUI_MODE="force_gui"
            shift
            ;;
        --cli)
            GUI_MODE="force_cli"
            shift
            ;;
        --clean)
            CLEAN_BUILD=true
            shift
            ;;
        --release)
            RELEASE_MODE=true
            shift
            ;;
        --help|-h)
            echo "KukiOS Run Script Usage:"
            echo "  ./run_kukios.sh [OPTIONS]"
            echo
            echo "OPTIONS:"
            echo "  --gui       Force GUI mode (skip startup prompt)"
            echo "  --cli       Force CLI mode (skip startup prompt)"
            echo "  --clean     Clean build before running"
            echo "  --release   Build in release mode"
            echo "  --help, -h  Show this help message"
            echo
            echo "Examples:"
            echo "  ./run_kukios.sh                 # Normal build and run with startup prompt"
            echo "  ./run_kukios.sh --gui           # Run directly in GUI mode"
            echo "  ./run_kukios.sh --cli --clean   # Clean build and run in CLI mode"
            echo "  ./run_kukios.sh --release       # Release build"
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            print_status "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Clean build if requested
if [ "$CLEAN_BUILD" = true ]; then
    print_status "Cleaning previous build artifacts..."
    cargo clean
    print_success "Clean completed"
fi

# Build configuration
BUILD_FLAGS="--target x86_64-kukios.json"
if [ "$RELEASE_MODE" = true ]; then
    BUILD_FLAGS="$BUILD_FLAGS --release"
    print_status "Building KukiOS in release mode..."
else
    print_status "Building KukiOS in debug mode..."
fi

# Build the OS
print_status "Compiling KukiOS..."
if cargo build $BUILD_FLAGS; then
    print_success "Build completed successfully"
else
    print_error "Build failed"
    exit 1
fi

# Check for bootimage tool
if ! command -v bootimage &> /dev/null; then
    print_warning "bootimage tool not found, installing..."
    cargo install bootimage
fi

# Create bootable image
print_status "Creating bootable image..."
if cargo bootimage $BUILD_FLAGS; then
    print_success "Bootimage created successfully"
else
    print_error "Failed to create bootimage"
    exit 1
fi

# Determine QEMU options based on GUI mode
QEMU_OPTS="-serial stdio"

case $GUI_MODE in
    force_gui)
        print_status "Forcing GUI mode (VGA display enabled)"
        QEMU_OPTS="$QEMU_OPTS -vga std"
        ;;
    force_cli)
        print_status "Forcing CLI mode (display disabled for text focus)"
        QEMU_OPTS="$QEMU_OPTS -display none"
        ;;
    ask)
        print_status "Normal boot with startup prompt"
        QEMU_OPTS="$QEMU_OPTS -vga std"
        ;;
esac

# Additional QEMU options for better experience
QEMU_OPTS="$QEMU_OPTS -m 128M"  # 128MB RAM
QEMU_OPTS="$QEMU_OPTS -cpu qemu64"  # Standard CPU
QEMU_OPTS="$QEMU_OPTS -no-reboot"  # Exit on reboot
QEMU_OPTS="$QEMU_OPTS -no-shutdown"  # Exit on shutdown
QEMU_OPTS="$QEMU_OPTS -usb -device usb-mouse"  # Register touchpad as mouse input

print_status "Starting KukiOS in QEMU..."
echo
print_success "KukiOS is now running!"
print_status "GUI Features:"
echo "  - At startup, you'll see an interface selection prompt"
echo "  - Choose 'Y' for GUI mode or 'N' for CLI mode"
echo "  - GUI mode includes desktop, windows, and applications"
echo "  - CLI mode provides the traditional command-line interface"
echo
print_status "Controls:"
echo "  - Ctrl+C to exit QEMU and return to host system"
echo "  - In GUI mode: Click windows to interact (mouse support basic)"
echo "  - In CLI mode: Type 'help' to see available commands"
echo
print_warning "Note: This is a development version of KukiOS"
echo

# Run in QEMU
if [ "$RELEASE_MODE" = true ]; then
    BOOTIMAGE_PATH="target/x86_64-kukios/release/bootimage-kukios.bin"
else
    BOOTIMAGE_PATH="target/x86_64-kukios/debug/bootimage-kukios.bin"
fi

if [ -f "$BOOTIMAGE_PATH" ]; then
    exec qemu-system-x86_64 -drive format=raw,file="$BOOTIMAGE_PATH" $QEMU_OPTS
else
    print_error "Bootimage not found at $BOOTIMAGE_PATH"
    exit 1
fi
