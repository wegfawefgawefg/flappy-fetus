use glam::{IVec2, Vec2};

use crate::{obstacle::Obstacle, player::Player, step};

pub enum Mode {
    Title,
    Playing,
    GameOver,
}

pub struct State {
    pub mode: Mode,

    pub running: bool,
    pub now: f64,
    pub time_since_last_update: f32,

    pub play_time: f32,
    pub score: f32,

    pub pause: bool,
    pub win: bool,

    pub player: Player,
    pub obstacles: Vec<Obstacle>,
    pub obstacle_spawn_frame_countdown_timer: u32,
    pub obstacle_spawn_period_in_frames: u32,
}

impl State {
    pub const STARTING_OBSTACLE_SPAWN_FRAME_PERIOD: u32 = 1 * step::FRAMES_PER_SECOND;
    pub fn new() -> State {
        State {
            mode: Mode::Title,

            running: true,
            now: 0.0,
            time_since_last_update: 0.0,

            play_time: 0.0,
            score: 0.0,

            pause: false,
            win: false,

            player: Player::new(glam::IVec2 { x: 0, y: 0 }),
            obstacles: Vec::new(),
            obstacle_spawn_frame_countdown_timer: State::STARTING_OBSTACLE_SPAWN_FRAME_PERIOD,
            obstacle_spawn_period_in_frames: State::STARTING_OBSTACLE_SPAWN_FRAME_PERIOD,
        }
    }

    pub fn reset(&mut self) {
        self.player.pos = IVec2 { x: 0, y: 0 };
        self.player.vel = Vec2 {
            x: Player::STARTING_SPEED,
            y: 0.0,
        };
        self.obstacles.clear();
        self.obstacle_spawn_frame_countdown_timer = State::STARTING_OBSTACLE_SPAWN_FRAME_PERIOD;
        self.score = 0.0;
    }
}
