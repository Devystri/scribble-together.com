
use crate::chunk::{self, Chunk, SIZE};
use actix_web::{get, HttpRequest, HttpResponse, Result};
use csv::Terminator;

#[get("/tile/get/{adress:.*}")]
async fn index(req: HttpRequest) -> Result<HttpResponse> {
    let adress = req.match_info().get("adress").unwrap();
    let map = chunk::MAP.lock();
    let map = match map {
        Ok(map) => map,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
    println!("{:?}", adress);
    let chunk = match map.get_chunk(&format!("./{}", adress)) {
        Ok(chunk) => chunk,
        Err(e) => {
            println!("{:?}", e);
            return Ok(HttpResponse::NotFound().finish());
        }
    };

    println!("{}", adress);
    if let Chunk::Leaf { id: _, pixels } = chunk {
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b';')
            .has_headers(false)
            .terminator(Terminator::CRLF)
            .from_writer(vec![]);
            for x in 0..SIZE-1 {
                for y in 0..SIZE-1 {
                
                    if pixels[x][y].color < 0 {
                        continue;
                    }
                    match wtr.write_record(&[
                        x.to_string(),
                        y.to_string(),
                        pixels[x][y].color.to_string(),
                    ]) {
                        Ok(_) => (),
                        Err(e) => {
                            println!("{:?}", e);
                            return Ok(HttpResponse::InternalServerError().body(e.to_string()));
                        }
                    }
                }
        }

        let data = match wtr.into_inner() {
            Ok(data) => match String::from_utf8(data) {
                Ok(data) => data,
                Err(e) => {
                    println!("{:?}", e);
                    return Ok(HttpResponse::InternalServerError().finish());
                }
            },
            Err(e) => {
                println!("{:?}", e);
                return Ok(HttpResponse::InternalServerError().finish());
            }
        };
        Ok(HttpResponse::Ok().content_type("text/csv").body(data))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
