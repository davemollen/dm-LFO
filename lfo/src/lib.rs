mod oscillator;
mod ramp_smooth;
mod shared {
  pub mod float_ext;
}
use {
  oscillator::{LfoShape, Oscillator},
  ramp_smooth::RampSmooth,
};

pub struct Lfo {
  smooth_freq: RampSmooth,
  smooth_depth: RampSmooth,
  smooth_curve: RampSmooth,
  oscillator: Oscillator,
}

impl Lfo {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      smooth_freq: RampSmooth::new(sample_rate, 12.),
      smooth_depth: RampSmooth::new(sample_rate, 12.),
      smooth_curve: RampSmooth::new(sample_rate, 12.),
      oscillator: Oscillator::new(sample_rate),
    }
  }

  pub fn initialize_params(&mut self, freq: f32, depth: f32, curve: f32, chance: f32) {
    self.smooth_freq.initialize(freq);
    self.smooth_depth.initialize(depth);
    self.smooth_curve.initialize(curve);
    self.oscillator.initialize(chance);
  }

  pub fn process(
    &mut self,
    freq: f32,
    depth: f32,
    shape: LfoShape,
    offset: f32,
    curve: f32,
    chance: f32,
  ) -> f32 {
    let freq = self.smooth_freq.process(freq);
    let depth = self.smooth_depth.process(depth);
    let curve = self.smooth_curve.process(curve);

    let lfo = (self.oscillator.process(freq, shape, chance) * depth + offset)
      .clamp(0., 1.)
      .powf(curve);

    Self::amplitude_to_cv(lfo)
  }

  fn amplitude_to_cv(amplitude: f32) -> f32 {
    amplitude * 10.
  }
}
