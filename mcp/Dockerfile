FROM python:3.11-slim

# Set working directory
WORKDIR /app

# Install system dependencies for Firefox and browser automation
RUN apt-get update && apt-get install -y \
    wget \
    curl \
    ca-certificates \
    gnupg \
    unzip \
    xvfb \
    firefox-esr \
    # Additional dependencies for browser automation
    libgtk-3-0 \
    libdbus-glib-1-2 \
    libxt6 \
    libxcomposite1 \
    libxdamage1 \
    libxrandr2 \
    libasound2 \
    libpangocairo-1.0-0 \
    libatk1.0-0 \
    libcairo-gobject2 \
    libgtk-3-0 \
    libgdk-pixbuf2.0-0 \
    libxss1 \
    libgconf-2-4 \
    libxrandr2 \
    libasound2 \
    libpangocairo-1.0-0 \
    libatk1.0-0 \
    libcairo-gobject2 \
    libdrm2 \
    libxcomposite1 \
    libxdamage1 \
    libxrandr2 \
    libgbm1 \
    libxkbcommon0 \
    libatspi2.0-0 \
    # Process management utilities
    psmisc \
    procps \
    # Cleanup
    && rm -rf /var/lib/apt/lists/*

# Install geckodriver for Firefox
ENV GECKODRIVER_VERSION=0.34.0
RUN wget -q -O /tmp/geckodriver.tar.gz \
    "https://github.com/mozilla/geckodriver/releases/download/v${GECKODRIVER_VERSION}/geckodriver-v${GECKODRIVER_VERSION}-linux64.tar.gz" \
    && tar -xzf /tmp/geckodriver.tar.gz -C /usr/local/bin \
    && chmod +x /usr/local/bin/geckodriver \
    && rm /tmp/geckodriver.tar.gz

# Set Firefox preferences for headless operation
RUN mkdir -p /app/.mozilla/firefox/default.profile \
    && echo 'user_pref("browser.shell.checkDefaultBrowser", false);' > /app/.mozilla/firefox/default.profile/prefs.js \
    && echo 'user_pref("browser.startup.page", 0);' >> /app/.mozilla/firefox/default.profile/prefs.js \
    && echo 'user_pref("browser.startup.homepage_override.mstone", "ignore");' >> /app/.mozilla/firefox/default.profile/prefs.js

# Set environment variables for headless browser operation
ENV DISPLAY=:99
ENV MOZ_HEADLESS=1
ENV FIREFOX_BINARY_PATH=/usr/bin/firefox-esr
ENV GECKODRIVER_PATH=/usr/local/bin/geckodriver

# Copy requirements
COPY pyproject.toml ./

# Install Python dependencies including tarzi and browser automation packages
RUN pip install --no-cache-dir --upgrade pip && \
    pip install --no-cache-dir \
    mcp>=1.10.0 \
    tarzi>=0.0.16 \
    uvicorn>=0.30.0 \
    fastapi>=0.115.0 \
    pydantic>=2.0.0 \
    selenium>=4.15.0 \
    playwright>=1.40.0 \
    requests>=2.31.0 \
    beautifulsoup4>=4.12.0 \
    lxml>=4.9.0 \
    webdriver-manager>=4.0.0

# Install Playwright browsers (includes Firefox)
RUN playwright install firefox && \
    playwright install-deps firefox

# Copy Firefox configuration
COPY firefox-config/ /app/firefox-config/

# Copy the application
COPY tarzi_mcp_server/ ./tarzi_mcp_server/

# Copy startup script
COPY start-server.sh /app/start-server.sh

# Create Firefox profile directory
RUN mkdir -p /app/.mozilla/firefox/tarzi.profile

# Create a non-root user
RUN groupadd -r mcpuser && useradd -r -g mcpuser mcpuser

# Set up proper permissions
RUN chown -R mcpuser:mcpuser /app && \
    chown -R mcpuser:mcpuser /app/.mozilla && \
    chmod -R 755 /app/.mozilla && \
    chmod +x /app/start-server.sh

# Create directory for browser data
RUN mkdir -p /app/browser-data && \
    chown -R mcpuser:mcpuser /app/browser-data

USER mcpuser

# Expose the default port
EXPOSE 8000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=15s --retries=3 \
    CMD curl -f http://localhost:8000/ || exit 1

# Default command using startup script
CMD ["/app/start-server.sh", "--host", "0.0.0.0", "--port", "8000"]