# Apps API examples

All examples assume the service is running on `localhost:3000`.

---

## Create an app

```bash
curl -s -X POST http://localhost:3000/apps \
  -H "Content-Type: application/json" \
  -d '{"bundle_id":"com.example.myapp","name":"My App","developer":"Example Corp","description":"A great example app"}' | jq
```

## Create an app without description

```bash
curl -s -X POST http://localhost:3000/apps \
  -H "Content-Type: application/json" \
  -d '{"bundle_id":"com.example.minimal","name":"Minimal App","developer":"Example Corp"}' | jq
```

## Validation failure — empty required fields → 422

```bash
curl -s -X POST http://localhost:3000/apps \
  -H "Content-Type: application/json" \
  -d '{"bundle_id":"","name":"","developer":""}' | jq
```

## Duplicate bundle_id → 409

```bash
curl -s -X POST http://localhost:3000/apps \
  -H "Content-Type: application/json" \
  -d '{"bundle_id":"com.example.myapp","name":"My App","developer":"Example Corp"}' | jq
```

## List apps — default (page 1, 20 per page)

```bash
curl -s http://localhost:3000/apps | jq
```

## List apps — page 2, 5 per page

```bash
curl -s "http://localhost:3000/apps?page=2&per_page=5" | jq
```

## Get app by ID

Replace the UUID with a real one from the create or list response.

```bash
curl -s http://localhost:3000/apps/00000000-0000-0000-0000-000000000000 | jq
```

## Get app — not found → 404

```bash
curl -s http://localhost:3000/apps/ffffffff-ffff-ffff-ffff-ffffffffffff | jq
```
