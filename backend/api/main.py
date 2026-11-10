from fastapi import FastAPI, Response
from integrations.google.manager import GoogleMCPManager
import uvicorn

app = FastAPI()
google_manager = GoogleMCPManager()

@app.get("/health")
async def health():
    return {"status": "running"}

@app.post("/connect/google")
async def connect_google():
    connections = {}

    for service in google_manager.endpoints:
        if service in google_manager.connections:
            connections[service] = "connected"
            continue

        try:
            await google_manager.connect(service)
            await google_manager.verify(service)
            connections[service] = "connected"
        except Exception as exc:
            # Don't cache an unverified session: the next click must retry.
            await google_manager.disconnect(service)
            connections[service] = f"error: {exc}"

    failed = [s for s, status in connections.items() if status != "connected"]

    if failed and len(failed) == len(connections):
        # Plain text so the desktop toast can show the reason directly.
        return Response(
            status_code=502,
            content="\n".join(f"{service}: {connections[service]}" for service in failed),
            media_type="text/plain",
        )

    return {
        "status": "partial" if failed else "connected",
        "connections": connections
    }

if __name__ == "__main__":
    uvicorn.run(
        app,
        host="127.0.0.1",
        port=8000
    )

