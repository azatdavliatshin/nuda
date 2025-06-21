export default function handler(req) {
    return {
        status: 200,
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message: 'Hello from SSR route!' })
    };
}
