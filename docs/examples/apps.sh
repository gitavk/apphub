#!/usr/bin/env bash
BASE="${BASE:-http://localhost:3000}"

# Create an app
curl -s -X POST "$BASE/apps" \
  -H "Content-Type: application/json" \
  -d '{"bundle_id":"com.example.myapp","name":"My App","developer":"Example Corp","description":"A great example app"}' | jq

# Create an app without description
curl -s -X POST "$BASE/apps" \
  -H "Content-Type: application/json" \
  -d '{"bundle_id":"com.example.minimal","name":"Minimal App","developer":"Example Corp"}' | jq

# Validation failure — empty required fields → 422
curl -s -X POST "$BASE/apps" \
  -H "Content-Type: application/json" \
  -d '{"bundle_id":"","name":"","developer":""}' | jq

# List apps (default: page 1, 20 per page)
curl -s "$BASE/apps" | jq

# List apps — page 2, 5 per page
curl -s "$BASE/apps?page=2&per_page=5" | jq

# Get app by ID (replace UUID with a real one from create/list)
APP_ID="00000000-0000-0000-0000-000000000000"
curl -s "$BASE/apps/$APP_ID" | jq

# Get app — not found → 404
curl -s "$BASE/apps/ffffffff-ffff-ffff-ffff-ffffffffffff" | jq
