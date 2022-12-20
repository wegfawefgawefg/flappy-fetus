// pub enum Sounds {
//     Fetus,
//     Scizors,
//     Title,
//     GameOver,
//     Background,
//     WombWall,
//     Backdrop,
// }

use raylib::prelude::*;

pub enum Song {
    Title,
    GameOver,
    Playing,
}

pub struct Audio {
    pub rl_audio_device: RaylibAudio,
    pub songs: Vec<Music>,
}

impl Audio {
    pub fn new(rl: &mut RaylibHandle, rlt: &RaylibThread) -> Self {
        let rl_audio_device = RaylibAudio::init_audio_device();

        let error = "Error loading audio";
        let mut songs = Vec::new();
        let file_names = vec!["title", "gameover", "metal"];
        for name in file_names {
            let path = format!("assets/music/{}.ogg", name);
            let music = Music::load_music_stream(rlt, path.as_str()).expect(error);
            songs.push(music);
        }

        // rl_audio_device.update_music_stream(&mut songs[Song::Title as usize]);

        Self {
            rl_audio_device,
            songs,
        }
    }
}
