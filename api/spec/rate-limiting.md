# Rate Limiting and Quotas

## Overview

Tarzi API implements rate limiting to ensure fair usage and maintain service quality for all users. Rate limits are enforced per API key and are based on subscription tiers.

## Rate Limiting Strategy

### Per-API-Key Limits

Rate limits are applied individually to each API key, allowing for:
- Fair resource allocation
- Predictable performance
- Scalable usage based on subscription tiers

### Time Windows

Rate limits use a sliding window approach with these characteristics:
- **Window Size**: 1 hour (3600 seconds)
- **Granularity**: 1-minute buckets
- **Reset**: Continuous sliding window (not fixed hourly resets)

## Subscription Tiers

### Free Tier

| Metric | Limit |
|--------|-------|
| Requests per hour | 100 |
| Requests per day | 1,000 |
| Burst limit | 10 requests |
| Concurrent requests | 2 |

### Pro Tier

| Metric | Limit |
|--------|-------|
| Requests per hour | 1,000 |
| Requests per day | 10,000 |
| Burst limit | 50 requests |
| Concurrent requests | 10 |

### Enterprise Tier

| Metric | Limit |
|--------|-------|
| Requests per hour | 10,000 |
| Requests per day | 100,000 |
| Burst limit | 200 requests |
| Concurrent requests | 50 |

### Custom Tiers

Enterprise customers can request custom rate limits based on their specific needs.

## Rate Limit Headers

All API responses include rate limiting information in HTTP headers:

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1640995200
X-RateLimit-Window: 3600
X-RateLimit-Tier: pro
```

### Header Descriptions

| Header | Description |
|--------|-------------|
| `X-RateLimit-Limit` | Total requests allowed in the current window |
| `X-RateLimit-Remaining` | Remaining requests in the current window |
| `X-RateLimit-Reset` | Unix timestamp when the oldest request in the window expires |
| `X-RateLimit-Window` | Window size in seconds (always 3600) |
| `X-RateLimit-Tier` | Current subscription tier |

## Rate Limit Exceeded Response

When rate limits are exceeded, the API returns a `429 Too Many Requests` status:

```json
{
  "error": {
    "type": "rate_limit_error",
    "message": "Rate limit exceeded. Please wait before making another request",
    "code": "RATE_LIMIT_EXCEEDED",
    "details": {
      "limit": 1000,
      "window": "1 hour",
      "reset_time": "2024-01-15T14:00:00Z",
      "retry_after": 3600,
      "tier": "pro"
    }
  }
}
```

### Response Headers for Rate Limit Errors

```http
HTTP/1.1 429 Too Many Requests
Content-Type: application/json; charset=utf-8
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 0
X-RateLimit-Reset: 1640995200
Retry-After: 3600
X-Request-ID: req_1234567890abcdef
```

## Quota System

### Credit-Based Quotas

Each request consumes credits based on the endpoint used:

| Endpoint | Credits per Request |
|----------|-------------------|
| `/search` | 1 credit |
| `/images` | 1 credit |
| `/news` | 1 credit |
| `/videos` | 1 credit |
| `/maps` | 1 credit |
| `/shopping` | 1 credit |
| `/scholar` | 2 credits |
| `/patents` | 2 credits |
| `/autocomplete` | 0.5 credits |

### Monthly Quotas by Tier

| Tier | Monthly Credits | Overage Rate |
|------|----------------|--------------|
| Free | 1,000 | N/A (hard limit) |
| Pro | 10,000 | $0.001 per credit |
| Enterprise | 100,000 | $0.0008 per credit |

### Quota Headers

Credit information is included in all successful responses:

```http
X-Quota-Limit: 10000
X-Quota-Remaining: 9999
X-Quota-Reset: 1643673600
X-Quota-Period: monthly
```

### Quota Exceeded Response

```json
{
  "error": {
    "type": "quota_error",
    "message": "Monthly quota exceeded for your subscription plan",
    "code": "QUOTA_EXCEEDED",
    "details": {
      "current_usage": 10000,
      "quota_limit": 10000,
      "quota_period": "monthly",
      "reset_date": "2024-02-01T00:00:00Z",
      "upgrade_url": "https://tarzi.dev/upgrade"
    }
  }
}
```

## Burst Limits

### Burst Protection

Burst limits prevent sudden spikes that could overwhelm the system:

- **Free Tier**: 10 requests in 1 minute
- **Pro Tier**: 50 requests in 1 minute  
- **Enterprise Tier**: 200 requests in 1 minute

### Burst Limit Headers

```http
X-Burst-Limit: 50
X-Burst-Remaining: 49
X-Burst-Reset: 1640995260
```

## Concurrent Request Limits

### Per-API-Key Concurrency

Maximum simultaneous active requests per API key:

- **Free Tier**: 2 concurrent requests
- **Pro Tier**: 10 concurrent requests
- **Enterprise Tier**: 50 concurrent requests

### Concurrent Limit Exceeded

```json
{
  "error": {
    "type": "rate_limit_error",
    "message": "Too many concurrent requests. Please wait for existing requests to complete",
    "code": "CONCURRENT_LIMIT_EXCEEDED",
    "details": {
      "concurrent_limit": 10,
      "active_requests": 10,
      "retry_after": 5
    }
  }
}
```

## IP-Based Limits

### Global IP Limits

To prevent abuse, global IP-based limits apply:

- **Per IP**: 10,000 requests per hour across all API keys
- **Burst**: 500 requests per minute per IP

### IP Limit Response

```json
{
  "error": {
    "type": "rate_limit_error",
    "message": "IP address rate limit exceeded",
    "code": "IP_RATE_LIMIT_EXCEEDED",
    "details": {
      "client_ip": "203.0.113.1",
      "limit": 10000,
      "window": "1 hour",
      "retry_after": 600
    }
  }
}
```

## Best Practices

### Efficient API Usage

1. **Monitor Headers**: Always check rate limit headers
2. **Implement Backoff**: Use exponential backoff for retries
3. **Batch Requests**: Combine multiple queries when possible
4. **Cache Results**: Cache responses to reduce API calls
5. **Off-Peak Usage**: Distribute load across time zones

### Request Optimization

1. **Use Pagination**: Request only needed results with `num` parameter
2. **Optimize Queries**: Use specific, targeted search queries
3. **Avoid Polling**: Use webhooks or event-driven approaches when possible
4. **Connection Reuse**: Use HTTP/1.1 keep-alive or HTTP/2

### Error Handling

```javascript
async function makeRequest(url, options) {
  try {
    const response = await fetch(url, options);
    
    // Check rate limit headers
    const remaining = parseInt(response.headers.get('X-RateLimit-Remaining'));
    const reset = parseInt(response.headers.get('X-RateLimit-Reset'));
    
    if (remaining < 10) {
      console.warn(`Rate limit approaching: ${remaining} requests remaining`);
    }
    
    if (response.status === 429) {
      const retryAfter = parseInt(response.headers.get('Retry-After'));
      console.log(`Rate limited. Waiting ${retryAfter} seconds...`);
      await new Promise(resolve => setTimeout(resolve, retryAfter * 1000));
      return makeRequest(url, options); // Retry
    }
    
    return response;
  } catch (error) {
    console.error('Request failed:', error);
    throw error;
  }
}
```

### Python Example

```python
import time
import requests
from typing import Optional

class TarziAPIClient:
    def __init__(self, api_key: str):
        self.api_key = api_key
        self.base_url = "https://api.tarzi.dev/v1"
        
    def make_request(self, endpoint: str, data: dict) -> Optional[dict]:
        url = f"{self.base_url}/{endpoint}"
        headers = {
            "Content-Type": "application/json",
            "X-API-KEY": self.api_key
        }
        
        response = requests.post(url, json=data, headers=headers)
        
        # Check rate limit headers
        remaining = int(response.headers.get('X-RateLimit-Remaining', 0))
        reset_time = int(response.headers.get('X-RateLimit-Reset', 0))
        
        if remaining < 10:
            print(f"Warning: Only {remaining} requests remaining")
        
        if response.status_code == 429:
            retry_after = int(response.headers.get('Retry-After', 60))
            print(f"Rate limited. Waiting {retry_after} seconds...")
            time.sleep(retry_after)
            return self.make_request(endpoint, data)  # Retry
        
        response.raise_for_status()
        return response.json()
```

## Monitoring Rate Limits

### Dashboard Metrics

Monitor these metrics in your dashboard:

- Current rate limit usage
- Quota consumption trends
- Peak usage patterns
- Rate limit violations
- Average response times

### Alerts

Set up alerts for:

- Rate limit usage > 80%
- Quota consumption > 90%
- Frequent rate limit violations
- Unusual traffic patterns

### API for Rate Limit Information

Get current rate limit status:

```bash
curl -X GET https://api.tarzi.dev/v1/rate-limit \
  -H "X-API-KEY: your-api-key"
```

Response:
```json
{
  "rate_limit": {
    "limit": 1000,
    "remaining": 856,
    "reset": 1640995200,
    "window": 3600
  },
  "quota": {
    "limit": 10000,
    "remaining": 7432,
    "reset": 1643673600,
    "period": "monthly"
  },
  "tier": "pro"
}
```

## Upgrading Limits

### Automatic Scaling

Pro and Enterprise tiers support temporary limit increases:

- **Automatic**: 2x burst for up to 1 hour per day
- **Notification**: Email alerts when burst is activated
- **Billing**: Overage charges apply for sustained usage

### Custom Limits

Contact support for:

- Higher concurrent request limits
- Custom quota arrangements
- Dedicated infrastructure
- SLA guarantees

### Migration Between Tiers

Upgrading your subscription immediately increases limits:

- **Immediate**: New limits take effect within 5 minutes
- **Prorated**: Billing is prorated for the current month
- **No Downtime**: No service interruption during upgrades

## Rate Limit Bypass

### Whitelisting

Enterprise customers can request:

- Specific endpoint whitelisting
- IP address exemptions
- Temporary limit suspension
- Custom rate limit policies

### Priority Queuing

Enterprise tier includes:

- Priority request processing
- Lower latency guarantees
- Dedicated capacity allocation
- 99.9% uptime SLA