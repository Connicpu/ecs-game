use std::io::{self, BufRead};
use std::str::FromStr;
use rustc_serialize::{Encoder, Decoder, Encodable, Decodable};
use world::item::Item;
use world::entities::EntityType;

#[derive(Copy, Clone, Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub enum InputTile {
    Open,
    Wall,
    Spawn(u8),
    Item(char),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tile {
    Open,
    Wall,
    Breakable(Item),
}

#[derive(Clone, Debug)]
pub struct Tilemap {
    width: u32,
    height: u32,
    collision_map: Vec<bool>,
    tile_map: Vec<Tile>,
    spawns: Vec<(EntityType, u32, u32)>,
}

impl Tilemap {
    pub fn width(&self) -> u32 {
        self.width
    }
    
    pub fn height(&self) -> u32 {
        self.height
    }
    
    pub fn filled_at(&self, row: u32, col: u32) -> bool {
        self.collision_map[(row * self.width + col) as usize]
    }
    
    pub fn tile_at(&self, row: u32, col: u32) -> &Tile {
        &self.tile_map[(row * self.width + col) as usize]
    }
    
    pub fn tile_at_mut(&mut self, row: u32, col: u32) -> &mut Tile {
        &mut self.tile_map[(row * self.width + col) as usize]
    }
    
    pub fn spawns(&self) -> &[(EntityType, u32, u32)] {
        &self.spawns
    }
    
    pub fn parse_text_map<R: BufRead>(reader: R) -> Res<Tilemap> {
        let (width, height, input_tiles) = try!(Tilemap::parse_text_map_input(reader));
        Tilemap::parse_input(width, height, &input_tiles)
    }
    
    pub fn parse_encode<R: BufRead, E: Encoder>(
        reader: R, encoder: &mut E
    ) -> Res<Result<(), E::Error>> {
        let result = try!(Tilemap::parse_text_map_input(reader));
        Ok(result.encode(encoder))
    }
    
    pub fn decode_parse<D: Decoder>(decoder: &mut D) -> Result<Res<Tilemap>, D::Error> {
        let result = try!(<(u32, u32, Vec<InputTile>) as Decodable>::decode(decoder));
        let (width, height, input_tiles) = result;
        Ok(Tilemap::parse_input(width, height, &input_tiles))
    }
    
    fn parse_input(width: u32, height: u32, input_tiles: &[InputTile]) -> Res<Tilemap> {
        let collision_map = input_tiles.iter().map(|&tile| {
            match tile {
                InputTile::Open => false,
                InputTile::Wall => true,
                InputTile::Spawn(_) => false,
                InputTile::Item(_) => true,
            }
        }).collect();
        
        let tile_map: Result<_, Error> = input_tiles.iter().map(|&tile| {
            Ok(match tile {
                InputTile::Open => Tile::Open,
                InputTile::Wall => Tile::Wall,
                InputTile::Spawn(_) => Tile::Open,
                InputTile::Item(id) => Tile::Breakable(try!(Item::parse(id)))
            })
        }).collect();
        
        let spawns: Result<_, Error> = input_tiles.iter().enumerate().filter_map(|(i, &tile)| {
            let x = i as u32 % width;
            let y = i as u32 / width;
            match tile {
                InputTile::Spawn(id) => Some((id, x, y)),
                _ => None,
            }
        }).map(|(id, x, y)| {
            let entity = try!(EntityType::parse(id));
            Ok((entity, x, y))
        }).collect();
        
        Ok(Tilemap {
            width: width,
            height: height,
            collision_map: collision_map,
            tile_map: try!(tile_map),
            spawns: try!(spawns),
        })
    }
    
    fn parse_text_map_input<R: BufRead>(mut reader: R) -> Res<(u32, u32, Vec<InputTile>)> {
        let mut line = String::new();
        
        // Parse width, height
        try!(reader.read_line(&mut line));
        let width;
        let height;
        {
            let mut split = line.split_whitespace();
            let height_s = try!(split.next().ok_or(Error::BadMapData));
            let width_s = try!(split.next().ok_or(Error::BadMapData));
            width = try!(u32::from_str(width_s).map_err(|_| Error::BadMapData));
            height = try!(u32::from_str(height_s).map_err(|_| Error::BadMapData));
        }
        
        // Parse map lines
        let mut tiles = Vec::new();
        for c in reader.chars() {
            let c = try!(c);
            match c {
                '\n' | '\r' => {
                    let len = tiles.len() as u32;
                    if len % width != 0 || len > width * height {
                        return Err(Error::BadMapSize);
                    }
                    if len == width * height {
                        break;
                    }
                },
                // Open air
                '_' => {
                    tiles.push(InputTile::Open);
                },
                // Walls
                '#' => {
                    tiles.push(InputTile::Wall);
                },
                // Entity spawns
                c if c.is_digit(10) => {
                    let id = c.to_digit(10).unwrap() as u8;
                    tiles.push(InputTile::Spawn(id));
                },
                // Anything else must be an item block
                c => {
                    tiles.push(InputTile::Item(c));
                }
            }
        }
        
        Ok((width, height, tiles))
    }
}

pub type Res<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BadMapData,
    BadMapSize,
    InvalidItem(char),
    InvalidEntity(u8),
    NotUtf8,
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(io: io::Error) -> Error {
        Error::Io(io)
    }
}

impl From<io::CharsError> for Error {
    fn from(e: io::CharsError) -> Error {
        match e {
            io::CharsError::NotUtf8 => Error::NotUtf8,
            io::CharsError::Other(e) => From::from(e),
        }
    }
}

impl From<char> for Error {
    fn from(c: char) -> Error {
        Error::InvalidItem(c)
    }
}

impl From<u8> for Error {
    fn from(id: u8) -> Error {
        Error::InvalidEntity(id)
    }
}

pub fn load_map(path: &str) -> Tilemap {
    use std::fs::File;
    use std::io::BufReader;
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);
    Tilemap::parse_text_map(buf).unwrap()
}


