#!/bin/bash

# Tarzi MCP Server Startup Script with Headless Browser Support
set -e

echo "🚀 Starting Tarzi MCP Server with headless browser support..."

# Create necessary directories
mkdir -p /app/browser-data/downloads
mkdir -p /app/browser-data/cache
mkdir -p /app/.mozilla/firefox/tarzi.profile

# Set up Firefox profile if it doesn't exist
if [ ! -f "/app/.mozilla/firefox/tarzi.profile/prefs.js" ]; then
    echo "📄 Setting up Firefox profile..."
    cp -r /app/firefox-config/* /app/.mozilla/firefox/tarzi.profile/ 2>/dev/null || true
fi

# Set environment variables for browser automation
export DISPLAY=${DISPLAY:-:99}
export MOZ_HEADLESS=${MOZ_HEADLESS:-1}
export FIREFOX_BINARY_PATH=${FIREFOX_BINARY_PATH:-/usr/bin/firefox-esr}
export GECKODRIVER_PATH=${GECKODRIVER_PATH:-/usr/local/bin/geckodriver}

# Set up virtual display for headless operation
if [ -z "$DISPLAY" ] || [ "$DISPLAY" = ":99" ]; then
    echo "🖥️  Setting up virtual display..."
    Xvfb :99 -screen 0 1920x1080x24 -ac +extension GLX +render -noreset &
    export DISPLAY=:99
    sleep 2
fi

# Verify Firefox installation
echo "🦊 Verifying Firefox installation..."
if command -v firefox-esr >/dev/null 2>&1; then
    echo "✅ Firefox ESR found: $(firefox-esr --version)"
else
    echo "❌ Firefox ESR not found!"
    exit 1
fi

# Verify geckodriver installation
echo "🔧 Verifying geckodriver installation..."
if command -v geckodriver >/dev/null 2>&1; then
    echo "✅ Geckodriver found: $(geckodriver --version | head -1)"
else
    echo "❌ Geckodriver not found!"
    exit 1
fi

# Test Firefox headless mode
echo "🧪 Testing Firefox headless mode..."
timeout 10s firefox-esr --headless --new-instance --no-remote --profile /app/.mozilla/firefox/tarzi.profile --url about:blank &
FIREFOX_PID=$!
sleep 3
if kill -0 $FIREFOX_PID 2>/dev/null; then
    echo "✅ Firefox headless mode working"
    kill $FIREFOX_PID 2>/dev/null || true
else
    echo "⚠️  Firefox headless test completed"
fi

# Set browser configuration for tarzi
export TARZI_BROWSER_BINARY_PATH="/usr/bin/firefox-esr"
export TARZI_BROWSER_PROFILE_PATH="/app/.mozilla/firefox/tarzi.profile"
export TARZI_BROWSER_DATA_DIR="/app/browser-data"
export TARZI_BROWSER_CACHE_DIR="/app/browser-data/cache"
export TARZI_BROWSER_DOWNLOADS_DIR="/app/browser-data/downloads"

# Set browser-specific environment variables
export SELENIUM_BROWSER="firefox"
export PLAYWRIGHT_BROWSERS_PATH="/app/.playwright"

echo "🌐 Browser environment configured:"
echo "   - Display: $DISPLAY"
echo "   - Firefox Binary: $FIREFOX_BINARY_PATH"
echo "   - Geckodriver: $GECKODRIVER_PATH"
echo "   - Profile: $TARZI_BROWSER_PROFILE_PATH"
echo "   - Data Directory: $TARZI_BROWSER_DATA_DIR"

# Start the MCP server
echo "🚀 Starting Tarzi MCP Server..."
exec python -m tarzi_mcp_server.server "$@"