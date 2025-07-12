# Authentication

## Overview

Tarzi uses API key authentication for all endpoints. This ensures secure access to the API while maintaining compatibility with Serper's authentication mechanism.

## API Key Authentication

### Header-based Authentication

All API requests must include the API key in the `X-API-KEY` header:

```http
X-API-KEY: your-api-key-here
```

### Example Request

```bash
curl -X POST https://api.tarzi.dev/v1/search \
  -H "Content-Type: application/json" \
  -H "X-API-KEY: your-api-key-here" \
  -d '{
    "q": "tarzi search api"
  }'
```

## API Key Management

### Obtaining API Keys

1. Sign up at [https://tarzi.dev/signup](https://tarzi.dev/signup)
2. Verify your email address
3. Navigate to the API Keys section in your dashboard
4. Generate a new API key
5. Copy and securely store your API key

### API Key Properties

- **Format**: 32-character alphanumeric string
- **Example**: `tk_1234567890abcdef1234567890abcdef`
- **Prefix**: All API keys start with `tk_`
- **Scope**: Full access to all available endpoints
- **Lifetime**: API keys do not expire unless revoked

### API Key Security

- **Keep Secret**: Never expose API keys in client-side code
- **Environment Variables**: Store API keys in environment variables
- **Rotation**: Regularly rotate API keys for enhanced security
- **Monitoring**: Monitor API key usage in your dashboard

## Authentication Errors

### Invalid API Key

```json
{
  "error": {
    "type": "authentication_error",
    "message": "Invalid API key provided",
    "code": "INVALID_API_KEY"
  }
}
```

**HTTP Status Code**: `401 Unauthorized`

### Missing API Key

```json
{
  "error": {
    "type": "authentication_error",
    "message": "API key is required",
    "code": "MISSING_API_KEY"
  }
}
```

**HTTP Status Code**: `401 Unauthorized`

### Suspended API Key

```json
{
  "error": {
    "type": "authentication_error",
    "message": "API key has been suspended",
    "code": "SUSPENDED_API_KEY"
  }
}
```

**HTTP Status Code**: `403 Forbidden`

### Quota Exceeded

```json
{
  "error": {
    "type": "quota_error",
    "message": "API key quota exceeded",
    "code": "QUOTA_EXCEEDED"
  }
}
```

**HTTP Status Code**: `429 Too Many Requests`

## Rate Limiting

### Rate Limit Headers

All responses include rate limiting information:

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1640995200
X-RateLimit-Window: 3600
```

### Rate Limit Tiers

| Tier | Requests/Hour | Requests/Day | Burst Limit |
|------|---------------|--------------|-------------|
| Free | 100 | 1,000 | 10 |
| Pro | 1,000 | 10,000 | 50 |
| Enterprise | 10,000 | 100,000 | 200 |

### Rate Limit Reset

Rate limits reset at the top of each hour (UTC). The `X-RateLimit-Reset` header contains the Unix timestamp when the rate limit will reset.

## IP Whitelisting

### Optional IP Restrictions

For enhanced security, you can restrict API key usage to specific IP addresses:

1. Navigate to your API key settings
2. Add IP addresses or CIDR ranges
3. Save the configuration

### IP Restriction Format

```json
{
  "allowed_ips": [
    "192.168.1.1",
    "10.0.0.0/8",
    "203.0.113.0/24"
  ]
}
```

## Best Practices

### Security Recommendations

1. **Use HTTPS**: Always use HTTPS for API requests
2. **Server-Side Only**: Never expose API keys in client-side code
3. **Environment Variables**: Store API keys in environment variables
4. **Least Privilege**: Use IP whitelisting when possible
5. **Regular Rotation**: Rotate API keys periodically
6. **Monitor Usage**: Regularly check API usage patterns

### Code Examples

#### Node.js with Environment Variables

```javascript
const axios = require('axios');

const apiKey = process.env.TARZI_API_KEY;

const searchResults = await axios.post('https://api.tarzi.dev/v1/search', {
  q: 'search query'
}, {
  headers: {
    'X-API-KEY': apiKey,
    'Content-Type': 'application/json'
  }
});
```

#### Python with Environment Variables

```python
import os
import requests

api_key = os.getenv('TARZI_API_KEY')

response = requests.post(
    'https://api.tarzi.dev/v1/search',
    json={'q': 'search query'},
    headers={
        'X-API-KEY': api_key,
        'Content-Type': 'application/json'
    }
)
```

## Migration from Serper

### Drop-in Replacement

Tarzi is designed as a drop-in replacement for Serper. Simply update:

1. **Base URL**: Change from `https://google.serper.dev` to `https://api.tarzi.dev/v1`
2. **Keep everything else**: Headers, request format, and response handling remain the same

### Migration Example

**Before (Serper)**:
```bash
curl -X POST https://google.serper.dev/search \
  -H "X-API-KEY: your-serper-key" \
  -H "Content-Type: application/json" \
  -d '{"q": "search query"}'
```

**After (Tarzi)**:
```bash
curl -X POST https://api.tarzi.dev/v1/search \
  -H "X-API-KEY: your-tarzi-key" \
  -H "Content-Type: application/json" \
  -d '{"q": "search query"}'
```