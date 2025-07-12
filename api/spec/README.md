# Tarzi API Specification

This directory contains the REST API specification for Tarzi, designed to be compatible with the Serper API interface.

## Documentation Structure

- `api-overview.md` - High-level API overview and compatibility notes
- `authentication.md` - Authentication and authorization mechanisms
- `endpoints/` - Individual endpoint specifications
  - `search.md` - Web search endpoint
  - `images.md` - Image search endpoint
  - `news.md` - News search endpoint
  - `videos.md` - Video search endpoint
  - `maps.md` - Maps/Places search endpoint
  - `shopping.md` - Shopping search endpoint
  - `scholar.md` - Scholar search endpoint
  - `patents.md` - Patent search endpoint
  - `autocomplete.md` - Autocomplete endpoint
- `data-models.md` - Request and response data models
- `error-handling.md` - Error response specifications
- `rate-limiting.md` - Rate limiting and quota specifications

## Design Principles

1. **Serper API Compatibility**: Maintain full compatibility with Serper API request/response formats
2. **RESTful Design**: Follow REST principles for consistent API design
3. **JSON-First**: Use JSON for all request and response payloads
4. **HTTP Standards**: Proper use of HTTP methods, status codes, and headers
5. **Extensibility**: Design for future enhancements while maintaining backward compatibility

## Version

Current API Version: `v1`

Base URL: `https://api.tarzi.dev/v1`