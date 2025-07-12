# Data Models

## Overview

This document defines the common data models used across all Tarzi API endpoints. All models follow JSON Schema standards and maintain compatibility with Serper API formats.

## Common Request Models

### Base Search Request

```json
{
  "type": "object",
  "properties": {
    "q": {
      "type": "string",
      "description": "Search query",
      "minLength": 1,
      "maxLength": 500,
      "required": true
    },
    "gl": {
      "type": "string",
      "description": "Country code (ISO 3166-1 alpha-2)",
      "pattern": "^[a-z]{2}$",
      "default": "us"
    },
    "hl": {
      "type": "string",
      "description": "Interface language",
      "pattern": "^[a-z]{2}(-[A-Z]{2})?$",
      "default": "en"
    },
    "num": {
      "type": "integer",
      "description": "Number of results",
      "minimum": 1,
      "maximum": 100,
      "default": 10
    },
    "start": {
      "type": "integer",
      "description": "Start index for pagination",
      "minimum": 0,
      "default": 0
    },
    "autocorrect": {
      "type": "boolean",
      "description": "Enable query autocorrection",
      "default": true
    },
    "safe": {
      "type": "string",
      "description": "Safe search filter",
      "enum": ["off", "moderate", "strict"],
      "default": "off"
    },
    "filter": {
      "type": "string",
      "description": "Duplicate filtering",
      "enum": ["0", "1"],
      "default": "1"
    },
    "lr": {
      "type": "string",
      "description": "Language restriction",
      "nullable": true
    },
    "tbs": {
      "type": "string",
      "description": "Time-based search parameters",
      "nullable": true
    }
  },
  "required": ["q"]
}
```

### Image Search Request

```json
{
  "allOf": [
    {
      "$ref": "#/components/schemas/BaseSearchRequest"
    },
    {
      "type": "object",
      "properties": {
        "imgSize": {
          "type": "string",
          "description": "Image size filter",
          "enum": ["large", "medium", "small", "icon"],
          "nullable": true
        },
        "imgType": {
          "type": "string",
          "description": "Image type filter",
          "enum": ["photo", "clipart", "lineart", "face"],
          "nullable": true
        },
        "imgColorType": {
          "type": "string",
          "description": "Color type filter",
          "enum": ["color", "gray", "mono", "trans"],
          "nullable": true
        },
        "imgDominantColor": {
          "type": "string",
          "description": "Dominant color filter",
          "enum": ["black", "blue", "brown", "gray", "green", "orange", "pink", "purple", "red", "teal", "white", "yellow"],
          "nullable": true
        },
        "imgUsageRights": {
          "type": "string",
          "description": "Usage rights filter",
          "enum": ["fmc", "fc", "fm", "f", "cc"],
          "nullable": true
        }
      }
    }
  ]
}
```

### News Search Request

```json
{
  "allOf": [
    {
      "$ref": "#/components/schemas/BaseSearchRequest"
    },
    {
      "type": "object",
      "properties": {
        "sort": {
          "type": "string",
          "description": "Sort order",
          "enum": ["relevance", "date"],
          "default": "relevance"
        },
        "tbs": {
          "type": "string",
          "description": "Time-based search parameters",
          "enum": ["qdr:h", "qdr:d", "qdr:w", "qdr:m", "qdr:y"],
          "nullable": true
        }
      }
    }
  ]
}
```

## Common Response Models

### Base Search Response

```json
{
  "type": "object",
  "properties": {
    "searchParameters": {
      "$ref": "#/components/schemas/SearchParameters"
    },
    "searchInformation": {
      "$ref": "#/components/schemas/SearchInformation"
    },
    "credits": {
      "$ref": "#/components/schemas/CreditsInfo"
    }
  },
  "required": ["searchParameters", "searchInformation", "credits"]
}
```

### Search Parameters

```json
{
  "type": "object",
  "properties": {
    "q": {
      "type": "string",
      "description": "Processed search query"
    },
    "gl": {
      "type": "string",
      "description": "Applied country code"
    },
    "hl": {
      "type": "string",
      "description": "Applied interface language"
    },
    "num": {
      "type": "integer",
      "description": "Number of results requested"
    },
    "start": {
      "type": "integer",
      "description": "Start index used"
    },
    "autocorrect": {
      "type": "boolean",
      "description": "Autocorrection enabled"
    },
    "safe": {
      "type": "string",
      "description": "Safe search filter applied"
    },
    "filter": {
      "type": "string",
      "description": "Duplicate filtering applied"
    },
    "type": {
      "type": "string",
      "description": "Search type",
      "enum": ["search", "images", "news", "videos", "maps", "shopping", "scholar", "patents", "autocomplete"]
    },
    "engine": {
      "type": "string",
      "description": "Search engine used",
      "enum": ["google"]
    }
  },
  "required": ["q", "gl", "hl", "num", "start", "type", "engine"]
}
```

### Search Information

```json
{
  "type": "object",
  "properties": {
    "queryDisplayed": {
      "type": "string",
      "description": "Query as displayed in results"
    },
    "totalResults": {
      "type": "string",
      "description": "Total number of results (formatted)"
    },
    "timeTaken": {
      "type": "number",
      "description": "Time taken to process request (seconds)"
    },
    "formattedTotalResults": {
      "type": "string",
      "description": "Formatted total results count"
    },
    "formattedTimeTaken": {
      "type": "string",
      "description": "Formatted time taken"
    }
  },
  "required": ["queryDisplayed", "totalResults", "timeTaken"]
}
```

### Credits Information

```json
{
  "type": "object",
  "properties": {
    "remaining": {
      "type": "integer",
      "description": "Remaining credits after this request"
    },
    "used": {
      "type": "integer",
      "description": "Credits used for this request"
    },
    "resetTime": {
      "type": "string",
      "format": "date-time",
      "description": "When credits reset (ISO 8601 format)"
    }
  },
  "required": ["remaining", "used", "resetTime"]
}
```

## Search-Specific Response Models

### Organic Search Result

```json
{
  "type": "object",
  "properties": {
    "title": {
      "type": "string",
      "description": "Page title"
    },
    "link": {
      "type": "string",
      "format": "uri",
      "description": "Page URL"
    },
    "snippet": {
      "type": "string",
      "description": "Page description/snippet"
    },
    "sitelinks": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/Sitelink"
      },
      "description": "Additional page links"
    },
    "position": {
      "type": "integer",
      "description": "Result position (1-based)"
    },
    "date": {
      "type": "string",
      "description": "Publication date (if available)"
    },
    "attributes": {
      "type": "object",
      "additionalProperties": {
        "type": "string"
      },
      "description": "Structured data attributes"
    }
  },
  "required": ["title", "link", "snippet", "position"]
}
```

### Sitelink

```json
{
  "type": "object",
  "properties": {
    "title": {
      "type": "string",
      "description": "Sitelink title"
    },
    "link": {
      "type": "string",
      "format": "uri",
      "description": "Sitelink URL"
    }
  },
  "required": ["title", "link"]
}
```

### Knowledge Graph

```json
{
  "type": "object",
  "properties": {
    "title": {
      "type": "string",
      "description": "Entity name"
    },
    "type": {
      "type": "string",
      "description": "Entity type/category"
    },
    "website": {
      "type": "string",
      "format": "uri",
      "description": "Official website"
    },
    "imageUrl": {
      "type": "string",
      "format": "uri",
      "description": "Entity image URL"
    },
    "description": {
      "type": "string",
      "description": "Entity description"
    },
    "descriptionSource": {
      "type": "string",
      "description": "Source of description"
    },
    "descriptionLink": {
      "type": "string",
      "format": "uri",
      "description": "Link to description source"
    },
    "attributes": {
      "type": "object",
      "additionalProperties": {
        "type": "string"
      },
      "description": "Entity facts and attributes"
    }
  },
  "required": ["title", "type"]
}
```

### People Also Ask

```json
{
  "type": "object",
  "properties": {
    "question": {
      "type": "string",
      "description": "The question text"
    },
    "snippet": {
      "type": "string",
      "description": "Answer snippet"
    },
    "title": {
      "type": "string",
      "description": "Source title"
    },
    "link": {
      "type": "string",
      "format": "uri",
      "description": "Source link"
    }
  },
  "required": ["question", "snippet", "title", "link"]
}
```

### Related Search

```json
{
  "type": "object",
  "properties": {
    "query": {
      "type": "string",
      "description": "Related search term"
    }
  },
  "required": ["query"]
}
```

## Image-Specific Response Models

### Image Result

```json
{
  "type": "object",
  "properties": {
    "title": {
      "type": "string",
      "description": "Image title or alt text"
    },
    "imageUrl": {
      "type": "string",
      "format": "uri",
      "description": "Full-size image URL"
    },
    "imageWidth": {
      "type": "integer",
      "description": "Full-size image width in pixels"
    },
    "imageHeight": {
      "type": "integer",
      "description": "Full-size image height in pixels"
    },
    "thumbnailUrl": {
      "type": "string",
      "format": "uri",
      "description": "Thumbnail image URL"
    },
    "thumbnailWidth": {
      "type": "integer",
      "description": "Thumbnail width in pixels"
    },
    "thumbnailHeight": {
      "type": "integer",
      "description": "Thumbnail height in pixels"
    },
    "source": {
      "type": "string",
      "description": "Image source/website name"
    },
    "domain": {
      "type": "string",
      "description": "Domain of the source website"
    },
    "link": {
      "type": "string",
      "format": "uri",
      "description": "URL of the page containing the image"
    },
    "googleUrl": {
      "type": "string",
      "format": "uri",
      "description": "Google Images URL for this result"
    },
    "position": {
      "type": "integer",
      "description": "Result position (1-based)"
    },
    "snippet": {
      "type": "string",
      "description": "Image description or caption"
    },
    "isProduct": {
      "type": "boolean",
      "description": "Whether this is a product image"
    }
  },
  "required": ["title", "imageUrl", "thumbnailUrl", "position"]
}
```

## News-Specific Response Models

### News Result

```json
{
  "type": "object",
  "properties": {
    "title": {
      "type": "string",
      "description": "Article headline"
    },
    "link": {
      "type": "string",
      "format": "uri",
      "description": "URL to the full article"
    },
    "snippet": {
      "type": "string",
      "description": "Article excerpt/summary"
    },
    "date": {
      "type": "string",
      "description": "Publication date in relative format"
    },
    "source": {
      "type": "string",
      "description": "Publication name"
    },
    "imageUrl": {
      "type": "string",
      "format": "uri",
      "description": "Article thumbnail image URL"
    },
    "position": {
      "type": "integer",
      "description": "Result position (1-based)"
    }
  },
  "required": ["title", "link", "snippet", "date", "source", "position"]
}
```

## Error Response Models

### Error Response

```json
{
  "type": "object",
  "properties": {
    "error": {
      "$ref": "#/components/schemas/ErrorDetail"
    }
  },
  "required": ["error"]
}
```

### Error Detail

```json
{
  "type": "object",
  "properties": {
    "type": {
      "type": "string",
      "description": "Error type",
      "enum": [
        "authentication_error",
        "authorization_error",
        "validation_error",
        "quota_error",
        "rate_limit_error",
        "internal_error",
        "service_unavailable"
      ]
    },
    "message": {
      "type": "string",
      "description": "Human-readable error message"
    },
    "code": {
      "type": "string",
      "description": "Machine-readable error code"
    },
    "details": {
      "type": "object",
      "description": "Additional error details",
      "additionalProperties": true
    }
  },
  "required": ["type", "message", "code"]
}
```

## Validation Rules

### Query Validation

- `q` (query): Required, 1-500 characters
- `gl` (country): Optional, 2-letter ISO country code
- `hl` (language): Optional, 2-letter language code with optional region
- `num` (results): Optional, 1-100
- `start` (pagination): Optional, 0 or positive integer
- `safe` (safe search): Optional, one of: "off", "moderate", "strict"
- `filter` (duplicate filtering): Optional, "0" or "1"

### Image-Specific Validation

- `imgSize`: Optional, one of: "large", "medium", "small", "icon"
- `imgType`: Optional, one of: "photo", "clipart", "lineart", "face"
- `imgColorType`: Optional, one of: "color", "gray", "mono", "trans"
- `imgDominantColor`: Optional, valid color name
- `imgUsageRights`: Optional, valid usage rights code

### News-Specific Validation

- `sort`: Optional, one of: "relevance", "date"
- `tbs`: Optional, valid time-based search parameter

## Response Headers

### Standard Headers

All responses include these headers:

```http
Content-Type: application/json; charset=utf-8
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1640995200
X-Request-ID: req_1234567890abcdef
```

### Rate Limiting Headers

- `X-RateLimit-Limit`: Total requests allowed per time window
- `X-RateLimit-Remaining`: Remaining requests in current window
- `X-RateLimit-Reset`: Unix timestamp when rate limit resets
- `X-RateLimit-Window`: Time window in seconds

### Request Tracking

- `X-Request-ID`: Unique identifier for the request
- `X-Response-Time`: Time taken to process request (milliseconds)