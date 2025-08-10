#!/bin/bash

# Gemma Translator RS - Usage Examples
# This script demonstrates various ways to use the application

set -e

BINARY="./target/release/gemma-edge-translator"
MODEL_PATH="models/gemma-2-2b-it-Q4_K_M.gguf"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🎯 Gemma Translator RS - Usage Examples${NC}"
echo "========================================="

# Check if binary exists
if [[ ! -f "$BINARY" ]]; then
    echo -e "${RED}❌ Binary not found. Please run:${NC}"
    echo "   cargo build --release --features \"ui,realtime\""
    exit 1
fi

# Check if model exists
if [[ ! -f "$MODEL_PATH" ]]; then
    echo -e "${YELLOW}⚠️  Model not found at $MODEL_PATH${NC}"
    echo "   Please download a Gemma model first (see README for instructions)"
    echo "   Continuing with examples anyway..."
    MODEL_PATH="path/to/your/model.gguf"
fi

# Check API key
if [[ -z "$OPENAI_API_KEY" ]]; then
    echo -e "${YELLOW}⚠️  OPENAI_API_KEY not set${NC}"
    echo "   Export your API key: export OPENAI_API_KEY=\"your-key\""
    echo "   Or use --api-key flag in commands below"
    echo ""
fi

echo ""
echo -e "${GREEN}📝 Example Commands:${NC}"
echo ""

echo -e "${BLUE}1. Show help:${NC}"
echo "   $BINARY --help"
echo ""

echo -e "${BLUE}2. Translate a WAV file (Spanish to English):${NC}"
echo "   $BINARY \\"
echo "     --wav input_spanish.wav \\"
echo "     --direction es-en \\"
echo "     --gemma-model $MODEL_PATH"
echo ""

echo -e "${BLUE}3. Translate a WAV file (English to Spanish):${NC}"
echo "   $BINARY \\"
echo "     --wav input_english.wav \\"
echo "     --direction en-es \\"
echo "     --gemma-model $MODEL_PATH"
echo ""

echo -e "${BLUE}4. Real-time recording and translation (5 seconds):${NC}"
echo "   $BINARY \\"
echo "     --realtime 5 \\"
echo "     --direction es-en \\"
echo "     --gemma-model $MODEL_PATH \\"
echo "     --verbose"
echo ""

echo -e "${BLUE}5. Real-time with custom duration (10 seconds):${NC}"
echo "   $BINARY \\"
echo "     --realtime 10 \\"
echo "     --direction en-es \\"
echo "     --gemma-model $MODEL_PATH"
echo ""

echo -e "${BLUE}6. Use local Whisper instead of OpenAI:${NC}"
echo "   $BINARY \\"
echo "     --wav input.wav \\"
echo "     --direction es-en \\"
echo "     --gemma-model $MODEL_PATH \\"
echo "     --local"
echo ""

echo -e "${BLUE}7. Launch web UI:${NC}"
echo "   $BINARY \\"
echo "     --ui \\"
echo "     --port 8080 \\"
echo "     --gemma-model $MODEL_PATH"
echo "   # Then open http://localhost:8080"
echo ""

echo -e "${BLUE}8. Custom context size (for limited RAM):${NC}"
echo "   $BINARY \\"
echo "     --wav input.wav \\"
echo "     --direction es-en \\"
echo "     --gemma-model $MODEL_PATH \\"
echo "     --gemma-ctx 1024"
echo ""

echo -e "${BLUE}9. With explicit API key:${NC}"
echo "   $BINARY \\"
echo "     --wav input.wav \\"
echo "     --direction es-en \\"
echo "     --gemma-model $MODEL_PATH \\"
echo "     --api-key sk-your-openai-key-here"
echo ""

echo -e "${BLUE}10. Verbose logging for debugging:${NC}"
echo "    $BINARY \\"
echo "      --realtime 5 \\"
echo "      --direction es-en \\"
echo "      --gemma-model $MODEL_PATH \\"
echo "      --verbose"
echo ""

echo -e "${GREEN}💡 Tips:${NC}"
echo "• Use shorter context sizes (--gemma-ctx) on Raspberry Pi"
echo "• Test with --local flag if OpenAI API is slow/unavailable"
echo "• Try --verbose for troubleshooting"
echo "• The web UI provides a user-friendly interface for testing"
echo ""

echo -e "${GREEN}🎙️ Quick Test (if you have a microphone):${NC}"
if [[ -f "$BINARY" && -f "$MODEL_PATH" && -n "$OPENAI_API_KEY" ]]; then
    echo "Ready to test! Try:"
    echo "   $BINARY --realtime 3 --direction es-en --gemma-model $MODEL_PATH"
else
    echo "Complete the setup first (see README.md), then try:"
    echo "   $BINARY --realtime 3 --direction es-en --gemma-model $MODEL_PATH"
fi

echo ""
echo "Happy translating! 🚀"
