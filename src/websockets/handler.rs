use std::{time::{Duration, Instant}, sync::Mutex, string};

use crate::{chunk::{self, Pixel}, common::is_numeric};
use actix::{Actor, ActorContext, AsyncContext, StreamHandler, Addr, Handler, Message};
use actix_web::{
    web::{self},
    Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws;

use futures_util::Stream;
use once_cell::sync::Lazy;


use self::errors::HandleRequestError;

#[derive(Debug)]
pub struct WebSocketServer {
    hb: Instant,
    chunks_adress: [String; 9],
}

#[derive(Message)]
#[rtype(result = "()")]
struct ServerEvent {
    event: String,
}

impl Handler<ServerEvent> for WebSocketServer {
    type Result = ();

    fn handle(&mut self, msg: ServerEvent, ctx: &mut Self::Context) {
        ctx.text(msg.event);
    }
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
            Ok(ws::Message::Text(text)) => 
            {
                let data = text.to_string();
                if data.starts_with('/') {
                    let v: Vec<&str> = data.splitn(2, ' ').collect();
                    match v[0] {
                        "/send" =>{
                            match self.handle_request(v[1]) {
                                Ok(_) => (),
                                Err(e) => println!("{:?}", e),
                            }
                        },
                        "/register" =>{
                            let load_adress = v[1].to_string().trim().to_lowercase();
                            // Adress layout: root/x/y
                            let settings = load_adress.split('/');
                            let n: usize = settings.clone().count();
                            if !(2..=16).contains(&n) {
                                println!("Invalid adress length");
                                return;
                            }
                            let coords = settings.clone().last().unwrap().split('_').collect::<Vec<&str>>();
                            if coords.len() != 2 {
                                println!("Invalid adress format coords must have 2 values");
                                return;
                            }
                        
                            let x = coords[0].parse::<isize>().unwrap();
                            let y = coords[1].parse::<isize>().unwrap();

                            self.chunks_adress[0] = load_adress;
                            self.chunks_adress[1] = format!("{}_{}", x + 1, y);
                            self.chunks_adress[2] = format!("{}_{}", x + 1, y + 1);
                            self.chunks_adress[3] = format!("{}_{}", x, y + 1);
                            self.chunks_adress[4] = format!("{}_{}", x - 1, y + 1);
                            self.chunks_adress[5] = format!("{}_{}", x - 1, y);
                            self.chunks_adress[6] = format!("{}_{}", x - 1, y - 1);
                            self.chunks_adress[7] = format!("{}_{}", x, y - 1);
                            self.chunks_adress[8] = format!("{}_{}", x + 1, y - 1);
                            let mut map = match chunk::MAP.lock() {
                                Ok(map) => map,
                                Err(e) => {
                                    println!("{:?}", e);
                                    return;
                                }
                            };
                            let addr = ctx.address();
                            let sess = Session{ addr, id: 9 };
                            match map.add_session(&sess, &self.chunks_adress[0]){
                                Ok(_) => (),
                                Err(e) => println!("{:?}", e),
                            }

                        },
                        _ => (),
                    }
                }
                
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            },
            _ => ctx.stop(),
        }
    }
}

impl WebSocketServer {
    fn new() -> Self {
        Self { hb: Instant::now(), chunks_adress: [(); 9].map(|_| String::new()) }
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

    fn send_update(&self, sessions: &[Session], pixels: &[Pixel]) {
        for session in sessions {
            let addr = session.addr.clone();
            let mut data=  String::from("/update ");
            for pixel in pixels {
                data.push_str(&pixel.to_string());
            }
            addr.do_send(ServerEvent{ event: data });
        }
    }

    fn handle_request(&self, message: &str) -> Result<(), HandleRequestError> {
                
        let pixel: Vec<Pixel> = chunk::Chunk::from_string(message);

        let mut map = match chunk::MAP.lock() {
            Ok(map) => map,
            Err(e) => {
                println!("{:?}", e);
                return Err(HandleRequestError::ImpossibleToHandle);
            }
        };

        match map.add_pixel(&pixel, "") {
            Ok(sessions) => {self.send_update(&sessions, &pixel); },
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

#[derive(Debug, Clone)]
pub struct Session{
    pub addr: Addr<WebSocketServer>,
    pub id: usize,
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
