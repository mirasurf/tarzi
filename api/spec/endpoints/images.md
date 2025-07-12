# Images Search Endpoint

## Overview

The images search endpoint returns image search results from Google Images, including image URLs, thumbnails, source information, and metadata.

## Endpoint

```
POST /images
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
  "imgSize": "large",
  "imgType": "photo",
  "imgColorType": "color",
  "imgDominantColor": null,
  "imgUsageRights": null,
  "tbs": null
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
| `imgSize` | string | No | null | Image size (large, medium, small, icon) |
| `imgType` | string | No | null | Image type (photo, clipart, lineart, face) |
| `imgColorType` | string | No | null | Color type (color, gray, mono, trans) |
| `imgDominantColor` | string | No | null | Dominant color (black, blue, brown, gray, green, orange, pink, purple, red, teal, white, yellow) |
| `imgUsageRights` | string | No | null | Usage rights (fmc, fc, fm, f, cc) |
| `tbs` | string | No | null | Time-based search parameters |

## Response Format

### Success Response (200 OK)

```json
{
  "searchParameters": {
    "q": "sunset mountains",
    "gl": "us",
    "hl": "en",
    "num": 10,
    "start": 0,
    "autocorrect": true,
    "safe": "off",
    "filter": "1",
    "imgSize": "large",
    "imgType": "photo",
    "imgColorType": "color",
    "type": "images",
    "engine": "google"
  },
  "images": [
    {
      "title": "Beautiful Sunset Over Mountains",
      "imageUrl": "https://example.com/images/sunset-mountains-1.jpg",
      "imageWidth": 1920,
      "imageHeight": 1080,
      "thumbnailUrl": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcExample",
      "thumbnailWidth": 259,
      "thumbnailHeight": 194,
      "source": "Nature Photography",
      "domain": "example.com",
      "link": "https://example.com/sunset-mountains-gallery",
      "googleUrl": "https://www.google.com/imgres?imgurl=https%3A%2F%2Fexample.com%2Fimages%2Fsunset-mountains-1.jpg&tbnid=ExampleThumbnailId&imgrefurl=https%3A%2F%2Fexample.com%2Fsunset-mountains-gallery&docid=ExampleDocId&w=1920&h=1080&ved=0ahUKEwiExample",
      "position": 1,
      "snippet": "Stunning sunset view over mountain peaks",
      "isProduct": false
    },
    {
      "title": "Mountain Sunset Landscape",
      "imageUrl": "https://example2.com/photos/mountain-sunset.jpg",
      "imageWidth": 2048,
      "imageHeight": 1365,
      "thumbnailUrl": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcExample2",
      "thumbnailWidth": 275,
      "thumbnailHeight": 183,
      "source": "Landscape Photos",
      "domain": "example2.com",
      "link": "https://example2.com/mountain-sunset-collection",
      "googleUrl": "https://www.google.com/imgres?imgurl=https%3A%2F%2Fexample2.com%2Fphotos%2Fmountain-sunset.jpg&tbnid=Example2ThumbnailId&imgrefurl=https%3A%2F%2Fexample2.com%2Fmountain-sunset-collection&docid=Example2DocId&w=2048&h=1365&ved=0ahUKEwiExample2",
      "position": 2,
      "snippet": "Golden hour mountain landscape photography",
      "isProduct": false
    }
  ],
  "searchInformation": {
    "queryDisplayed": "sunset mountains",
    "totalResults": "About 45,600,000 results",
    "timeTaken": 0.34,
    "formattedTotalResults": "45,600,000",
    "formattedTimeTaken": "0.34 seconds"
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
- `type`: Search type (always "images")
- `engine`: Search engine used (always "google")

#### Images
- `images`: Array of image search results
- `title`: Image title or alt text
- `imageUrl`: Full-size image URL
- `imageWidth`: Full-size image width in pixels
- `imageHeight`: Full-size image height in pixels
- `thumbnailUrl`: Thumbnail image URL
- `thumbnailWidth`: Thumbnail width in pixels
- `thumbnailHeight`: Thumbnail height in pixels
- `source`: Image source/website name
- `domain`: Domain of the source website
- `link`: URL of the page containing the image
- `googleUrl`: Google Images URL for this result
- `position`: Result position (1-based)
- `snippet`: Image description or caption
- `isProduct`: Whether this is a product image

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

### Basic Image Search

**Request:**
```bash
curl -X POST https://api.tarzi.dev/v1/images \
  -H "Content-Type: application/json" \
  -H "X-API-KEY: your-api-key" \
  -d '{
    "q": "golden retriever puppies"
  }'
```

### Filtered Image Search

**Request:**
```bash
curl -X POST https://api.tarzi.dev/v1/images \
  -H "Content-Type: application/json" \
  -H "X-API-KEY: your-api-key" \
  -d '{
    "q": "business presentations",
    "imgSize": "large",
    "imgType": "photo",
    "imgColorType": "color",
    "safe": "moderate"
  }'
```

### Color-Specific Search

**Request:**
```bash
curl -X POST https://api.tarzi.dev/v1/images \
  -H "Content-Type: application/json" \
  -H "X-API-KEY: your-api-key" \
  -d '{
    "q": "flowers",
    "imgColorType": "color",
    "imgDominantColor": "red",
    "num": 20
  }'
```

## Image Size Options

| Value | Description |
|-------|-------------|
| `large` | Large images (typically >1024px) |
| `medium` | Medium images (typically 512-1024px) |
| `small` | Small images (typically <512px) |
| `icon` | Icon-sized images (typically <128px) |

## Image Type Options

| Value | Description |
|-------|-------------|
| `photo` | Photographic images |
| `clipart` | Clip art and illustrations |
| `lineart` | Line art and drawings |
| `face` | Images containing faces |

## Color Type Options

| Value | Description |
|-------|-------------|
| `color` | Full color images |
| `gray` | Grayscale images |
| `mono` | Monochrome (black and white) images |
| `trans` | Images with transparency |

## Dominant Color Options

| Value | Description |
|-------|-------------|
| `black` | Predominantly black images |
| `blue` | Predominantly blue images |
| `brown` | Predominantly brown images |
| `gray` | Predominantly gray images |
| `green` | Predominantly green images |
| `orange` | Predominantly orange images |
| `pink` | Predominantly pink images |
| `purple` | Predominantly purple images |
| `red` | Predominantly red images |
| `teal` | Predominantly teal images |
| `white` | Predominantly white images |
| `yellow` | Predominantly yellow images |

## Usage Rights Options

| Value | Description |
|-------|-------------|
| `fmc` | Free to use or share commercially |
| `fc` | Free to use or share |
| `fm` | Free to use |
| `f` | Free to use or share, even commercially |
| `cc` | Creative Commons licensed |

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

### Invalid Image Parameters (400 Bad Request)

```json
{
  "error": {
    "type": "validation_error",
    "message": "Invalid value for parameter 'imgSize'. Must be one of: large, medium, small, icon",
    "code": "INVALID_PARAMETER"
  }
}
```

## Rate Limiting

The images endpoint is subject to the standard rate limits:
- Headers: `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset`
- Quota exceeded returns `429 Too Many Requests`

## Performance

- **Average Response Time**: 400-900ms
- **95th Percentile**: <1.8s
- **99th Percentile**: <3.5s
- **Availability**: 99.9% uptime SLA

## Notes

- Image results are fetched in real-time with no caching
- Image URLs may be temporary and subject to change
- Thumbnail URLs are generated by Google and may expire
- Copyright and usage rights information is provided for reference only
- Always respect image copyrights and usage rights
- Some images may not be available in all regions due to regional restrictions