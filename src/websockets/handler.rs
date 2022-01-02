use std::time::{Duration, Instant};

use crate::chunk::{self, Pixel};
use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web::{
    web::{self},
    Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws;

use serde_json;

use self::errors::HandleRequestError;
struct WebSocketServer {
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
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => match self.handle_request(&text.to_string()) {
                Ok(_) => (),
                Err(e) => println!("{:?}", e),
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl WebSocketServer {
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

    fn handle_request(&self, message: &str) -> Result<(), HandleRequestError> {
        //TODO error handling
        let pixel: Pixel = match serde_json::from_str(message) {
            Ok(pixel) => pixel,
            Err(e) => {
                println!("{:?}", e);
                return Err(HandleRequestError::ImpossibleToHandle);
            }
        };

        let mut map = match chunk::MAP.lock() {
            Ok(map) => map,
            Err(e) => {
                println!("{:?}", e);
                return Err(HandleRequestError::ImpossibleToHandle);
            }
        };

        match map.add_pixel(&pixel, "") {
            Ok(_) => {}
            Err(e) => println!("{:?}", e),
        }
        match map.save() {
            Ok(_) => {}
            Err(e) => {
                println!("{:?}", e);
            }
        }
        println!("{}", message);
        Ok(())
    }
}

pub async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(WebSocketServer::new(), &req, stream);

    println!("{:?}", resp);
    resp
}

mod errors {

    #[derive(Debug)]
    pub enum HandleRequestError {
        ImpossibleToHandle,
    }
}
