
use crate::chunk::{self, Chunk, SIZE};
use actix_web::{get, HttpRequest, HttpResponse, Result};

#[get("/tile/get/{adress:.*}")]
async fn index(req: HttpRequest) -> Result<HttpResponse> {
    let adress = req.match_info().get("adress").unwrap();
    let map = chunk::MAP.lock();
    let map = match map {
        Ok(map) => map,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
    let chunk = match map.get_chunk(adress) {
        Ok(chunk) => chunk,
        Err(e) => {
            println!("{:?}", e);
            return Ok(HttpResponse::NotFound().finish());
        }
    };

    if let Chunk::Leaf { id: _, pixels, sessions: _ } = chunk {
        let mut data = String::new();
        for x in 0..(SIZE -1) {
            for y in 0..(SIZE-1) {
                data.push_str(&format!("{:03}{:03}{:03}\n", x, y, pixels[x][y].color));
            }
        }
        Ok(HttpResponse::Ok().content_type("text/csv").body(data))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
