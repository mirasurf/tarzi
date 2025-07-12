# News Search Endpoint

## Overview

The news search endpoint returns current news articles from Google News, including headlines, snippets, source information, publication dates, and thumbnails.

## Endpoint

```
POST /news
```

## Request Format

### Headers

```http
Content-Type: application/json
X-API-KEY: your-api-key-here
```

### Request Body

```json
{
  "q": "search query",
  "gl": "us",
  "hl": "en",
  "num": 10,
  "start": 0,
  "autocorrect": true,
  "safe": "off",
  "filter": "1",
  "sort": "relevance",
  "tbs": "qdr:d"
}
```

### Parameters

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `q` | string | Yes | - | Search query |
| `gl` | string | No | "us" | Country code (ISO 3166-1 alpha-2) |
| `hl` | string | No | "en" | Interface language |
| `num` | integer | No | 10 | Number of results (1-100) |
| `start` | integer | No | 0 | Start index for pagination |
| `autocorrect` | boolean | No | true | Enable query autocorrection |
| `safe` | string | No | "off" | Safe search filter (off, moderate, strict) |
| `filter` | string | No | "1" | Duplicate filtering (0, 1) |
| `sort` | string | No | "relevance" | Sort order (relevance, date) |
| `tbs` | string | No | null | Time-based search parameters (qdr:h, qdr:d, qdr:w, qdr:m, qdr:y) |

## Response Format

### Success Response (200 OK)

```json
{
  "searchParameters": {
    "q": "artificial intelligence",
    "gl": "us",
    "hl": "en",
    "num": 10,
    "start": 0,
    "autocorrect": true,
    "safe": "off",
    "filter": "1",
    "sort": "relevance",
    "tbs": "qdr:d",
    "type": "news",
    "engine": "google"
  },
  "news": [
    {
      "title": "OpenAI Announces New AI Breakthrough in Language Processing",
      "link": "https://example.com/news/openai-ai-breakthrough-2024",
      "snippet": "OpenAI has unveiled a significant advancement in artificial intelligence that could revolutionize how machines understand and process human language...",
      "date": "2 hours ago",
      "source": "Tech News Daily",
      "imageUrl": "https://example.com/images/openai-announcement.jpg",
      "position": 1
    },
    {
      "title": "Major Tech Companies Invest $10 Billion in AI Research",
      "link": "https://businessnews.com/tech-companies-ai-investment",
      "snippet": "A consortium of leading technology companies has announced a collective investment of $10 billion in artificial intelligence research and development...",
      "date": "4 hours ago",
      "source": "Business News",
      "imageUrl": "https://businessnews.com/images/ai-investment.jpg",
      "position": 2
    },
    {
      "title": "AI Ethics Committee Releases New Guidelines for Responsible AI",
      "link": "https://ethicstoday.org/ai-guidelines-2024",
      "snippet": "The International AI Ethics Committee has published comprehensive guidelines aimed at ensuring the responsible development and deployment of artificial intelligence...",
      "date": "6 hours ago",
      "source": "Ethics Today",
      "imageUrl": "https://ethicstoday.org/images/ai-ethics.jpg",
      "position": 3
    }
  ],
  "searchInformation": {
    "queryDisplayed": "artificial intelligence",
    "totalResults": "About 12,400,000 results",
    "timeTaken": 0.28,
    "formattedTotalResults": "12,400,000",
    "formattedTimeTaken": "0.28 seconds"
  },
  "credits": {
    "remaining": 999,
    "used": 1,
    "resetTime": "2024-01-15T13:00:00Z"
  }
}
```

### Response Fields

#### Search Parameters
- `searchParameters`: Object containing the processed search parameters
- `q`: The search query as processed
- `gl`, `hl`, `num`, etc.: Applied search parameters
- `type`: Search type (always "news")
- `engine`: Search engine used (always "google")

#### News
- `news`: Array of news article results
- `title`: Article headline
- `link`: URL to the full article
- `snippet`: Article excerpt/summary
- `date`: Publication date in relative format (e.g., "2 hours ago")
- `source`: Publication name
- `imageUrl`: Article thumbnail image URL
- `position`: Result position (1-based)

#### Search Information
- `searchInformation`: Metadata about the search
- `queryDisplayed`: The query as displayed
- `totalResults`: Total number of results
- `timeTaken`: Time taken to process (seconds)
- `formattedTotalResults`: Formatted total results
- `formattedTimeTaken`: Formatted time taken

#### Credits
- `credits`: Information about API usage
- `remaining`: Remaining credits
- `used`: Credits used for this request
- `resetTime`: When credits reset

## Examples

### Basic News Search

**Request:**
```bash
curl -X POST https://api.tarzi.dev/v1/news \
  -H "Content-Type: application/json" \
  -H "X-API-KEY: your-api-key" \
  -d '{
    "q": "climate change"
  }'
```

### Recent News Search

**Request:**
```bash
curl -X POST https://api.tarzi.dev/v1/news \
  -H "Content-Type: application/json" \
  -H "X-API-KEY: your-api-key" \
  -d '{
    "q": "stock market",
    "tbs": "qdr:h",
    "sort": "date"
  }'
```

### Localized News Search

**Request:**
```bash
curl -X POST https://api.tarzi.dev/v1/news \
  -H "Content-Type: application/json" \
  -H "X-API-KEY: your-api-key" \
  -d '{
    "q": "brexit news",
    "gl": "gb",
    "hl": "en",
    "num": 20
  }'
```

## Sort Options

| Value | Description |
|-------|-------------|
| `relevance` | Sort by relevance to search query |
| `date` | Sort by publication date (newest first) |

## Time-Based Search Parameters

| Value | Description |
|-------|-------------|
| `qdr:h` | Past hour |
| `qdr:d` | Past day |
| `qdr:w` | Past week |
| `qdr:m` | Past month |
| `qdr:y` | Past year |

## Error Responses

### Invalid Query (400 Bad Request)

```json
{
  "error": {
    "type": "validation_error",
    "message": "Search query is required",
    "code": "MISSING_QUERY"
  }
}
```

### Invalid Time Parameter (400 Bad Request)

```json
{
  "error": {
    "type": "validation_error",
    "message": "Invalid value for parameter 'tbs'. Must be one of: qdr:h, qdr:d, qdr:w, qdr:m, qdr:y",
    "code": "INVALID_PARAMETER"
  }
}
```

### Invalid Sort Parameter (400 Bad Request)

```json
{
  "error": {
    "type": "validation_error",
    "message": "Invalid value for parameter 'sort'. Must be one of: relevance, date",
    "code": "INVALID_PARAMETER"
  }
}
```

## Rate Limiting

The news endpoint is subject to the standard rate limits:
- Headers: `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset`
- Quota exceeded returns `429 Too Many Requests`

## Performance

- **Average Response Time**: 350-750ms
- **95th Percentile**: <1.4s
- **99th Percentile**: <3s
- **Availability**: 99.9% uptime SLA

## Notes

- News results are fetched in real-time with no caching
- Publication dates are provided in relative format for better UX
- Image URLs may be temporary and subject to change
- Some news sources may not be available in all regions
- Breaking news may appear with very recent timestamps
- The availability of images depends on the source publication
- Safe search filtering applies to news content appropriateness