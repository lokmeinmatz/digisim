use std::marker::PhantomData;
use std::collections::HashMap;
use ggez::graphics::{Mesh, MeshBuilder};
use ggez::{Context};
use std::hash::{Hash, Hasher};
use crate::entities::World;
use std::collections::hash_map::{Values, ValuesMut};

#[derive(Debug)]
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

impl<T> Copy for Id<T> {}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Id<T> {
        *self
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


    /// Returns the element if exists, None otherwise
    pub fn get(&self, id: Id<T>) -> Option<&T> {
        self.data.get(&id)
    }

    pub fn iter(&self) -> Values<Id<T>, T> {
        self.data.values()
    }

    pub fn iter_mut(&mut self) -> ValuesMut<Id<T>, T> {
        self.data.values_mut()
    }
}


pub trait Entity {
    fn render(&self, world: &World, ctx: &mut Context)-> ggez::GameResult<()>;
    fn update(&mut self, world: &mut World, ctx: &mut Context) -> ggez::GameResult<()>;
}

