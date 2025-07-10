FROM python:3.11-slim

# Set working directory
WORKDIR /app

# Install system dependencies for tarzi (may need browser automation tools)
RUN apt-get update && apt-get install -y \
    wget \
    curl \
    ca-certificates \
    gnupg \
    && rm -rf /var/lib/apt/lists/*

# Copy requirements
COPY pyproject.toml ./

# Install Python dependencies
RUN pip install --no-cache-dir --upgrade pip && \
    pip install --no-cache-dir .

# Copy the application
COPY tarzi_mcp_server/ ./tarzi_mcp_server/

# Create a non-root user
RUN groupadd -r mcpuser && useradd -r -g mcpuser mcpuser
RUN chown -R mcpuser:mcpuser /app
USER mcpuser

# Expose the default port
EXPOSE 8000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8000/ || exit 1

# Default command
CMD ["python", "-m", "tarzi_mcp_server.server", "--host", "0.0.0.0", "--port", "8000"]