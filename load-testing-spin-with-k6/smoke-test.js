import http from 'k6/http';
import { check, fail, sleep } from 'k6';

export const options = {
    vus: 3,
    duration: '30s',
};

const json = __ENV.JSON;
let url = "http://localhost:3000";
if (json != "1") {
    url = `${url}/plain`;
}
export default () => {
    const response = http.get(url);
    if (!check(response, {
        "Testing Response Code": (r) => r.status == 200
    })) {
        fail(`HTTP request failed with status ${r.status}`);
    }

    if (!check(response, {
        "Testing Response Containes desired key": (r) => r.json()["key"] == 42,
    })) {
        fail(`HTTP response did not contain desired key (42)`);
    }
    sleep(1);
};