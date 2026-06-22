import http from 'k6/http';
import { check } from 'k6';

const BASE = __ENV.BASE_URL || 'http://localhost:3000';

export const options = {
  stages: [
    { duration: '30s', target: 10  },
    { duration: '60s', target: 50  },
    { duration: '60s', target: 150 },
    { duration: '20s', target: 0   },
  ],
  thresholds: {
    http_req_failed:   ['rate<0.01'],
    http_req_duration: ['p(95)<500'],
  },
};

export function setup() {
  const res = http.get(`${BASE}/apps?per_page=100`);
  return JSON.parse(res.body).map((a) => a.id);
}

export default function (ids) {
  const id  = ids[Math.floor(Math.random() * ids.length)];
  const res = http.patch(
    `${BASE}/apps/${id}`,
    JSON.stringify({ name: `Updated ${__VU}` }),
    { headers: { 'Content-Type': 'application/json' } },
  );
  check(res, { 'status 200': (r) => r.status === 200 });
}
