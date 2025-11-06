#!/bin/bash
set -e

echo "Testing new Stellar CLI features..."
echo ""
echo "1. Building with cargo check (checking syntax)..."

# Try to build just checking syntax
cd cmd/soroban-cli
cargo check --message-format=short 2>&1 | grep -E "(Checking|Compiling|error|warning)" | tail -50

echo ""
echo "Features added:"
echo "  - fortune: Get blockchain fortunes 🔮"
echo "  - stats: Show project statistics 📊"
echo "  - meme: Generate crypto memes 🚀"
