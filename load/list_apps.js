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
    http_req_duration: ['p(95)<1000'],
  },
};

export default function () {
  const page = Math.ceil(Math.random() * 50);
  const res = http.get(`${BASE}/apps?page=${page}&per_page=20`);
  check(res, { 'status 200': (r) => r.status === 200 });
}
