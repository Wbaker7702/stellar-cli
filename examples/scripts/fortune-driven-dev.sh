#!/bin/bash
# Fortune-Driven Development (FDD) Script
# Let the cosmic forces guide your commits!

set -e

echo "🔮 Fortune-Driven Development (FDD)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Get fortune to decide if we should commit
echo "📜 Consulting the blockchain oracle..."
FORTUNE=$(stellar fortune -t blockchain)
echo "$FORTUNE"
echo ""

# Extract fortune text (simple approach)
FORTUNE_TEXT=$(echo "$FORTUNE" | grep -E "^[[:space:]]*[🌟🚀⛓️💎📈]" | head -1)

# Check if fortune is favorable
if echo "$FORTUNE_TEXT" | grep -qiE "success|positive|favorable|great|excellent|moon|deploy"; then
    echo "✅ Fortune is FAVORABLE! Proceeding with commit..."
    echo ""
    
    # Show stats
    stellar stats
    echo ""
    
    # Commit changes
    if git diff --cached --quiet; then
        echo "📝 No staged changes to commit"
    else
        read -p "📝 Enter commit message: " commit_msg
        git commit -m "$commit_msg" -m "Fortune: $FORTUNE_TEXT"
        
        echo ""
        echo "✅ Committed successfully!"
        
        # Celebrate
        stellar meme -t chad
        
        # Notify
        stellar notify send \
            -m "New commit: $commit_msg" \
            -t "Git Commit" \
            -p normal
    fi
    
elif echo "$FORTUNE_TEXT" | grep -qiE "caution|warning|test|debug"; then
    echo "⚠️  Fortune advises CAUTION. Run tests first!"
    echo ""
    
    read -p "Run tests before committing? [Y/n]: " run_tests
    if [ "$run_tests" != "n" ]; then
        echo "🧪 Running tests..."
        cargo test
        
        echo ""
        echo "✅ Tests passed! Safe to commit."
        stellar meme -t hodl
    fi
    
else
    echo "❌ Fortune is UNFAVORABLE! Maybe wait a bit..."
    echo ""
    echo "💡 Suggestions:"
    echo "   • Review your changes carefully"
    echo "   • Run tests: cargo test"
    echo "   • Try again in a few minutes"
    echo ""
    
    stellar meme -t wojak
    
    read -p "Proceed anyway? [y/N]: " force
    if [ "$force" != "y" ]; then
        echo "🛑 Commit cancelled by cosmic forces"
        exit 1
    fi
fi

echo ""
echo "╔═══════════════════════════════════════════════════════╗"
echo "║    🔮 May the blockchain be with you! 🔮            ║"
echo "╚═══════════════════════════════════════════════════════╝"
