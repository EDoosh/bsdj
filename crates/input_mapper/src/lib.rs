use crate::input_handler::InputHandler;
use bevy::prelude::*;
use std::fmt::Debug;
use std::hash::Hash;

pub mod input_handler;
pub mod input_parser;
pub mod inputs;

pub struct CustomInputPlugin<'a, T>(std::marker::PhantomData<&'a T>);

impl<'a, T> Default for CustomInputPlugin<'a, T> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<'a, T> Plugin for CustomInputPlugin<'a, T>
where
    InputHandler<T>: Default,
    T: Hash + Eq + Clone + Send + Sync + Debug,
    'a: 'static,
{
    fn build(&self, app: &mut App) {
        app.init_resource::<InputHandler<T>>();
    }
}
