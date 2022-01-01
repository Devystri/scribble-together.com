use std::time::{Duration, Instant};

use actix::{Actor, StreamHandler, AsyncContext, ActorContext};
use actix_web::{get, web, App, HttpRequest, HttpServer, Responder, Error, HttpResponse };
use actix_web_actors::ws;
use actix_http::Response;
struct WebSocketServer{
    hb: Instant,

}
/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
impl Actor for WebSocketServer {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketServer {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        // process websocket messages
        println!("WS: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}
impl WebSocketServer{
    fn new() -> Self {
        Self { hb: Instant::now() }
    }
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}


pub async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp= ws::start(WebSocketServer::new(), &req, stream);

    println!("{:?}", resp);
    resp
}
