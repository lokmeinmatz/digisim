use crate::ecs::{Id, Entity, Storage};
use ggez::Context;
use ggez::graphics::{Mesh, MeshBuilder, DrawMode, Rect, Color, DrawParam};
use crate::Vec2;


pub struct World {
    meshes: Storage<Mesh>,
    entities: Storage<Box<dyn Entity>>,

}

impl World {
    pub fn new(ctx: &mut Context) -> World {
        let mut w = World {
            meshes: Storage::new(),
            entities: Storage::new()
        };

        let cb = CheckerBoard::new(ctx, &mut w.meshes);
        w.entities.push(Box::new(cb));
        w
    }

    pub fn render(&self, ctx: &mut Context) -> ggez::GameResult<()> {
        for entity in self.entities.iter() {
            (*entity).render(self, ctx)?;
        }

        Ok(())
    }
}


pub struct CheckerBoard {
    mesh_id: Id<Mesh>
}

impl CheckerBoard {
    pub fn new(ctx: &mut Context, meshes: &mut Storage<Mesh>) -> CheckerBoard {

        let mut mesh_b = MeshBuilder::new();
        let white = Color::from_rgb(255, 255, 255);
        for row in 0..8 {
            for col in ((row % 2)..8).step_by(2) {
                mesh_b.rectangle(DrawMode::fill(), Rect::new(col as f32, row as f32, 1.0, 1.0), white);
            }
        }


        let mesh_id = meshes.push(mesh_b.build(ctx).unwrap());

        CheckerBoard {
            mesh_id
        }

    }
}

impl Entity for CheckerBoard {
    fn render(&self, world: &World, ctx: &mut Context) -> ggez::GameResult<()> {
        ggez::graphics::draw(ctx, world.meshes.get(self.mesh_id).unwrap(), DrawParam::default())?;

        Ok(())
    }

    fn update(&mut self, world: &mut World, ctx: &mut Context) -> ggez::GameResult<()> {
        Ok(())
    }
}