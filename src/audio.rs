use sdl2;
use sdl2::audio::{ AudioDevice, AudioCallback, AudioSpecDesired };

// Acts as the callback that AudioDevice uses to play sounds
struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume:f32,
}

pub struct Audio {
    device: AudioDevice<SquareWave>,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

impl Audio {
    pub fn new(ctx: &sdl2::Sdl) -> Audio {
        let audio_subsystem = ctx.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        };

        let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25
            }
        }).unwrap();

        Audio {
            device: device,
        }
    }

    pub fn play(&self) {
        self.device.resume();
    }

    pub fn stop(&self) {
        self.device.pause();
    }
}
