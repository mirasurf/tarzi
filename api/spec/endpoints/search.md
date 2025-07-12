# Web Search Endpoint

## Overview

The web search endpoint returns comprehensive search results from Google, including organic results, knowledge graph, people also ask, related searches, and more.

## Endpoint

```
POST /search
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
  "page": 1,
  "type": "search",
  "device": "desktop",
  "safe": "off",
  "filter": "1",
  "tbs": null,
  "lr": null
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
| `page` | integer | No | 1 | Page number (alternative to start) |
| `type` | string | No | "search" | Search type identifier |
| `device` | string | No | "desktop" | Device type (desktop, mobile, tablet) |
| `safe` | string | No | "off" | Safe search filter (off, moderate, strict) |
| `filter` | string | No | "1" | Duplicate filtering (0, 1) |
| `tbs` | string | No | null | Time-based search parameters |
| `lr` | string | No | null | Language restriction |

## Response Format

### Success Response (200 OK)

```json
{
  "searchParameters": {
    "q": "apple inc",
    "gl": "us",
    "hl": "en",
    "num": 10,
    "start": 0,
    "autocorrect": true,
    "page": 1,
    "type": "search",
    "device": "desktop",
    "safe": "off",
    "filter": "1",
    "engine": "google"
  },
  "organic": [
    {
      "title": "Apple Inc. - Official Website",
      "link": "https://www.apple.com/",
      "snippet": "Discover the innovative world of Apple and shop everything iPhone, iPad, Apple Watch, Mac, and Apple TV, plus explore accessories, entertainment, and expert device support.",
      "sitelinks": [
        {
          "title": "iPhone",
          "link": "https://www.apple.com/iphone/"
        },
        {
          "title": "iPad",
          "link": "https://www.apple.com/ipad/"
        }
      ],
      "position": 1,
      "date": "2024-01-15",
      "attributes": {
        "Founded": "1976",
        "Headquarters": "Cupertino, California"
      }
    }
  ],
  "knowledgeGraph": {
    "title": "Apple Inc.",
    "type": "Technology company",
    "website": "https://www.apple.com/",
    "imageUrl": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQwGQRv5n7ZWZ5TL7mOzLcvCqOCLAUzNEgGzA&usqp=CAU",
    "description": "Apple Inc. is an American multinational technology company that specializes in consumer electronics, software and online services.",
    "descriptionSource": "Wikipedia",
    "descriptionLink": "https://en.wikipedia.org/wiki/Apple_Inc.",
    "attributes": {
      "Founded": "April 1, 1976",
      "Founders": "Steve Jobs, Steve Wozniak, Ronald Wayne",
      "CEO": "Tim Cook",
      "Headquarters": "Cupertino, California, United States",
      "Revenue": "$394.3 billion (2022)"
    }
  },
  "peopleAlsoAsk": [
    {
      "question": "What does Apple Inc do?",
      "snippet": "Apple Inc. designs, manufactures, and markets smartphones, personal computers, tablets, wearables, and accessories worldwide.",
      "title": "Apple Inc. - Business Overview",
      "link": "https://www.apple.com/about/"
    },
    {
      "question": "Who founded Apple Inc?",
      "snippet": "Apple Inc. was founded by Steve Jobs, Steve Wozniak, and Ronald Wayne in April 1976.",
      "title": "Apple Inc. - History",
      "link": "https://en.wikipedia.org/wiki/Apple_Inc."
    }
  ],
  "relatedSearches": [
    {
      "query": "Apple Inc stock price"
    },
    {
      "query": "Apple Inc products"
    },
    {
      "query": "Apple Inc history"
    },
    {
      "query": "Apple Inc revenue"
    }
  ],
  "searchInformation": {
    "queryDisplayed": "apple inc",
    "totalResults": "About 2,340,000,000 results",
    "timeTaken": 0.52,
    "formattedTotalResults": "2,340,000,000",
    "formattedTimeTaken": "0.52 seconds"
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
- `engine`: Search engine used (always "google")

#### Organic Results
- `organic`: Array of organic search results
- `title`: Page title
- `link`: Page URL
- `snippet`: Page description/snippet
- `sitelinks`: Array of additional page links
- `position`: Result position (1-based)
- `date`: Publication date (if available)
- `attributes`: Key-value pairs of structured data

#### Knowledge Graph
- `knowledgeGraph`: Rich information panel (when available)
- `title`: Entity name
- `type`: Entity type/category
- `website`: Official website
- `imageUrl`: Entity image
- `description`: Entity description
- `descriptionSource`: Source of description
- `descriptionLink`: Link to description source
- `attributes`: Key-value pairs of entity facts

#### People Also Ask
- `peopleAlsoAsk`: Array of related questions
- `question`: The question text
- `snippet`: Answer snippet
- `title`: Source title
- `link`: Source link

#### Related Searches
- `relatedSearches`: Array of related search queries
- `query`: Related search term

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

### Basic Search

**Request:**
```bash
curl -X POST https://api.tarzi.dev/v1/search \
  -H "Content-Type: application/json" \
  -H "X-API-KEY: your-api-key" \
  -d '{
    "q": "artificial intelligence"
  }'
```

### Search with Localization

**Request:**
```bash
curl -X POST https://api.tarzi.dev/v1/search \
  -H "Content-Type: application/json" \
  -H "X-API-KEY: your-api-key" \
  -d '{
    "q": "weather today",
    "gl": "gb",
    "hl": "en"
  }'
```

### Paginated Search

**Request:**
```bash
curl -X POST https://api.tarzi.dev/v1/search \
  -H "Content-Type: application/json" \
  -H "X-API-KEY: your-api-key" \
  -d '{
    "q": "programming tutorials",
    "num": 20,
    "start": 20
  }'
```

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

### Query Too Long (400 Bad Request)

```json
{
  "error": {
    "type": "validation_error",
    "message": "Query exceeds maximum length of 500 characters",
    "code": "QUERY_TOO_LONG"
  }
}
```

### Invalid Parameters (400 Bad Request)

```json
{
  "error": {
    "type": "validation_error",
    "message": "Invalid value for parameter 'num'. Must be between 1 and 100",
    "code": "INVALID_PARAMETER"
  }
}
```

## Rate Limiting

The search endpoint is subject to the standard rate limits:
- Headers: `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset`
- Quota exceeded returns `429 Too Many Requests`

## Performance

- **Average Response Time**: 300-800ms
- **95th Percentile**: <1.5s
- **99th Percentile**: <3s
- **Availability**: 99.9% uptime SLA

## Notes

- Results are fetched in real-time with no caching
- Knowledge graph and people also ask may not be available for all queries
- Sitelinks are included when available from the search engine
- Position numbers are 1-based for consistency with other search APIs