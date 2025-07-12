# Tarzi API Overview

## Introduction

Tarzi is a high-performance search API service designed to be fully compatible with the Serper API interface. This ensures seamless migration and integration for applications currently using Serper.

## Base URL

```
https://api.tarzi.dev/v1
```

## Serper API Compatibility

Tarzi maintains 100% compatibility with Serper API in terms of:
- Request/response formats
- HTTP methods and status codes
- Authentication mechanisms
- Error response structures
- Rate limiting headers

### Supported Endpoints

| Endpoint | Purpose | Serper Compatible |
|----------|---------|-------------------|
| `POST /search` | Web search results | ✅ |
| `POST /images` | Image search results | ✅ |
| `POST /news` | News search results | ✅ |
| `POST /videos` | Video search results | ✅ |
| `POST /maps` | Maps/Places search results | ✅ |
| `POST /shopping` | Shopping search results | ✅ |
| `POST /scholar` | Scholar search results | ✅ |
| `POST /patents` | Patent search results | ✅ |
| `POST /autocomplete` | Search autocomplete | ✅ |

## Common Request Structure

All endpoints follow the same request pattern:

```http
POST /{endpoint}
Content-Type: application/json
X-API-KEY: {your-api-key}

{
  "q": "search query",
  "gl": "country_code",
  "hl": "language_code",
  "num": 10,
  "start": 0,
  "autocorrect": true,
  "page": 1,
  "type": "search",
  "device": "desktop"
}
```

## Common Response Structure

All successful responses include:

```json
{
  "searchParameters": {
    "q": "search query",
    "gl": "us",
    "hl": "en",
    "num": 10,
    "type": "search",
    "engine": "google"
  },
  "organic": [...],
  "peopleAlsoAsk": [...],
  "relatedSearches": [...],
  "searchInformation": {
    "queryDisplayed": "search query",
    "totalResults": "About 1,000,000 results",
    "timeTaken": 0.85,
    "formattedTotalResults": "1,000,000",
    "formattedTimeTaken": "0.85 seconds"
  }
}
```

## Authentication

Tarzi uses API key authentication via the `X-API-KEY` header:

```http
X-API-KEY: your-api-key-here
```

## Rate Limiting

Standard rate limits apply:
- Free tier: 100 requests/hour
- Pro tier: 1,000 requests/hour  
- Enterprise tier: 10,000 requests/hour

Rate limit information is returned in response headers:
- `X-RateLimit-Limit`
- `X-RateLimit-Remaining`
- `X-RateLimit-Reset`

## Error Handling

Errors follow HTTP status codes with JSON error responses:

```json
{
  "error": {
    "type": "authentication_error",
    "message": "Invalid API key",
    "code": "INVALID_API_KEY"
  }
}
```

## Localization

Tarzi supports the same localization parameters as Serper:
- `gl`: Country code (e.g., "us", "gb", "de")
- `hl`: Language code (e.g., "en", "es", "fr")
- `lr`: Language restriction

## Search Parameters

Common parameters supported across all endpoints:
- `q`: Search query (required)
- `num`: Number of results (1-100, default: 10)
- `start`: Start index for pagination (default: 0)
- `gl`: Geographic location
- `hl`: Interface language
- `lr`: Language restriction
- `safe`: Safe search filter
- `filter`: Duplicate filtering
- `autocorrect`: Query autocorrection

## Response Times

Tarzi aims for sub-second response times:
- Average: 300-800ms
- 95th percentile: <1.5s
- 99th percentile: <3s

## Data Freshness

All search results are fetched in real-time with no caching, ensuring the most current information available.