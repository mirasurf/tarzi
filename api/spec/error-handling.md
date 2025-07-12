# Error Handling

## Overview

Tarzi API uses conventional HTTP response codes to indicate the success or failure of an API request. In general, codes in the 2xx range indicate success, codes in the 4xx range indicate an error with the provided information, and codes in the 5xx range indicate an error with Tarzi's servers.

## Error Response Format

All errors follow a consistent JSON structure:

```json
{
  "error": {
    "type": "error_type",
    "message": "Human-readable error description",
    "code": "MACHINE_READABLE_CODE",
    "details": {
      "additional": "context",
      "parameter": "field_name"
    }
  }
}
```

## HTTP Status Codes

### 2xx Success

| Code | Status | Description |
|------|--------|-------------|
| 200 | OK | Request successful |

### 4xx Client Errors

| Code | Status | Description |
|------|--------|-------------|
| 400 | Bad Request | The request was invalid or cannot be served |
| 401 | Unauthorized | Authentication required or invalid |
| 403 | Forbidden | Request forbidden, often due to insufficient permissions |
| 404 | Not Found | The requested resource could not be found |
| 422 | Unprocessable Entity | Request was well-formed but contains semantic errors |
| 429 | Too Many Requests | Rate limit exceeded |

### 5xx Server Errors

| Code | Status | Description |
|------|--------|-------------|
| 500 | Internal Server Error | An unexpected error occurred |
| 502 | Bad Gateway | Invalid response from upstream server |
| 503 | Service Unavailable | Server temporarily unavailable |
| 504 | Gateway Timeout | Request timeout |

## Authentication Errors (401 Unauthorized)

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

### Malformed API Key

```json
{
  "error": {
    "type": "authentication_error",
    "message": "API key format is invalid",
    "code": "MALFORMED_API_KEY"
  }
}
```

## Authorization Errors (403 Forbidden)

### Suspended API Key

```json
{
  "error": {
    "type": "authorization_error",
    "message": "API key has been suspended",
    "code": "SUSPENDED_API_KEY",
    "details": {
      "suspension_reason": "Terms of service violation",
      "contact": "support@tarzi.dev"
    }
  }
}
```

### Insufficient Permissions

```json
{
  "error": {
    "type": "authorization_error",
    "message": "API key does not have permission to access this endpoint",
    "code": "INSUFFICIENT_PERMISSIONS",
    "details": {
      "required_permission": "search:advanced",
      "current_permissions": ["search:basic"]
    }
  }
}
```

### IP Restriction

```json
{
  "error": {
    "type": "authorization_error",
    "message": "Request from unauthorized IP address",
    "code": "IP_NOT_ALLOWED",
    "details": {
      "client_ip": "203.0.113.1",
      "allowed_ips": ["192.168.1.0/24", "10.0.0.1"]
    }
  }
}
```

## Validation Errors (400 Bad Request)

### Missing Required Parameter

```json
{
  "error": {
    "type": "validation_error",
    "message": "Search query is required",
    "code": "MISSING_PARAMETER",
    "details": {
      "parameter": "q",
      "required": true
    }
  }
}
```

### Invalid Parameter Value

```json
{
  "error": {
    "type": "validation_error",
    "message": "Invalid value for parameter 'num'. Must be between 1 and 100",
    "code": "INVALID_PARAMETER_VALUE",
    "details": {
      "parameter": "num",
      "provided_value": 150,
      "valid_range": "1-100"
    }
  }
}
```

### Invalid Parameter Type

```json
{
  "error": {
    "type": "validation_error",
    "message": "Parameter 'num' must be an integer",
    "code": "INVALID_PARAMETER_TYPE",
    "details": {
      "parameter": "num",
      "expected_type": "integer",
      "provided_type": "string"
    }
  }
}
```

### Query Too Long

```json
{
  "error": {
    "type": "validation_error",
    "message": "Query exceeds maximum length of 500 characters",
    "code": "QUERY_TOO_LONG",
    "details": {
      "query_length": 567,
      "max_length": 500
    }
  }
}
```

### Invalid Country Code

```json
{
  "error": {
    "type": "validation_error",
    "message": "Invalid country code. Must be a valid ISO 3166-1 alpha-2 code",
    "code": "INVALID_COUNTRY_CODE",
    "details": {
      "parameter": "gl",
      "provided_value": "usa",
      "expected_format": "ISO 3166-1 alpha-2 (e.g., 'us', 'gb', 'de')"
    }
  }
}
```

### Invalid Language Code

```json
{
  "error": {
    "type": "validation_error",
    "message": "Invalid language code format",
    "code": "INVALID_LANGUAGE_CODE",
    "details": {
      "parameter": "hl",
      "provided_value": "english",
      "expected_format": "ISO 639-1 (e.g., 'en', 'es', 'fr')"
    }
  }
}
```

### Invalid JSON

```json
{
  "error": {
    "type": "validation_error",
    "message": "Request body contains invalid JSON",
    "code": "INVALID_JSON",
    "details": {
      "json_error": "Unexpected token '}' at position 45"
    }
  }
}
```

## Rate Limiting Errors (429 Too Many Requests)

### Rate Limit Exceeded

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
      "retry_after": 3600
    }
  }
}
```

## Quota Errors (429 Too Many Requests)

### Quota Exceeded

```json
{
  "error": {
    "type": "quota_error",
    "message": "API quota exceeded for your subscription plan",
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

### Credits Depleted

```json
{
  "error": {
    "type": "quota_error",
    "message": "No credits remaining. Please purchase additional credits",
    "code": "CREDITS_DEPLETED",
    "details": {
      "remaining_credits": 0,
      "purchase_url": "https://tarzi.dev/billing/credits"
    }
  }
}
```

## Server Errors (5xx)

### Internal Server Error (500)

```json
{
  "error": {
    "type": "internal_error",
    "message": "An unexpected error occurred. Please try again later",
    "code": "INTERNAL_SERVER_ERROR",
    "details": {
      "request_id": "req_1234567890abcdef",
      "timestamp": "2024-01-15T12:34:56Z"
    }
  }
}
```

### Service Unavailable (503)

```json
{
  "error": {
    "type": "service_unavailable",
    "message": "Service temporarily unavailable due to maintenance",
    "code": "SERVICE_UNAVAILABLE",
    "details": {
      "maintenance_end": "2024-01-15T15:00:00Z",
      "status_page": "https://status.tarzi.dev"
    }
  }
}
```

### Gateway Timeout (504)

```json
{
  "error": {
    "type": "timeout_error",
    "message": "Request timed out. Please try again",
    "code": "GATEWAY_TIMEOUT",
    "details": {
      "timeout_duration": "30s",
      "retry_suggested": true
    }
  }
}
```

### Search Engine Error (502)

```json
{
  "error": {
    "type": "upstream_error",
    "message": "Search engine temporarily unavailable",
    "code": "SEARCH_ENGINE_ERROR",
    "details": {
      "engine": "google",
      "retry_after": 60
    }
  }
}
```

## Endpoint-Specific Errors

### Search Endpoint Errors

#### No Results Found

```json
{
  "error": {
    "type": "search_error",
    "message": "No results found for the given query",
    "code": "NO_RESULTS_FOUND",
    "details": {
      "query": "very specific search that returns nothing",
      "suggestions": [
        "Try different keywords",
        "Check spelling",
        "Use broader terms"
      ]
    }
  }
}
```

### Image Search Errors

#### Invalid Image Filter

```json
{
  "error": {
    "type": "validation_error",
    "message": "Invalid value for image filter parameter",
    "code": "INVALID_IMAGE_FILTER",
    "details": {
      "parameter": "imgSize",
      "provided_value": "huge",
      "valid_values": ["large", "medium", "small", "icon"]
    }
  }
}
```

### News Search Errors

#### Invalid Time Range

```json
{
  "error": {
    "type": "validation_error",
    "message": "Invalid time-based search parameter",
    "code": "INVALID_TIME_RANGE",
    "details": {
      "parameter": "tbs",
      "provided_value": "qdr:invalid",
      "valid_values": ["qdr:h", "qdr:d", "qdr:w", "qdr:m", "qdr:y"]
    }
  }
}
```

## Error Response Headers

Error responses include relevant headers:

```http
HTTP/1.1 400 Bad Request
Content-Type: application/json; charset=utf-8
X-Request-ID: req_1234567890abcdef
X-Error-Code: INVALID_PARAMETER_VALUE
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1640995200
```

## Handling Errors

### Retry Logic

Different error types require different retry strategies:

#### Retryable Errors (with backoff)
- `500 Internal Server Error`
- `502 Bad Gateway`
- `503 Service Unavailable`
- `504 Gateway Timeout`

#### Non-Retryable Errors
- `400 Bad Request` (fix request first)
- `401 Unauthorized` (check API key)
- `403 Forbidden` (check permissions)
- `422 Unprocessable Entity` (fix request format)

#### Rate Limit Errors
- `429 Too Many Requests` (wait for reset time)

### Example Error Handling (JavaScript)

```javascript
async function handleApiError(response) {
  const errorData = await response.json();
  const { error } = errorData;
  
  switch (error.code) {
    case 'RATE_LIMIT_EXCEEDED':
      const retryAfter = error.details.retry_after;
      console.log(`Rate limited. Retry after ${retryAfter} seconds`);
      await new Promise(resolve => setTimeout(resolve, retryAfter * 1000));
      return 'retry';
      
    case 'INVALID_API_KEY':
      console.error('Invalid API key. Please check your credentials.');
      return 'stop';
      
    case 'QUOTA_EXCEEDED':
      console.error('Quota exceeded. Please upgrade your plan.');
      return 'stop';
      
    case 'INTERNAL_SERVER_ERROR':
    case 'SERVICE_UNAVAILABLE':
      console.log('Server error. Retrying with backoff...');
      return 'retry_with_backoff';
      
    default:
      console.error(`API Error: ${error.message}`);
      return 'stop';
  }
}
```

### Example Error Handling (Python)

```python
import time
import requests
from typing import Dict, Any

def handle_api_error(response: requests.Response) -> str:
    """Handle API errors and return retry strategy."""
    try:
        error_data = response.json()
        error = error_data.get('error', {})
        error_code = error.get('code')
        
        if error_code == 'RATE_LIMIT_EXCEEDED':
            retry_after = error.get('details', {}).get('retry_after', 60)
            print(f"Rate limited. Waiting {retry_after} seconds...")
            time.sleep(retry_after)
            return 'retry'
            
        elif error_code in ['INVALID_API_KEY', 'QUOTA_EXCEEDED']:
            print(f"Fatal error: {error.get('message')}")
            return 'stop'
            
        elif error_code in ['INTERNAL_SERVER_ERROR', 'SERVICE_UNAVAILABLE']:
            print("Server error. Will retry with backoff...")
            return 'retry_with_backoff'
            
        else:
            print(f"API Error: {error.get('message')}")
            return 'stop'
            
    except ValueError:
        print("Invalid JSON in error response")
        return 'stop'
```

## Best Practices

### Error Handling Best Practices

1. **Always check HTTP status codes** before processing responses
2. **Implement exponential backoff** for retryable errors
3. **Respect rate limit headers** to avoid unnecessary errors
4. **Log error details** for debugging and monitoring
5. **Handle network timeouts** gracefully
6. **Provide meaningful error messages** to end users
7. **Monitor error rates** to detect issues early

### Request Validation

To minimize validation errors:

1. **Validate parameters client-side** before sending requests
2. **Use proper data types** for all parameters
3. **Check parameter ranges** and formats
4. **Sanitize user input** before including in requests
5. **Use URL encoding** for query parameters with special characters

### Monitoring and Alerting

Set up monitoring for:

- Error rates by endpoint
- Authentication failures
- Rate limit violations
- Server error frequency
- Response time anomalies

### Error Logging

Include these details in error logs:

- Request ID (from `X-Request-ID` header)
- Timestamp
- HTTP status code
- Error code and message
- Request parameters (excluding sensitive data)
- User/API key identifier (anonymized)