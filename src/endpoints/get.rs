use std::io;

use actix_web::{get, HttpResponse, HttpRequest, Result};
use csv::Terminator;
use crate::chunk::{self, Chunk};


#[get("/tile/get/{adress}")]
async fn index(req: HttpRequest) -> Result<HttpResponse> {
    let adress = req.match_info().get("adress").unwrap();
    let map = chunk::MAP.lock();
    let map = match map {
        Ok(map) => map,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
        
    let chunk = match map.get_chunk(&String::from(adress)){
        Ok(chunk) => chunk,
        Err(e) => {
            println!("{:?}", e);
            return Ok(HttpResponse::NotFound().finish())
        }
    };
    
    println!("{}", adress);
    if let Chunk::Leaf {id:_ , pixels } = chunk  {
        let mut wtr = csv::WriterBuilder::new().delimiter(b';').has_headers(false).terminator( Terminator::CRLF).from_writer(vec![]);
        for pixel in pixels{
            if pixel.color < 0  {
                continue;
            }
            match wtr.write_record(&[pixel.x.to_string(), pixel.y.to_string(), pixel.color.to_string()]){
                Ok(_) => (),
                Err(e) => {
                    println!("{:?}", e);
                    return Ok(HttpResponse::InternalServerError().body(e.to_string()))},
            }
    
        }

        let data = match wtr.into_inner() {
            Ok(data) => match String::from_utf8( data){
                Ok(data) => data,
                Err(e) => {
                    println!("{:?}", e);
                    return Ok(HttpResponse::InternalServerError().finish())},
            },
            Err(e) => {
                println!("{:?}", e);
                return Ok(HttpResponse::InternalServerError().finish())},
        };
        Ok(HttpResponse::Ok().content_type("text/csv").body(data))

    }else {
        Ok(HttpResponse::NotFound().finish())
    }


}