use std::marker::PhantomData;
use std::collections::HashMap;
use ggez::graphics::{Mesh, MeshBuilder};
use ggez::{Context};
use std::hash::{Hash, Hasher};


#[derive(Debug, Clone, Copy)]
pub struct Id<T> (usize, PhantomData<T>);

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Id<T> {}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

pub struct Storage<T> {
    data: HashMap<Id<T>, T>,
    next_free_id: Id<T>
}


impl<T> Storage<T> {
    pub fn new() -> Self {
        Storage {
            data: HashMap::new(),
            next_free_id: Id(0, PhantomData)
        }
    }

    pub fn push(&mut self, component: T) -> Id<T> {
        assert!(self.data.insert(self.next_free_id, component).is_none());
        let this_id = self.next_free_id;
        self.next_free_id = Id(this_id.0 + 1, PhantomData);
        this_id
    }
}

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

        let grid = Grid::new(ctx, &mut w.meshes);
        w.entities.push(Box::new(grid));
        w
    }
}

pub trait Entity {
    fn render(&mut self, world: &World, ctx: &mut Context)-> ggez::GameResult<()>;
    fn update(&mut self, world: &mut World, ctx: &mut Context) -> ggez::GameResult<()>;
}


pub struct Grid {
    mesh_id: Id<Mesh>
}

impl Grid {
    pub fn new(ctx: &mut Context, meshes: &mut Storage<Mesh>) -> Grid {

        let mesh = MeshBuilder::new().build(ctx).unwrap();

    }
}

impl Entity for Grid {
    fn render(&mut self, world: &World, ctx: &mut Context) -> ggez::GameResult<()> {

    }

    fn update(&mut self, world: &mut World, ctx: &mut Context) -> ggez::GameResult<()> {

    }
}

pub fn render(world: &World, ctx: &mut Context) -> ggez::GameResult<()> {
    Ok(())
}