import { Redis, Config } from "@fermyon/spin-sdk"

const encoder = new TextEncoder()
const decoder = new TextDecoder()

const redisAddress = "redis://localhost:6379/"


export async function handleRequest(request) {
    const connectionString = Config.get("redis_connection_string");
    const channel = Config.get("redis_channel")
    if (!connectionString || !channel) {
        return {
            status: 500,
            headers: { "content-type": "text/plain" },
            body: "Redis Connection not configured."
        }
    }
    Redis.publish(connectionString, channel, encoder.encode("This message has been generated using the Spin HTTP app written in JS").buffer)

    return {
        status: 201,
        headers: { "content-type": "text/plain" },
        body: "Your message has been submitted to Redis"
    }
}
