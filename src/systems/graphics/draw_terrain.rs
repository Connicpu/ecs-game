use glium::{self, Surface, Program, VertexBuffer};
use glium::index::{NoIndices, PrimitiveType};
use cgmath::Matrix;
use GameData;
use systems::Services;
use components::GameComponents;
use ecs::{System, Process};

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

#[derive(Copy, Clone, Debug)]
struct Instance {
    offset: [f32; 2],
    tile: u32,
}

implement_vertex!(Instance, offset, tile);

pub struct DrawTerrain {
    program: Option<Program>,
    vertices: Option<VertexBuffer<Vertex>>,
    instanced: Option<VertexBuffer<Instance>>,
}

impl Process for DrawTerrain {
    fn process(&mut self, data: &mut GameData) {
        if self.program.is_none() {
            self.initialize(&data.services);
        }
        
        if data.services.tilemap_changed || self.instanced.is_none() {
            self.setup_tiles(&data.services);
            data.services.tilemap_changed = false;
        }
        
        let frame = data.services.frame.as_mut().unwrap();
        let program = self.program.as_ref().unwrap();
        let vertices = self.vertices.as_ref().unwrap();
        let instanced = self.instanced.as_ref().unwrap().per_instance().unwrap();
        let cam_matrix = data.services.camera.matrix();
        
        let uniforms = uniform! {
            matrix: Into::<[[f32; 4]; 4]>::into(cam_matrix.transpose()),
            tex: data.services.tileset.sampled().magnify_filter(
                glium::uniforms::MagnifySamplerFilter::Nearest
            ),
        };
        
        frame.draw(
            (vertices, instanced),
            NoIndices(PrimitiveType::TriangleStrip),
            program,
            &uniforms,
            &data.services.draw_params,
        ).unwrap();
    }
}

impl DrawTerrain {
    pub fn new() -> DrawTerrain {
        DrawTerrain {
            program: None,
            vertices: None,
            instanced: None,
        }
    }
    
    pub fn initialize(&mut self, services: &Services) {
        self.program = Some(program!(&services.display,
            140 => {
                vertex: include_str!("terrain_vs.glsl"),
                fragment: include_str!("terrain_fs.glsl"),
            },
        ).unwrap());
        
        self.vertices = Some(VertexBuffer::new(&services.display, &[
            Vertex { position: [-0.5,  0.5], tex_coords: [0.0, 0.0] },
            Vertex { position: [ 0.5,  0.5], tex_coords: [1.0, 0.0] },
            Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 1.0] },
            Vertex { position: [ 0.5, -0.5], tex_coords: [1.0, 1.0] },
        ]).unwrap());
    }
    
    pub fn setup_tiles(&mut self, services: &Services) {
        use world::tilemap::{Tile};
        
        self.instanced = None;
        
        let width = services.tilemap.width();
        let height = services.tilemap.height();
        let tiles = services.tilemap.tiles();
        let instance_list: Vec<_> = tiles.iter().enumerate().filter(|&(_, &tile)| {
            tile != Tile::Open
        }).map(|(i, &tile)| {
            let x = i as u32 % width;
            let y = height - i as u32 / width;
            let id = match tile {
                Tile::Open => unreachable!(),
                Tile::Wall => 0,
                Tile::Breakable(_) => 1,
            };
            Instance {
                offset: [x as f32, 1.0 - (y as f32)],
                tile: id,
            }
        }).collect();
        
        println!("{} tiles", instance_list.len());
        
        let instanced = VertexBuffer::immutable(&services.display, &instance_list).unwrap();
        self.instanced = Some(instanced);
    }
}

impl System for DrawTerrain {
    type Components = GameComponents;
    type Services = Services;
}
