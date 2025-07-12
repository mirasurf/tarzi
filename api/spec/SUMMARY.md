# Tarzi API Specification Summary

## Overview

This document provides a comprehensive overview of the Tarzi REST API specification, designed to be fully compatible with the Serper API while providing enhanced features and performance.

## Design Goals

### ✅ Serper API Compatibility
- **100% Compatible**: Drop-in replacement for existing Serper integrations
- **Same Request Format**: Identical JSON request structures
- **Same Response Format**: Matching response schemas and field names
- **Same Authentication**: API key-based authentication via `X-API-KEY` header
- **Same Endpoints**: All Serper endpoints supported (`/search`, `/images`, `/news`, etc.)

### ✅ Enhanced Performance
- **Sub-second Response Times**: Average 300-800ms response times
- **High Availability**: 99.9% uptime SLA
- **Global CDN**: Distributed infrastructure for optimal performance
- **Real-time Results**: No caching, fresh data for every request

### ✅ Developer Experience
- **Comprehensive Documentation**: Detailed specs for all endpoints
- **Clear Error Messages**: Structured error responses with actionable information
- **Rate Limit Transparency**: Clear headers and proactive notifications
- **Multiple SDKs**: Support for popular programming languages

## API Specification Documents

### 📖 Core Documentation

| Document | Purpose | Status |
|----------|---------|--------|
| [`README.md`](README.md) | Documentation overview and structure | ✅ Complete |
| [`api-overview.md`](api-overview.md) | High-level API design and compatibility | ✅ Complete |
| [`authentication.md`](authentication.md) | API key authentication and security | ✅ Complete |
| [`data-models.md`](data-models.md) | Request/response schemas and validation | ✅ Complete |
| [`error-handling.md`](error-handling.md) | Error codes, messages, and handling | ✅ Complete |
| [`rate-limiting.md`](rate-limiting.md) | Rate limits, quotas, and tiers | ✅ Complete |

### 🔍 Endpoint Specifications

| Endpoint | Document | Purpose | Status |
|----------|----------|---------|--------|
| `POST /search` | [`endpoints/search.md`](endpoints/search.md) | Web search with rich results | ✅ Complete |
| `POST /images` | [`endpoints/images.md`](endpoints/images.md) | Image search with filters | ✅ Complete |
| `POST /news` | [`endpoints/news.md`](endpoints/news.md) | News search with date filters | ✅ Complete |
| `POST /videos` | [`endpoints/videos.md`](endpoints/videos.md) | Video search results | 📋 Planned |
| `POST /maps` | [`endpoints/maps.md`](endpoints/maps.md) | Maps/Places search | 📋 Planned |
| `POST /shopping` | [`endpoints/shopping.md`](endpoints/shopping.md) | Shopping results | 📋 Planned |
| `POST /scholar` | [`endpoints/scholar.md`](endpoints/scholar.md) | Academic search | 📋 Planned |
| `POST /patents` | [`endpoints/patents.md`](endpoints/patents.md) | Patent search | 📋 Planned |
| `POST /autocomplete` | [`endpoints/autocomplete.md`](endpoints/autocomplete.md) | Search suggestions | 📋 Planned |

## Key Features

### 🔐 Authentication & Security
- **API Key Authentication**: Simple and secure `X-API-KEY` header
- **IP Whitelisting**: Optional IP restrictions for enhanced security
- **Rate Limiting**: Fair usage policies with clear limits
- **HTTPS Only**: All requests must use HTTPS encryption

### 📊 Comprehensive Search Types
- **Web Search**: Organic results, knowledge graph, people also ask
- **Image Search**: High-quality images with advanced filtering
- **News Search**: Real-time news with publication dates
- **Video Search**: Video results with metadata
- **Maps Search**: Location-based results with details
- **Shopping Search**: Product results with pricing
- **Academic Search**: Scholarly articles and papers
- **Patent Search**: Patent database with detailed information
- **Autocomplete**: Search suggestions and completions

### 🚀 Performance Guarantees
- **Response Time**: Sub-second average response times
- **Availability**: 99.9% uptime SLA
- **Scalability**: Auto-scaling infrastructure
- **Global Reach**: Multi-region deployment

### 💡 Developer-Friendly
- **Clear Documentation**: Comprehensive API documentation
- **Error Handling**: Structured error responses with context
- **Rate Limit Headers**: Transparent usage information
- **Code Examples**: Multiple programming language examples

## Request/Response Format

### Standard Request Structure
```json
{
  "q": "search query",
  "gl": "us",
  "hl": "en", 
  "num": 10,
  "start": 0,
  "autocorrect": true,
  "safe": "off",
  "filter": "1"
}
```

### Standard Response Structure
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
  "searchInformation": {
    "queryDisplayed": "search query",
    "totalResults": "About 1,000,000 results",
    "timeTaken": 0.52
  },
  "credits": {
    "remaining": 999,
    "used": 1,
    "resetTime": "2024-01-15T13:00:00Z"
  }
}
```

## Subscription Tiers

### 🆓 Free Tier
- **Rate Limit**: 100 requests/hour
- **Monthly Quota**: 1,000 credits
- **Burst Limit**: 10 requests/minute
- **Concurrent Requests**: 2
- **Features**: All basic endpoints

### 💼 Pro Tier
- **Rate Limit**: 1,000 requests/hour
- **Monthly Quota**: 10,000 credits
- **Burst Limit**: 50 requests/minute
- **Concurrent Requests**: 10
- **Features**: All endpoints + priority support

### 🏢 Enterprise Tier
- **Rate Limit**: 10,000 requests/hour
- **Monthly Quota**: 100,000 credits
- **Burst Limit**: 200 requests/minute
- **Concurrent Requests**: 50
- **Features**: All endpoints + SLA + custom limits

## Error Handling

### Error Response Format
```json
{
  "error": {
    "type": "validation_error",
    "message": "Human-readable error description",
    "code": "MACHINE_READABLE_CODE",
    "details": {
      "parameter": "field_name",
      "provided_value": "invalid_value"
    }
  }
}
```

### Error Categories
- **Authentication Errors** (401): Invalid or missing API keys
- **Authorization Errors** (403): Insufficient permissions or suspended keys
- **Validation Errors** (400): Invalid request parameters
- **Rate Limit Errors** (429): Quota or rate limit exceeded
- **Server Errors** (5xx): Internal server or upstream issues

## Migration from Serper

### Simple Migration Process
1. **Update Base URL**: Change from `https://google.serper.dev` to `https://api.tarzi.dev/v1`
2. **Update API Key**: Replace Serper API key with Tarzi API key
3. **Keep Everything Else**: No changes to request/response handling needed

### Before (Serper)
```javascript
const response = await fetch('https://google.serper.dev/search', {
  method: 'POST',
  headers: {
    'X-API-KEY': 'serper-api-key',
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({ q: 'search query' })
});
```

### After (Tarzi)
```javascript
const response = await fetch('https://api.tarzi.dev/v1/search', {
  method: 'POST', 
  headers: {
    'X-API-KEY': 'tarzi-api-key',
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({ q: 'search query' })
});
```

## Best Practices

### 🎯 Efficient Usage
1. **Monitor Rate Limits**: Check `X-RateLimit-*` headers
2. **Implement Retry Logic**: Use exponential backoff for server errors
3. **Cache Results**: Store responses to reduce API calls
4. **Optimize Queries**: Use specific, targeted search terms
5. **Batch Requests**: Combine multiple searches when possible

### 🛡️ Security
1. **Protect API Keys**: Never expose keys in client-side code
2. **Use Environment Variables**: Store keys securely
3. **Enable IP Restrictions**: Whitelist specific IP addresses
4. **Monitor Usage**: Watch for unusual activity patterns
5. **Rotate Keys**: Regularly update API keys

### 📈 Performance
1. **Use Pagination**: Request only needed results with `num` parameter
2. **Implement Caching**: Cache responses based on query and parameters
3. **Connection Reuse**: Use HTTP/1.1 keep-alive or HTTP/2
4. **Geographic Distribution**: Use multiple regions for global apps
5. **Monitor Metrics**: Track response times and error rates

## Getting Started

### 1. Sign Up
Create an account at [https://tarzi.dev/signup](https://tarzi.dev/signup)

### 2. Get API Key
Generate an API key from your dashboard

### 3. Make First Request
```bash
curl -X POST https://api.tarzi.dev/v1/search \
  -H "Content-Type: application/json" \
  -H "X-API-KEY: your-api-key" \
  -d '{"q": "hello world"}'
```

### 4. Handle Response
```json
{
  "searchParameters": { "q": "hello world", "gl": "us", "hl": "en" },
  "organic": [
    {
      "title": "Hello, World! - Wikipedia", 
      "link": "https://en.wikipedia.org/wiki/Hello,_world",
      "snippet": "A 'Hello, World!' program is generally the first program..."
    }
  ],
  "searchInformation": {
    "totalResults": "About 2,500,000 results",
    "timeTaken": 0.42
  }
}
```

## Support & Resources

### 📚 Documentation
- **API Reference**: Complete endpoint documentation
- **Code Examples**: Multi-language implementation examples  
- **Tutorials**: Step-by-step integration guides
- **Best Practices**: Performance and security recommendations

### 🆘 Support Channels
- **Documentation**: Comprehensive self-service docs
- **Community Forum**: Peer-to-peer support and discussions
- **Email Support**: Direct support for technical issues
- **Enterprise Support**: Dedicated support for enterprise customers

### 🔧 Developer Tools
- **API Playground**: Interactive API testing interface
- **Dashboard**: Usage monitoring and API key management
- **SDKs**: Official libraries for popular languages
- **Status Page**: Real-time service status and uptime

## Roadmap

### 🚀 Current (v1.0)
- ✅ Core search endpoints (search, images, news)
- ✅ Serper API compatibility
- ✅ Rate limiting and quotas
- ✅ Comprehensive documentation

### 🎯 Next Release (v1.1)
- 📋 Additional endpoints (videos, maps, shopping)
- 📋 Advanced filtering options
- 📋 Webhook support for real-time updates
- 📋 Enhanced analytics and reporting

### 🌟 Future (v2.0)
- 📋 GraphQL API support
- 📋 Advanced AI-powered search features
- 📋 Custom search engine integration
- 📋 Real-time collaboration features

## Conclusion

The Tarzi API specification provides a robust, scalable, and developer-friendly search API that maintains full compatibility with Serper while offering enhanced performance, comprehensive documentation, and enterprise-grade features.

The specification covers all aspects of the API including authentication, rate limiting, error handling, and detailed endpoint documentation, ensuring developers have everything they need to successfully integrate and build applications with the Tarzi search API.