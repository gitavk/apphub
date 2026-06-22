import http from 'k6/http';
import { check } from 'k6';

const BASE = __ENV.BASE_URL || 'http://localhost:3000';

export const options = {
  stages: [
    { duration: '30s', target: 5  },
    { duration: '60s', target: 20 },
    { duration: '60s', target: 50 },
    { duration: '20s', target: 0  },
  ],
  thresholds: {
    http_req_failed:   ['rate<0.01'],
    http_req_duration: ['p(95)<1000'],
  },
};

// run-scoped prefix ensures IDs are unique across repeated runs
const RUN_ID = Date.now();

export default function () {
  const id = `com.load.${RUN_ID}.${__VU}.${__ITER}`;
  const payload = JSON.stringify({
    bundle_id:   id,
    name:        'Load App',
    developer:   'Load Tester',
    description: null,
  });
  const res = http.post(`${BASE}/apps`, payload, {
    headers: { 'Content-Type': 'application/json' },
  });
  check(res, { 'status 201': (r) => r.status === 201 });
}
