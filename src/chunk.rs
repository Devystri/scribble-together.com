use bincode;
use flate2::{write::ZlibEncoder, Compression};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::{fs::File, io::BufWriter};

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
        pixels: Vec<Pixel>,
    },
}

impl Chunk {
    pub fn new_root() -> Self {
        Self::Container {
            id: String::new(),
            childrens: vec![Self::new_leaf()],
        }
    }

    pub fn new_leaf() -> Self {
        Self::Leaf {
            id: String::from("0"),
            pixels: Vec::new(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pixel {
    pub x: u8,
    pub y: u8,
    pub color: i8,
}

impl Pixel {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            color: -1,
        }
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
                let file = match File::create("".to_owned() + &id.to_string() + ".bin.gz") {
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

    pub fn add_pixel(&mut self, pixels_adds: &Vec<Pixel>, adress: &str) -> Result<(), AddError> {
        match self {
            Chunk::Container { id: _, childrens } => {
                for child in childrens.iter_mut() {
                    child.add_pixel(&pixels_adds, adress)?;
                }
                Ok(())
            }
            Chunk::Leaf { id: _, pixels } => {
                println!("{:?}", pixels_adds.clone());
                for  pixel in pixels_adds.iter() {
                    pixels.push((*pixel).clone());
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
