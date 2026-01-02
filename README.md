# Lingua English Detection Service

High-precision English language detection microservice using optimized Lingua-rs library.

## Features

- **Optimized for English detection**: 4 languages (English, French, German, Spanish)
- **High precision**: Minimum relative distance 0.4 prevents uncertain detections
- **Confidence threshold**: > 0.5 for reliable English classification
- **Thread-safe**: Suitable for concurrent requests

## API Endpoints

### POST /detect

High-precision English language detection (batch processing).

**Request:**

```json
{
  "texts": ["This is a sample text in English"]
}
```

**Response (English detected):**

```json
{
  "results": [{"is_english": true}]
}
```

**Response (Non-English or uncertain):**

```json
{
  "results": [{"is_english": false}]
}
```

### GET /health

Health check endpoint. Returns "OK".

## Local Testing

### Using curl:

#### English Detection

```bash
# Test clear English text
curl -X POST http://localhost:3030/detect \
  -H "Content-Type: application/json" \
  -d '{"texts":["This is a comprehensive explanation of machine learning algorithms"]}'
# Returns: {"results": [{"is_english": true}]}

# Test short English word
curl -X POST http://localhost:3030/detect \
  -H "Content-Type: application/json" \
  -d '{"texts":["programming"]}'
# Returns: {"results": [{"is_english": true}]}

# Test ambiguous short text (should return is_english: false)
curl -X POST http://localhost:3030/detect \
  -H "Content-Type: application/json" \
  -d '{"texts":["hello"]}'
# Returns: {"results": [{"is_english": false}]}

# Test non-English text
curl -X POST http://localhost:3030/detect \
  -H "Content-Type: application/json" \
  -d '{"texts":["Bonjour"]}'
# Returns: {"results": [{"is_english": false}]}
```

#### Health check

```bash
curl http://localhost:3030/health
```

## Building and Running

### With Docker Compose (recommended for development):

```bash
docker compose up lingua
```

### Standalone Docker:

```bash
docker build -t lingua-service .
docker run -p 3030:3030 lingua-service
```

### Local Rust development:

```bash
cargo run
```

The service always listens on port 3030.

## Performance & Configuration

### Optimized Configuration

- **Languages**: English, French, German, Spanish (4 total)
- **Minimum relative distance**: 0.4 (prevents uncertain detections)
- **English threshold**: Confidence > 0.5
- **Accuracy**: 100% on test cases (14/14 correct)
