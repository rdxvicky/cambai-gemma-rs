# Gemma Translator RS

🎙️ **Real-time Speech Translation** powered by **Whisper + Gemma**

A high-performance Rust application that combines OpenAI's Whisper for speech recognition with Google's Gemma language model for translation. Designed to run efficiently on everything from Raspberry Pi to Apple Silicon.

## ✨ Features

- **Speech-to-Text**: Uses Whisper API (OpenAI or local) for accurate transcription
- **Smart Translation**: Powered by Gemma-2B-IT model via llama.cpp
- **Real-time Capture**: Live microphone recording and processing
- **Web UI**: Optional local web interface with real-time performance monitoring
- **Performance Tracking**: Application-specific CPU and memory usage with peak detection
- **Cross-Platform**: Supports macOS, Linux, and Raspberry Pi
- **Efficient**: Optimized for resource-constrained devices

## 🚀 Quick Start

### Prerequisites

- **Rust** (1.75 or later)
- **OpenAI API Key** (or local Whisper setup)
- **Gemma Model** (GGUF format)

### Installation

```bash
# Clone the repository
git clone https://github.com/rdxvicky/gemma-edge-translator.git
cd gemma-edge-translator

# Build with all features
cargo build --release --features "ui,realtime"
```

### Basic Usage

```bash
# Translate a WAV file from Spanish to English
./target/release/gemma-edge-translator \
  --wav input.wav \
  --direction es-en \
  --gemma-model path/to/gemma-2b-it.gguf \
  --api-key your-openai-key

# Real-time translation (5 seconds of recording)
./target/release/gemma-edge-translator \
  --realtime 5 \
  --direction en-es \
  --gemma-model path/to/gemma-2b-it.gguf

# Launch web UI with performance monitoring
./target/release/gemma-edge-translator --ui --port 8080 --gemma-model models/gemma-2b-it.Q4_K_M.gguf --direction en-es
```

## 🎥 Tutorial Video

[![Gemma Edge Translator Tutorial - Real-time Speech Translation with Performance Monitoring](./assets/Screenshot%202025-08-11%20at%2011.19.16%20AM.png)](./assets/tutorial.mp4)

**🎬 Click the thumbnail above to watch the complete tutorial!**

**What you'll learn:**
- ✅ **Installation & Setup** - Complete build process from scratch
- ✅ **Model Configuration** - Download and setup Gemma-2B-IT model  
- ✅ **Web UI Demo** - Real-time performance monitoring in action
- ✅ **Translation Examples** - Text and voice input demonstrations
- ✅ **Performance Insights** - CPU and memory usage tracking explained
- ✅ **Speech Recognition** - Live microphone input and real-time translation
- ✅ **Peak Detection** - Understanding resource usage spikes

*📹 **Tutorial Video**: [`assets/tutorial.mp4`](./assets/tutorial.mp4) - Full demonstration of features and performance monitoring*

## 📋 Requirements

### System Requirements

| Platform | Minimum | Recommended |
|----------|---------|-------------|
| **macOS** | macOS 11+ (Apple Silicon/Intel) | 8GB RAM |
| **Linux** | Ubuntu 20.04+, Debian 11+ | 4GB RAM |
| **Raspberry Pi** | Pi 4 (8GB), 64-bit OS | Class 10 SD Card |

### Dependencies

- **Audio**: ALSA (Linux) or CoreAudio (macOS)
- **Build Tools**: GCC/Clang, pkg-config
- **Models**: Gemma-2B-IT GGUF format

## 🛠️ Platform-Specific Setup

### macOS (Intel/Apple Silicon)

```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install system dependencies
brew install pkg-config

# Build the project
cargo build --release --features "ui,realtime"
```

### Linux (Ubuntu/Debian)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install system dependencies
sudo apt update
sudo apt install -y \
  build-essential \
  pkg-config \
  libasound2-dev \
  libssl-dev \
  cmake

# Build the project
cargo build --release --features "ui,realtime"
```

### Raspberry Pi (64-bit OS)

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install Rust (this may take a while)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install build dependencies
sudo apt install -y \
  build-essential \
  pkg-config \
  libasound2-dev \
  libssl-dev \
  cmake \
  git

# Clone and build (expect 30-60 minutes)
git clone https://github.com/rdxvicky/gemma-edge-translator.git
cd gemma-edge-translator

# Build with optimizations for Pi
RUSTFLAGS="-C target-cpu=native" \
cargo build --release --features "ui,realtime"
```

## 📥 Model Setup

### Download Gemma Model

```bash
# Create models directory
mkdir -p models
cd models

# Download Gemma-2B-IT GGUF (adjust URL as needed)
wget https://huggingface.co/codegood/gemma-2b-it-Q4_K_M-GGUF?show_file_info=gemma-2b-it.Q4_K_M.gguf

### OpenAI API Setup

```bash
# Set your API key
export OPENAI_API_KEY="your-api-key-here"

# Or pass it as a command line argument
--api-key your-api-key-here
```

### Local Whisper Setup (Optional)

```bash
# Install whisper.cpp or similar
# Then use the --local flag
--local
```

## 🎯 Usage Examples

### File Translation

```bash
# Spanish audio to English text
./gemma-edge-translator \
  --wav spanish_audio.wav \
  --direction es-en \
  --gemma-model models/gemma-2-2b-it-Q4_K_M.gguf
```

### Real-time Translation

```bash
# Record 10 seconds and translate English to Spanish
./gemma-edge-translator \
  --realtime 10 \
  --direction en-es \
  --gemma-model models/gemma-2-2b-it-Q4_K_M.gguf \
  --verbose
```

### Web Interface

```bash
# Start web UI on port 3000
./gemma-edge-translator \
  --ui \
  --port 3000 \
  --gemma-model models/gemma-2-2b-it-Q4_K_M.gguf

# Then open http://localhost:3000
```

## ⚙️ Configuration

### Command Line Options

```
OPTIONS:
    --wav <WAV>                  Path to mono 16kHz WAV file
    --realtime <REALTIME>        Realtime mic capture (seconds)
    --direction <DIRECTION>      Direction: es-en or en-es
    --api-key <API_KEY>          OpenAI API key
    --local                      Use local Whisper API
    --gemma-model <GEMMA_MODEL>  Path to Gemma model (GGUF)
    --gemma-ctx <GEMMA_CTX>      Context tokens [default: 2048]
    --ui                         Run local UI
    --port <PORT>                UI port [default: 8080]
    --verbose                    Verbose logs
```

### Environment Variables

```bash
# OpenAI API Key
export OPENAI_API_KEY="your-key"

# Rust log level
export RUST_LOG="debug"  # or info, warn, error
```

## 🔧 Build Features

The project uses Cargo features for optional functionality:

```bash
# Basic build (translation only)
cargo build --release

# With UI support
cargo build --release --features ui

# With real-time recording
cargo build --release --features realtime

# Everything enabled
cargo build --release --features "ui,realtime"
```

## 📊 Performance Monitoring

The web UI includes real-time performance monitoring to help you understand your application's resource usage:

### Features
- **Application-specific tracking**: Shows CPU and memory usage for the translator process only
- **Peak detection**: Captures and displays maximum CPU and memory usage spikes
- **Real-time updates**: Performance metrics refresh every second
- **Historical data**: Maintains 5 minutes of performance history
- **Reset capability**: Clear peak metrics with the "Reset Peaks" button

### What to Expect
- **Memory Usage**: 
  - Starts at ~10-50MB (basic application)
  - Jumps to ~3GB when Gemma model loads
  - Peak memory shows the highest usage since startup
- **CPU Usage**:
  - Shows real-time CPU consumption during translations
  - Spikes during model inference (can reach 50-200%+ on multi-core systems)
  - Peak CPU captures the highest usage spike

### Usage
```bash
# Launch with performance monitoring
./target/release/gemma-edge-translator \
  --ui \
  --port 8080 \
  --gemma-model models/gemma-2b-it.Q4_K_M.gguf \
  --direction en-es

# Then visit http://localhost:8080 to see real-time metrics
```

## 🐛 Troubleshooting

### Common Issues

**Audio device not found**
```bash
# Linux: Check ALSA devices
arecord -l

# macOS: Check audio permissions in System Preferences
```

**Model loading fails**
```bash
# Verify model path and format
ls -la models/
file models/your-model.gguf
```

**Build errors on Pi**
```bash
# Increase swap space
sudo dphys-swapfile swapoff
sudo sed -i 's/CONF_SWAPSIZE=100/CONF_SWAPSIZE=2048/' /etc/dphys-swapfile
sudo dphys-swapfile setup
sudo dphys-swapfile swapon
```

### Performance Tuning

**Raspberry Pi Optimizations**
```bash
# Enable hardware acceleration
echo 'gpu_mem=128' | sudo tee -a /boot/config.txt

# Use faster SD card (Class 10 or better)
# Consider USB 3.0 storage for models
```

**Memory Usage**
```bash
# Reduce context size for limited RAM
--gemma-ctx 1024

# Use quantized models (Q4_K_M instead of F16)
```

## 📊 Performance Benchmarks (Still Validating, placeholder numbers)

| Platform | Model Size | Context | RAM Usage | Processing Time |
|----------|------------|---------|-----------|----------------|
| MacBook Pro M1 | 2B Q4_K_M | 2048 | ~3GB | ~2-3s |
| Raspberry Pi 4 8GB | 2B Q4_K_M | 1024 | ~2GB | ~8-12s |

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

```bash
git clone https://github.com/rdxvicky/gemma-edge-translator.git
cd gemma-edge-translator

# Install development dependencies
cargo install cargo-watch

# Run with auto-reload
cargo watch -x 'run --features "ui,realtime" -- --help'
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [OpenAI Whisper](https://github.com/openai/whisper) for speech recognition
- [Google Gemma](https://ai.google.dev/gemma) for language modeling
- [llama.cpp](https://github.com/ggerganov/llama.cpp) for efficient inference

---

**Made with ❤️ for real-time translation**
