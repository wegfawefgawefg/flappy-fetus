use rand::seq::SliceRandom;
use raylib::prelude::*;

pub enum Song {
    Title,
    GameOver,
    Playing,
}

#[derive(Copy, Clone)]
pub enum SoundEffect {
    SmackOne,
    SmackTwo,
    EndScreamOne,
    EndScreamTwo,
    EndScreamThree,
}

pub struct Audio {
    pub rl_audio_device: RaylibAudio,
    pub songs: Vec<Music>,
    pub sounds: Vec<Sound>,
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

        let error = "Error loading audio";
        let mut sounds = Vec::new();
        let file_names = vec![
            "smack_one",
            "smack_two",
            "end_scream_one",
            "end_scream_two",
            "end_scream_three",
        ];
        for name in file_names {
            let path = format!("assets/sounds/{}.ogg", name);
            // let music = Music::load_music_stream(rlt, path.as_str()).expect(error);
            let sound = Sound::load_sound(path.as_str()).expect(error);
            sounds.push(sound);
        }

        Self {
            rl_audio_device,
            songs,
            sounds,
        }
    }

    pub fn play_random_smack_sound(&mut self) {
        let sound_effect_options = [SoundEffect::SmackOne, SoundEffect::SmackTwo];
        let sound_effect = sound_effect_options
            .choose(&mut rand::thread_rng())
            .unwrap();
        let sound_effect = &mut self.sounds[*sound_effect as usize];
        self.rl_audio_device.play_sound(sound_effect);
    }

    pub fn play_random_end_scream_sound(&mut self) {
        let sound_effect_options = [
            SoundEffect::EndScreamOne,
            SoundEffect::EndScreamTwo,
            SoundEffect::EndScreamThree,
        ];
        let sound_effect = sound_effect_options
            .choose(&mut rand::thread_rng())
            .unwrap();
        let sound_effect = &mut self.sounds[*sound_effect as usize];
        self.rl_audio_device.play_sound(sound_effect);
    }
}
