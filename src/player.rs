use glam::{IVec2, UVec2, Vec2};
use raylib::prelude::*;

use crate::collisions::Bounded;
use crate::graphics::Graphics;
use crate::graphics::Textures;

pub struct Player {
    pub pos: IVec2,
    pub vel: Vec2,
}

impl Player {
    pub const SIZE: UVec2 = UVec2::new(32, 32);
    pub const STARTING_SPEED: f32 = 2.0;

    pub fn new(pos: IVec2) -> Player {
        Player {
            pos,
            vel: Vec2::new(Player::STARTING_SPEED, 0.0),
        }
    }

    pub fn render(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>, graphics: &mut Graphics) {
        let texture = &graphics.textures[Textures::Fetus as usize];
        d.draw_texture_pro(
            &graphics.textures[Textures::Fetus as usize],
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

impl Bounded for Player {
    fn get_pos(&self) -> IVec2 {
        self.pos
    }

    fn get_size(&self) -> UVec2 {
        Self::SIZE
    }
}
