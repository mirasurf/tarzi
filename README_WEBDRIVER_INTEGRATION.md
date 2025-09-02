# External WebDriver Integration Guide

This guide explains how to set up and use external WebDriver servers for Tarzi integration tests and production usage.

## Overview

Tarzi supports two WebDriver management modes:

1. **Self-managed**: Tarzi automatically starts and stops WebDriver processes
2. **External**: Connect to an externally managed WebDriver server

External WebDriver mode is particularly useful for:
- CI/CD environments where WebDriver is managed by the infrastructure
- Production deployments with dedicated WebDriver clusters
- Development environments with persistent WebDriver instances
- Integration testing with shared WebDriver resources

## Supported WebDriver Types

Tarzi supports the following WebDriver implementations:

| WebDriver | Default Port | Binary Name | Notes |
|-----------|--------------|-------------|-------|
| **GeckoDriver** (Firefox) | 4444 | `geckodriver` | Default, most stable |
| **ChromeDriver** (Chrome) | 9515 | `chromedriver` | Alternative option |

## Configuration

### Basic Configuration

Configure external WebDriver in your `tarzi.toml`:

```toml
[fetcher]
# Use external WebDriver server
web_driver_url = "http://localhost:4444"
web_driver = "geckodriver"  # Still required for capability detection
```

### Environment Variable Override

You can also set the WebDriver URL via environment variable:

```bash
export TARZI_WEBDRIVER_URL="http://localhost:4444"
```

### Programmatic Configuration

```rust
use tarzi::config::Config;

let mut config = Config::default();
config.fetcher.web_driver_url = Some("http://localhost:4444".to_string());
config.fetcher.web_driver = "geckodriver".to_string();
```

## Setting Up External WebDriver Servers

### Option 1: GeckoDriver (Firefox) - Recommended

#### Installation

**macOS (using Homebrew):**
```bash
brew install geckodriver
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get install firefox-geckodriver
```

**Manual Installation:**
1. Download from [Mozilla's releases](https://github.com/mozilla/geckodriver/releases)
2. Extract and add to PATH

#### Starting GeckoDriver Server

**Basic startup:**
```bash
geckodriver --port 4444
```

**With custom host and port:**
```bash
geckodriver --host 0.0.0.0 --port 4444
```

**With logging:**
```bash
geckodriver --port 4444 --log info
```

**Background service (Linux/macOS):**
```bash
nohup geckodriver --port 4444 --log info > geckodriver.log 2>&1 &
```

#### Docker Setup

```bash
# Run GeckoDriver in Docker
docker run -d -p 4444:4444 --name geckodriver \
  --shm-size=2g \
  selenium/standalone-firefox:latest
```

### Option 2: ChromeDriver (Chrome)

#### Installation

**macOS (using Homebrew):**
```bash
brew install chromedriver
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get install chromium-chromedriver
```

**Manual Installation:**
1. Download from [ChromeDriver releases](https://chromedriver.chromium.org/downloads)
2. Extract and add to PATH

#### Starting ChromeDriver Server

**Basic startup:**
```bash
chromedriver --port=9515
```

**With custom host and port:**
```bash
chromedriver --port=9515 --whitelisted-ips=0.0.0.0
```

**Background service:**
```bash
nohup chromedriver --port=9515 > chromedriver.log 2>&1 &
```

#### Docker Setup

```bash
# Run ChromeDriver in Docker
docker run -d -p 9515:9515 --name chromedriver \
  --shm-size=2g \
  selenium/standalone-chrome:latest
```

### Option 3: Selenium Grid (Advanced)

For production environments, consider using Selenium Grid:

```bash
# Start Selenium Hub
docker run -d -p 4444:4444 --name selenium-hub selenium/hub:latest

# Start Firefox node
docker run -d --name firefox-node \
  --link selenium-hub:hub \
  -e HUB_HOST=hub \
  selenium/node-firefox:latest

# Start Chrome node
docker run -d --name chrome-node \
  --link selenium-hub:hub \
  -e HUB_HOST=hub \
  selenium/node-chrome:latest
```

## Integration Testing

### Running External WebDriver Tests

1. **Start external WebDriver server:**
   ```bash
   geckodriver --port 4444 --log info
   ```

2. **Run integration tests:**
   ```bash
   # Run all integration tests
   make test-integration

   # Run specific external WebDriver tests
   cargo test --test search_external_webdriver_integration_tests

   # Run with verbose output
   cargo test --test search_external_webdriver_integration_tests -- --nocapture
   ```

### Test Categories

The external WebDriver integration tests include:

- **Individual Engine Tests**: Test each search engine (Bing, Google, DuckDuckGo, Brave, Baidu)
- **Search with Content**: Test content fetching alongside search
- **Error Handling**: Test behavior when WebDriver is unavailable
- **Multiple Engines**: Test switching between engines with same WebDriver instance

### Test Configuration

Tests automatically detect if external WebDriver is available and skip gracefully if not:

```rust
if !is_webdriver_available("http://localhost:4444").await {
    println!("⚠️  Skipping test - external WebDriver not available");
    return;
}
```

## Production Deployment

### Docker Compose Example

```yaml
version: '3.8'
services:
  geckodriver:
    image: selenium/standalone-firefox:latest
    ports:
      - "4444:4444"
    shm_size: 2g
    environment:
      - SE_NODE_MAX_SESSIONS=4
      - SE_NODE_SESSION_TIMEOUT=300

  tarzi-app:
    build: .
    depends_on:
      - geckodriver
    environment:
      - TARZI_WEBDRIVER_URL=http://geckodriver:4444
    volumes:
      - ./tarzi.toml:/app/tarzi.toml
```

### Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: tarzi-app
spec:
  replicas: 3
  selector:
    matchLabels:
      app: tarzi
  template:
    metadata:
      labels:
        app: tarzi
    spec:
      containers:
      - name: tarzi
        image: tarzi:latest
        env:
        - name: TARZI_WEBDRIVER_URL
          value: "http://selenium-hub:4444"
        resources:
          requests:
            memory: "512Mi"
            cpu: "250m"
          limits:
            memory: "1Gi"
            cpu: "500m"
---
apiVersion: v1
kind: Service
metadata:
  name: selenium-hub
spec:
  selector:
    app: selenium-hub
  ports:
  - port: 4444
    targetPort: 4444
```

## Troubleshooting

### Common Issues

#### 1. Connection Refused

**Error:** `External WebDriver server is not available at http://localhost:4444`

**Solutions:**
- Verify WebDriver server is running: `curl http://localhost:4444/status`
- Check firewall settings
- Ensure correct port number
- Verify WebDriver binary is in PATH

#### 2. Session Creation Failed

**Error:** `Failed to create new session`

**Solutions:**
- Check available system resources (memory, CPU)
- Verify browser binary is installed
- Check WebDriver logs for detailed error messages
- Try restarting WebDriver server

#### 3. Timeout Issues

**Error:** `Search timed out`

**Solutions:**
- Increase timeout in configuration
- Check network connectivity
- Verify search engine is accessible
- Monitor WebDriver server performance

### Debugging

#### Enable Verbose Logging

```bash
# GeckoDriver with debug logging
geckodriver --port 4444 --log debug

# ChromeDriver with verbose logging
chromedriver --port=9515 --verbose
```

#### Check WebDriver Status

```bash
# Check if WebDriver is responding
curl http://localhost:4444/status

# Check active sessions
curl http://localhost:4444/sessions
```

#### Monitor Resources

```bash
# Check WebDriver process
ps aux | grep geckodriver

# Monitor network connections
netstat -tulpn | grep 4444

# Check system resources
top -p $(pgrep geckodriver)
```

## Performance Considerations

### Resource Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| **Memory** | 512MB | 2GB |
| **CPU** | 1 core | 2 cores |
| **Disk** | 100MB | 1GB |

### Optimization Tips

1. **Connection Pooling**: Use Selenium Grid for multiple concurrent sessions
2. **Resource Limits**: Set appropriate memory and CPU limits
3. **Session Management**: Implement proper session cleanup
4. **Monitoring**: Monitor WebDriver server health and performance

### Scaling

For high-load scenarios:

1. **Horizontal Scaling**: Deploy multiple WebDriver instances
2. **Load Balancing**: Use Selenium Grid or custom load balancer
3. **Session Management**: Implement session pooling and reuse
4. **Monitoring**: Set up comprehensive monitoring and alerting

## Security Considerations

### Network Security

- Use HTTPS for WebDriver connections in production
- Implement proper firewall rules
- Use VPN or private networks for sensitive deployments

### Access Control

- Restrict WebDriver server access to trusted networks
- Implement authentication if needed
- Use container isolation for multi-tenant deployments

### Resource Limits

- Set appropriate memory and CPU limits
- Implement rate limiting for search operations
- Monitor and log all WebDriver activities

## Best Practices

1. **Health Checks**: Implement regular health checks for WebDriver servers
2. **Graceful Degradation**: Handle WebDriver unavailability gracefully
3. **Resource Cleanup**: Always clean up WebDriver sessions
4. **Monitoring**: Monitor WebDriver server performance and errors
5. **Documentation**: Document your WebDriver configuration and deployment

## Examples

### Complete Example: Local Development

```bash
# Terminal 1: Start GeckoDriver
geckodriver --port 4444 --log info

# Terminal 2: Run tests
cargo test --test search_external_webdriver_integration_tests -- --nocapture
```

### Complete Example: Docker Development

```bash
# Start WebDriver in Docker
docker run -d -p 4444:4444 --name geckodriver selenium/standalone-firefox:latest

# Run tests
TARZI_WEBDRIVER_URL=http://localhost:4444 cargo test --test search_external_webdriver_integration_tests
```

### Complete Example: Production Configuration

```toml
# tarzi.toml
[fetcher]
web_driver_url = "http://selenium-hub:4444"
web_driver = "geckodriver"
timeout = 60

[search]
engine = "bing"
limit = 10
```

This configuration will connect to a Selenium Grid hub for distributed WebDriver management in production.
