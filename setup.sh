#!/bin/bash

# Gemma Translator RS - Setup Script
# Automates the installation process for different platforms

set -e  # Exit on any error

echo "🚀 Gemma Translator RS Setup"
echo "============================="

# Detect OS
OS=$(uname -s)
ARCH=$(uname -m)

echo "Detected: $OS $ARCH"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source ~/.cargo/env
    echo "✅ Rust installed successfully"
else
    echo "✅ Rust found: $(rustc --version)"
fi

# Platform-specific dependencies
case $OS in
    "Darwin")  # macOS
        echo "🍎 Setting up for macOS..."
        if ! command -v brew &> /dev/null; then
            echo "❌ Homebrew not found. Please install Homebrew first:"
            echo "   /bin/bash -c \"\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""
            exit 1
        fi
        brew install pkg-config
        ;;
    "Linux")   # Linux
        echo "🐧 Setting up for Linux..."
        
        # Check if we're on a Raspberry Pi
        if [[ $ARCH == "aarch64" ]] && grep -q "Raspberry Pi" /proc/device-tree/model 2>/dev/null; then
            echo "🥧 Raspberry Pi detected - this may take 30-60 minutes..."
            
            # Increase swap for Pi
            if [[ $(cat /proc/swaps | wc -l) -le 1 ]]; then
                echo "📈 Increasing swap space for compilation..."
                sudo dphys-swapfile swapoff || true
                sudo sed -i 's/CONF_SWAPSIZE=.*/CONF_SWAPSIZE=2048/' /etc/dphys-swapfile 2>/dev/null || true
                sudo dphys-swapfile setup || true
                sudo dphys-swapfile swapon || true
            fi
        fi
        
        # Install dependencies based on package manager
        if command -v apt &> /dev/null; then
            sudo apt update
            sudo apt install -y build-essential pkg-config libasound2-dev libssl-dev cmake git
        elif command -v yum &> /dev/null; then
            sudo yum groupinstall -y "Development Tools"
            sudo yum install -y pkg-config alsa-lib-devel openssl-devel cmake git
        elif command -v pacman &> /dev/null; then
            sudo pacman -S --needed base-devel pkg-config alsa-lib openssl cmake git
        else
            echo "❌ Unsupported package manager. Please install dependencies manually:"
            echo "   - build-essential/base-devel"
            echo "   - pkg-config"
            echo "   - alsa development headers"
            echo "   - openssl development headers"
            echo "   - cmake"
            exit 1
        fi
        ;;
    *)
        echo "❌ Unsupported operating system: $OS"
        exit 1
        ;;
esac

echo "📦 Building Gemma Translator RS..."

# Set build flags for optimization
if [[ $OS == "Linux" && $ARCH == "aarch64" ]]; then
    export RUSTFLAGS="-C target-cpu=native"
    echo "🔧 Using native CPU optimizations for ARM64"
fi

# Build with all features
echo "⚡ Building with all features (this may take a while)..."
cargo build --release --features "ui,realtime"

echo ""
echo "✅ Build completed successfully!"
echo ""
echo "📁 Binary location: ./target/release/gemma-edge-translator"
echo ""
echo "🎯 Next steps:"
echo "1. Download a Gemma model (GGUF format):"
echo "   mkdir -p models"
echo "   # Download from Hugging Face (example URLs in README)"
echo ""
echo "2. Set your OpenAI API key:"
echo "   export OPENAI_API_KEY=\"your-key-here\""
echo ""
echo "3. Test the installation:"
echo "   ./target/release/gemma-edge-translator --help"
echo ""
echo "🌟 For detailed usage instructions, see the README.md file"
echo ""
echo "Happy translating! 🎉"
