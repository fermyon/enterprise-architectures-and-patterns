import http from 'k6/http';
import { check, fail, sleep } from 'k6';
export const options = {
    stages: [
        { duration: '5s', target: 200 },
        { duration: '5s', target: 220 },
        { duration: '5s', target: 230 },
        { duration: '10s', target: 240 },
        { duration: '10s', target: 0 },
    ],
};

const json = __ENV.JSON;
let url = "http://localhost:3000";
if (json != "1") {
    url = `${url}/plain`;
}

export default () => {
    const res = http.get(url);
    if (
        !check(res, {
            'Checking for status code 200': (res) => res.status == 200,
        })
    ) {
        fail(`HTTP request failed. Received status ${res.status}`);
    }
    // wait for 0.3 sec after each iteration
    sleep(.3)
};