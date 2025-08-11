export interface Env {
  UPSTREAM: string;
}

export default {
  async fetch(
    request: Request,
    env: Env,
    ctx: ExecutionContext
  ): Promise<Response> {
    const url = new URL(request.url);

    if (url.pathname === '/ws') {
      if (request.headers.get("Upgrade") !== "websocket") {
        return new Response("Expected WebSocket", { status: 400 });
      }

      const pair = new WebSocketPair();
      const [client, server] = Object.values(pair);

      ctx.waitUntil(handleWebSocket(server, env.UPSTREAM));

      return new Response(null, {
        status: 101,
        webSocket: client,
      });
    }

    return fetch(env.UPSTREAM, request);
  },
};

async function handleWebSocket(socket: WebSocket, upstream: string) {
  socket.accept();

  const upstreamSocket = await fetch(upstream, {
    headers: { Upgrade: 'websocket' },
    duplex: 'half',
  }).then(r => r.webSocket!);

  upstreamSocket.accept();

  socket.addEventListener('message', (event) => {
    upstreamSocket.send(event.data);
  });

  upstreamSocket.addEventListener('message', (event) => {
    socket.send(event.data);
  });
}