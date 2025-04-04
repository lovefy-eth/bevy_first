use std::ops::{Deref, DerefMut};
use bevy::prelude::*;

// 血条
#[derive(Component)]
pub struct Health(pub f32);

impl Default for Health {
    fn default() -> Self {
        Health(100.)
    }
}

impl Deref for Health {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Health {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Component,Default)]
pub enum RoleState {
    #[default]
    Idle,
    Moving
}