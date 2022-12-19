use glam::{IVec2, UVec2};
use raylib::prelude::*;

use crate::{
    collisions::Bounded,
    graphics::{Graphics, Textures},
};

pub struct Obstacle {
    pub pos: IVec2,
    pub size: UVec2,
}

impl Obstacle {
    pub const SIZE: UVec2 = UVec2::new(64, 32);

    pub fn new(pos: IVec2, size: UVec2) -> Obstacle {
        Obstacle { pos, size }
    }

    pub fn render(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>, graphics: &mut Graphics) {
        let texture = &graphics.textures[Textures::Scizors as usize];
        d.draw_texture_pro(
            &texture,
            Rectangle::new(0.0, 0.0, texture.width() as f32, texture.height() as f32),
            Rectangle::new(
                self.pos.x as f32,
                self.pos.y as f32,
                Self::SIZE.x as f32,
                Self::SIZE.y as f32,
            ),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::WHITE,
        );
    }
}

impl Bounded for Obstacle {
    fn get_pos(&self) -> IVec2 {
        self.pos
    }

    fn get_size(&self) -> UVec2 {
        Self::SIZE
    }
}
