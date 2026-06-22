import http from 'k6/http';
import { check } from 'k6';

const BASE   = __ENV.BASE_URL || 'http://localhost:3000';
const RUN_ID = Date.now();

export const options = {
  stages: [
    { duration: '30s', target: 10  },
    { duration: '60s', target: 50  },
    { duration: '60s', target: 100 },
    { duration: '60s', target: 150 },
    { duration: '20s', target: 0   },
  ],
  thresholds: {
    http_req_failed:   ['rate<0.01'],
    http_req_duration: ['p(95)<2000'],
  },
};

export default function () {
  const id  = `com.heavy.${RUN_ID}.${__VU}.${__ITER}`;
  const res = http.post(
    `${BASE}/apps`,
    JSON.stringify({ bundle_id: id, name: 'Heavy App', developer: 'Load Tester', description: null }),
    { headers: { 'Content-Type': 'application/json' } },
  );
  check(res, { 'status 201': (r) => r.status === 201 });
}
