import http from 'k6/http';
import { check } from 'k6';

const BASE   = __ENV.BASE_URL || 'http://localhost:3000';
const RUN_ID = Date.now();

export const options = {
  scenarios: {
    readers: {
      executor: 'ramping-vus',
      startVUs: 0,
      stages: [
        { duration: '30s', target: 20  },
        { duration: '60s', target: 70  },
        { duration: '60s', target: 100 },
        { duration: '20s', target: 0   },
      ],
      exec: 'read',
    },
    writers: {
      executor: 'ramping-vus',
      startVUs: 0,
      stages: [
        { duration: '30s', target: 5  },
        { duration: '60s', target: 20 },
        { duration: '60s', target: 30 },
        { duration: '20s', target: 0  },
      ],
      exec: 'write',
    },
  },
  thresholds: {
    'http_req_failed':                         ['rate<0.01'],
    'http_req_duration{scenario:readers}':     ['p(95)<200'],
    'http_req_duration{scenario:writers}':     ['p(95)<1000'],
  },
};

export function read() {
  const page = Math.ceil(Math.random() * 20);
  const res  = http.get(`${BASE}/apps?page=${page}&per_page=20`);
  check(res, { 'status 200': (r) => r.status === 200 });
}

export function write() {
  const id  = `com.mixed.${RUN_ID}.${__VU}.${__ITER}`;
  const res = http.post(
    `${BASE}/apps`,
    JSON.stringify({ bundle_id: id, name: 'Mixed App', developer: 'Load Tester', description: null }),
    { headers: { 'Content-Type': 'application/json' } },
  );
  check(res, { 'status 201': (r) => r.status === 201 });
}
