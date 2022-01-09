
use actix::Message;
use bincode;
use flate2::{write::ZlibEncoder, Compression};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs::{OpenOptions, self};
use std::sync::Mutex;
use std::{ io::BufWriter};

pub const SIZE: usize = 256;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Chunk {
    Container {
        #[serde(skip)]
        id: String,
        childrens: Vec<Chunk>,
    },
    Leaf {
        #[serde(skip)]
        id: String,
        #[serde(skip)]
        sessions: Vec<Session>,
        pixels: Vec<Vec<Pixel>>,
    },
}

impl Chunk {
    pub fn new_root() -> Self {
        Self::Container {
            id: String::from("map"),
            childrens: vec![Self::new_leaf("map", (0, 0)).unwrap()],
        }
    }

    pub fn new_leaf(parent_adress: &str, coords: (usize, usize)) -> Option<Self> {
        match fs::create_dir_all(format!("./{}", parent_adress)){
            Ok(_) => (),
            Err(e) => {
                println!("{:?}", e);
                return None;
            }
        }
        let id = format!("{}/{}_{}", parent_adress, coords.0, coords.1);
        let leaf = Self::Leaf {
            id,
            pixels: vec![vec![Pixel::default(); SIZE]; SIZE],
            sessions: vec![],

        };
        match leaf.save(){
            Ok(_) => (),
            Err(e) => {
                println!("{:?} 2", e);
                return None;
            }
        }
        Some(leaf)
        
        
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Message)]
#[rtype(result = "()")]
pub struct Pixel {
    pub color: i8,
    #[serde(skip_serializing)]
    pub x: u8,
    #[serde(skip_serializing)]
    pub y: u8,
}

impl Default for Pixel {
    fn default() -> Self {
        Self { color: -1, x: 0, y: 0 }
    }
}

impl Pixel {
    pub fn from_string(txt: &str)-> Option<Self> {
        // pixel format: xxxyyycolorcolorcolor
        // xxx: x coordinate
        // yyy: y coordinate
        // colorcolorxolor: color
        if txt.len() != 9 {
            println!("{} not is of a length of 9", txt);
            return  None;
        }
        let x = match txt[0..3].parse::<u8>() {
            Ok(x) => x,
            Err(e) => {
                println!("{:?}", e);
                return None;
            }
        };
        let y = match txt[3..6].parse::<u8>() {
            Ok(y) => y,
            Err(e) => {
                println!("{:?}", e);
                return None;
            }
        };
        let color = match txt[6..9].parse::<i8>() {
            Ok(color) => color,
            Err(e) => {
                println!("{:?}", e);
                return None;
            }
        };

        Some(Self { color, x, y })
    }
}

impl ToString for Pixel{
    fn to_string(&self) -> String {
        format!("{:03}{:03}{:03}\n", self.x, self.y, self.color)
        
    }
}


pub(crate) static MAP: Lazy<Mutex<Chunk>> = Lazy::new(|| Mutex::new(Chunk::new_root()));
use errors::*;

use crate::websockets::handler::Session;


impl Chunk {
    pub fn save(&self) -> Result<(), SaveError> {
        match self {
            Chunk::Container { id: _, childrens } => {
                for child in childrens.iter() {
                    child.save()?;
                }
                Ok(())
            }
            Chunk::Leaf { id, pixels, sessions: _ } => {
                let file = match OpenOptions::new().write(true).create(true).truncate(true).open("./".to_owned() + id + ".chunk.gz") {
                    Ok(file) => file,
                    Err(e) => return Err(SaveError::Io(e)),
                };
                let writer = BufWriter::new(file);
                let encoder = ZlibEncoder::new(writer, Compression::fast());
                match bincode::serialize_into(encoder, pixels) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(SaveError::Bincode(e)),
                }
            }
        }

    }

    pub fn add_pixel(&mut self, pixels_adds: &[Pixel], adress: &str) -> Result<Vec<Session>, AddError> {
        match self {
            Chunk::Container { id: _, childrens } => {
                for child in childrens.iter_mut() {
                    match child.add_pixel(pixels_adds, adress){
                        Ok(sessions) => return Ok(sessions),
                        Err(_) => continue,
                    }
                }
                Err(AddError::NotLeaf)
            }
            Chunk::Leaf { id: _, pixels, sessions } => {
                for pixel in pixels_adds.iter() {
                    pixels[pixel.x as usize][pixel.y as usize].color = pixel.color;
                    
                }
                Ok(sessions.clone())
            }
        }
    }

    pub fn get_chunk(&self, adress: &str) -> Result<&Chunk, GetError> {
        match self {
            Chunk::Container { id: _, childrens } => {
                for child in childrens.iter() {
                    match child.get_chunk(adress) {
                        Ok(chunk) => return Ok(chunk),
                        Err(e) => {
                            if let GetError::NotAdress = e {
                                continue;
                            } else {
                                return Err(e);
                            }
                        }
                    }
                }
                Err(GetError::NotFound)
            }
            Chunk::Leaf { id, pixels: _ , sessions: _} => {
                if id == adress {
                    Ok(self)
                } else {
                    Err(GetError::NotAdress)
                }
            }
        }
    }

    pub fn add_session(&mut self, session: &Session, adress: &str) -> Result<(), AddSessionError> {
        match self {
            Chunk::Container { id: _, childrens } => {
                for child in childrens.iter_mut() {
                    match child.add_session(session, adress){
                        Ok(_) => return Ok(()),
                        Err(e) => {
                            if let AddSessionError::NotFound = e {
                                continue;
                            } else {
                                return Err(e);
                            }
                        }
                    }
                }
                Err(AddSessionError::NotFound)
            }
            Chunk::Leaf { id, pixels: _, sessions } => {
                if id == adress {
                    sessions.push(session.clone());
                    
                    Ok(())
                } else {
                    Err(AddSessionError::NotFound)
                }
            }
        }
    }

    pub fn from_string(txt: &str) -> Vec<Pixel> {
        let mut pixels = Vec::new();
        let mut pixel = Pixel::default();
        let mut line= String::new();
        let mut param_index = 0; 
        for c in txt.chars() {
            if c == '\n' {
                if line.len() != 9 {
                    pixels.pop();
                    println!("len: {}", line.len());
                }
                pixels.push(pixel);
                pixel = Pixel::default();
                line.clear();
                param_index = 0;
                continue;
            }else{
                if c.is_numeric() || c == '-' {
                    line.push(c);
                }
                if !line.is_empty() && line.len() % 3 == 0 {
                    if param_index == 2 && line.len() >= 9 {
                        match line[6..9].parse::<i8>() {
                            
                            Ok(v) => {
                                pixel.color = v;
                            },
                            Err(e) => {
                                println!("No parse: {:?}", e);
                            }
                        };
                    }else{
                        match line[param_index*3..(param_index+1)*3].parse::<u8>() {
                            Ok(v) => {
                                match param_index {
                                    0 => pixel.x = v,
                                    1 => pixel.y = v,
                                    _ => ()
                                }
                                    
                            },
                            Err(e) => {
                                println!("No parse: {:?}", e);
                            }
                        };
                    }
                    param_index += 1;
                }
            }
        }

        pixels
    }


}
pub mod errors {
    use super::*;
    #[derive(Debug)]
    pub enum SaveError {
        Io(std::io::Error),
        Bincode(bincode::Error),
    }
    #[derive(Debug)]
    pub enum AddError {
        NotLeaf,
    }

    #[derive(Debug)]
    pub enum GetError {
        NotFound,
        NotAdress,
    }

    #[derive(Debug)]
    pub enum AddSessionError {
        NotLeaf,
        NotFound,
    }
}

#[cfg(test)]
mod test{
    use super::*;
    // Test str to pixel
    #[test]
    fn test_str_to_pixel() {
        let pixel = Pixel::from_string("001001-01");
        match pixel {
            Some(pixel) => {
                assert_eq!(pixel.x, 1);
                assert_eq!(pixel.y, 1);
                assert_eq!(pixel.color, -1);
            }
            None => panic!("pixel is None"),
        }
       // wrog length
        let pixel = Pixel::from_string("00100010101010101");
        assert!(pixel.is_none());

        // max than 1 Byte
        let pixel = Pixel::from_string("257-01000");
        assert!(pixel.is_none());

    }
    #[test]
    fn test_str_to_pixel_vec(){
        let pixels = Chunk::from_string("003000000\n");
        assert_eq!(pixels.len(), 1);
        println!("{:?}", pixels);
        assert!(pixels[0].x == 3);
        assert!(pixels[0].y == 0);
        assert!(pixels[0].color == 0);
        let pixels = Chunk::from_string("003020-20\n");
        assert_eq!(pixels.len(), 1);
        println!("{:?}", pixels);
        assert!(pixels[0].x == 3);
        assert!(pixels[0].y == 20);
        assert!(pixels[0].color == -20);

        // u8 oversize
        let pixels = Chunk::from_string("000380-25\n");
        assert_eq!(pixels.len(), 1);

        let pixelsize = Chunk::from_string("0000000380-925\n");
        assert_eq!(pixelsize.len(), 0);



    }

  
}



