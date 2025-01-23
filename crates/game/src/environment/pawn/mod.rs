// Copyright 2025 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

mod movement;
pub use movement::*;

#[derive(Debug, Clone, Component)]
pub struct Pawn {
    collider: Circle,
    origin: Vec2,
    layers: u16,
}

impl Pawn {

    #[must_use]
    pub fn new(
        origin: Vec2, 
        size: f32,
        layers: u16
    ) -> Self {
        Self { 
            collider: Circle::new(size/2.0),
            origin,
            layers
        }
    }   

    #[must_use]
    pub const fn collider(&self) -> Circle {
        self.collider
    }

    #[must_use]
    pub const fn origin(&self) -> Vec2 {
        self.origin
    }

    pub const fn set_origin(&mut self, v: Vec2) {
        self.origin = v;
    }

    #[must_use]
    pub const fn radius(&self) -> f32 {
        self.collider.radius
    }

    #[must_use]
    pub const fn layers(&self) -> u16 {
        self.layers
    }
    
}

pub fn sync_pawn_transform(
    mut q_pawns: Query<(&mut Transform, &Pawn)>,
    // r_ppu: Res<PixelsPerUnit>
) {
    q_pawns.iter_mut().for_each(|(mut t, p)| t.translation = p.origin()
        // .map(|v| ppu_snap_to(*r_ppu, v))
        .extend(t.translation.z)
    );
}
