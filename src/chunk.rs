
use std::{fs::File};
use serde::{Serialize, Deserialize};
use bincode;
use once_cell::sync::Lazy; 
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Chunk {
    Container{
        #[serde(skip)]
        id: String,
        childrens: Vec<Chunk>,
    },
    Leaf{
        #[serde(skip)]
        id: String,
        pixels: Vec<Pixel>,
    },
    
}

impl Chunk{
    pub fn new_root() -> Self {
       Self::Container{
            id: String::new(),
            childrens: vec![Self::new_leaf()],
        }
    }

    pub fn new_leaf() -> Self{
        Self::Leaf{
            id: String::from("0"),
            pixels: Vec::new(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pixel{
    pub x: u8,
    pub y: u8,
    pub color: i8,
}

impl Pixel{
    pub fn new() -> Self{
        Self{
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
            Chunk::Container { id:_, childrens } => {
                for child in childrens.iter() {
                    child.save()?;
                }
                Ok(())
            },
            Chunk::Leaf { id, pixels } => {
                let file = File::create("".to_owned() +  &id.to_string() + ".bin");

                let file = match file {
                    Ok(file) => file,
                    Err(e) => return Err(SaveError::Io(e)),
                };
                
                match bincode::serialize_into(&file, pixels) {
                    Ok(_) =>   Ok(()),
                    Err(e) => Err(SaveError::Bincode(e)),
                }
            },
        }   
        
    }

    pub fn add_pixel(&mut self, pixel: &Pixel, adress: &str) -> Result<(), AddError> {
        match self {
            Chunk::Container { id:_, childrens } => {
                for child in childrens.iter_mut() {
                    child.add_pixel(&pixel, adress)?;
                }
                Ok(())
            },
            Chunk::Leaf { id:_,  pixels } => {
                println!("{:?}", pixel.clone());
                pixels.push((*pixel).clone());
                Ok(())
            },
        }   
        
    }

    pub fn get_chunk(&self, adress: &String) -> Result<Chunk, GetError> {
        match self {
            Chunk::Container { id, childrens } => {
                for child in childrens.iter() {
                    match child.get_chunk(adress){
                        Ok(chunk) => return Ok(chunk),
                        Err(e) => {
                            if let GetError::NotAdress = e {
                                continue;
                            }else{
                                return Err(e);
                            }
                        },
                    }
                }
                Err(GetError::NotFound)
            },
            Chunk::Leaf { id,  pixels:_ } => {
                if id == adress {
                    Ok((*self).clone())
                } else {
                    Err(GetError::NotAdress)
                }
            },  
        }   
        
    }
        
}
pub mod errors{
    use super::*;
    #[derive(Debug)]
    pub enum SaveError{
        Io(std::io::Error),
        Bincode(bincode::Error),
    } 
    #[derive(Debug)]
    pub enum AddError{
        NotLeaf,
    }
    
    #[derive(Debug)]
    pub enum GetError{
        NotFound,
        NotAdress
    }
        
    
}