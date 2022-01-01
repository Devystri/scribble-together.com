use actix::{Actor, StreamHandler};
use actix_web::{get, web, App, HttpRequest, HttpServer, Responder, Error, HttpResponse };
use actix_web_actors::ws;
use actix_http::Response;
struct WebSocketServer;

impl Actor for WebSocketServer {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketServer {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => ctx.close(reason),
            _ => ctx.close(Some(ws::CloseCode::Error.into())),
        }
    }
}



pub async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::WsResponseBuilder::new(WebSocketServer{}, &req, stream);
    let resp = resp.start();
    println!("{:?}", resp);
    resp
}
