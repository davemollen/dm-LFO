#![feature(portable_simd)]
mod oscillator;
mod ramp_smooth;
mod shared {
  pub mod float_ext;
}
use {
  oscillator::{LfoShape, Oscillator},
  ramp_smooth::RampSmooth,
};

#[derive(Clone, Copy)]
pub enum InputMode {
  Add,
  SubtractA,
  SubtractB,
  Multiply,
  FM,
  PM,
}

pub struct Lfo {
  smooth_freq: RampSmooth,
  smooth_depth: RampSmooth,
  oscillator: Oscillator,
}

impl Lfo {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      smooth_freq: RampSmooth::new(sample_rate, 12.),
      smooth_depth: RampSmooth::new(sample_rate, 12.),
      oscillator: Oscillator::new(sample_rate),
    }
  }

  pub fn initialize_params(&mut self, freq: f32, depth: f32) {
    self.smooth_freq.initialize(freq);
    self.smooth_depth.initialize(depth);
  }

  pub fn process(
    &mut self,
    input: f32,
    input_mode: InputMode,
    freq: f32,
    depth: f32,
    shape: LfoShape,
  ) -> f32 {
    let freq = self.smooth_freq.process(freq);
    let depth = self.smooth_depth.process(depth);

    let lfo = match input_mode {
      InputMode::Add => self.oscillator.process(freq, shape, 0.) * depth + input,
      InputMode::SubtractA => self.oscillator.process(freq, shape, 0.) * depth - input,
      InputMode::SubtractB => input - self.oscillator.process(freq, shape, 0.) * depth,
      InputMode::Multiply => self.oscillator.process(freq, shape, 0.) * depth * input,
      InputMode::FM => self.oscillator.process(freq * input, shape, 0.) * depth,
      InputMode::PM => self.oscillator.process(freq, shape, input),
    };

    Self::amplitude_to_cv(lfo)
  }

  fn amplitude_to_cv(amplitude: f32) -> f32 {
    amplitude * 10.
  }
}
