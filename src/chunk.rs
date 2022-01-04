use bincode;
use flate2::{write::ZlibEncoder, Compression};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs::{OpenOptions, self};
use std::sync::Mutex;
use std::{fs::File, io::BufWriter};

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
        pixels: Vec<Vec<Pixel>>,
    },
}

impl Chunk {
    pub fn new_root() -> Self {
        Self::Container {
            id: String::from("map"),
            childrens: vec![Self::new_leaf("map", 0).unwrap()],
        }
    }

    pub fn new_leaf(parent_adress: &str, parent_length: u32) -> Option<Self> {
        match fs::create_dir_all(format!("./{}", parent_adress)){
            Ok(_) => (),
            Err(e) => {
                println!("{:?}", e);
                return None;
            }
        }
        let id = format!("./{}/{}", parent_adress, parent_length);
        let leaf = Self::Leaf {
            id,
            pixels: vec![vec![Pixel::default(); SIZE]; SIZE],
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
#[derive(Debug, Serialize, Deserialize, Clone)]
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

pub(crate) static MAP: Lazy<Mutex<Chunk>> = Lazy::new(|| Mutex::new(Chunk::new_root()));

use errors::*;

impl Chunk {
    pub fn save(&self) -> Result<(), SaveError> {
        match self {
            Chunk::Container { id: _, childrens } => {
                for child in childrens.iter() {
                    child.save()?;
                }
                Ok(())
            }
            Chunk::Leaf { id, pixels } => {
                let file = match OpenOptions::new().write(true).create(true).truncate(true).open("".to_owned() + id + ".chunk.gz") {
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

    pub fn add_pixel(&mut self, pixels_adds: &[Pixel], adress: &str) -> Result<(), AddError> {
        match self {
            Chunk::Container { id: _, childrens } => {
                for child in childrens.iter_mut() {
                    child.add_pixel(pixels_adds, adress)?;
                }
                Ok(())
            }
            Chunk::Leaf { id: _, pixels } => {
                for pixel in pixels_adds.iter() {
                    pixels[pixel.x as usize][pixel.y as usize].color = pixel.color;
                    
                }
                Ok(())
            }
        }
    }

    pub fn get_chunk(&self, adress: &str) -> Result<Chunk, GetError> {
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
            Chunk::Leaf { id, pixels: _ } => {
                if id == adress {
                    Ok((*self).clone())
                } else {
                    Err(GetError::NotAdress)
                }
            }
        }
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
}
