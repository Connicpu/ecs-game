use glium::{self, Surface, Program, VertexBuffer};
use glium::index::{NoIndices, PrimitiveType};
use cgmath::{Vector4, Matrix};
use GameData;
use systems::Services;
use components::GameComponents;
use ecs::{System, EntityIter};
use ecs::system::entity::EntityProcess;

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

pub struct DrawSprites {
    program: Option<Program>,
    vertices: Option<VertexBuffer<Vertex>>,
}

impl EntityProcess for DrawSprites {
    fn process(&mut self, entities: EntityIter<GameComponents>, data: &mut GameData) {
        if self.program.is_none() {
            self.initialize(&data.services);
        }
        
        let frame = data.services.frame.as_mut().unwrap();
        let program = self.program.as_ref().unwrap();
        let vertices = self.vertices.as_ref().unwrap();
        let cam_matrix = data.services.camera.matrix();
        
        for e in entities {
            let position = data.components.position[e];
            let sprite = &data.components.sprite[e];
            let tint = data.components.tint.get(&e)
                .map(|t| t.tint).unwrap_or(Vector4::new(1.0, 1.0, 1.0, 1.0));
            let frame_num = sprite.animation_frame();
            let matrix = sprite.matrix(&position.position, &cam_matrix);
            
            let uniforms = uniform! {
                matrix: Into::<[[f32; 4]; 4]>::into(matrix.transpose()),
                tex: sprite.texture.sampled().magnify_filter(
                    glium::uniforms::MagnifySamplerFilter::Nearest
                ),
                frame: frame_num,
                tint: [tint.x, tint.y, tint.z, tint.w],
            };
            
            frame.draw(
                vertices,
                NoIndices(PrimitiveType::TriangleStrip),
                program,
                &uniforms,
                &data.services.draw_params,
            ).unwrap();
        }
    }
}

impl DrawSprites {
    pub fn new() -> DrawSprites {
        DrawSprites {
            program: None,
            vertices: None,
        }
    }
    
    pub fn initialize(&mut self, services: &Services) {
        self.program = Some(program!(&services.display,
            330 => {
                vertex: include_str!("sprite_vs.glsl"),
                fragment: include_str!("sprite_fs.glsl"),
            },
        ).unwrap());
        
        self.vertices = Some(VertexBuffer::new(&services.display, &[
            Vertex { position: [-0.5,  0.5], tex_coords: [0.0, 1.0] },
            Vertex { position: [ 0.5,  0.5], tex_coords: [1.0, 1.0] },
            Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] },
            Vertex { position: [ 0.5, -0.5], tex_coords: [1.0, 0.0] },
        ]).unwrap());
    }
}

impl System for DrawSprites {
    type Components = GameComponents;
    type Services = Services;
}


