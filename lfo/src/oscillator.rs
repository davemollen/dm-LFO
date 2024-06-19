mod delta;
mod phasor;
use {
  crate::shared::float_ext::FloatExt,
  delta::Delta,
  phasor::Phasor,
  std::f32::consts::{PI, TAU},
};

#[derive(Clone, Copy)]
pub enum LfoShape {
  Sine,
  Triangle,
  SawUp,
  SawDown,
  Rectangle,
  SampleAndHold,
  Random,
  CurvedRandom,
  Noise,
}

pub struct Oscillator {
  phasor: Phasor,
  delta: Delta,
  is_enabled: bool,
  origin: f32,
  target: f32,
}

impl Oscillator {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      phasor: Phasor::new(sample_rate),
      delta: Delta::new(),
      is_enabled: true,
      origin: 0.,
      target: 0.,
    }
  }

  pub fn initialize(&mut self, chance: f32) {
    self.is_enabled = fastrand::f32() <= chance;
  }

  pub fn process(&mut self, freq: f32, shape: LfoShape, chance: f32) -> f32 {
    let phase = self.phasor.process(freq);
    let trigger = self.delta.process(phase) < 0.;
    if trigger {
      self.is_enabled = fastrand::f32() <= chance;
    }
    let phase = if self.is_enabled { phase } else { 0. };

    let wave = match shape {
      LfoShape::Sine => (phase * TAU).fast_sin(),
      LfoShape::Triangle => {
        if phase > 0.5 {
          (phase - 0.5) * -2. + 1.
        } else {
          phase * 2.
        }
      }
      LfoShape::SawDown => 1. - phase,
      LfoShape::SawUp => phase,
      LfoShape::Rectangle => {
        if phase > 0.5 {
          1.
        } else {
          0.
        }
      }
      LfoShape::SampleAndHold => {
        if trigger {
          self.target = fastrand::f32();
        }
        self.target
      }
      LfoShape::Random => {
        if trigger {
          self.origin = self.target;
          self.target = fastrand::f32();
        }
        self.linear_interp(phase)
      }
      LfoShape::CurvedRandom => {
        if trigger {
          self.origin = self.target;
          self.target = fastrand::f32();
        }
        self.cosine_interp(phase)
      }
      LfoShape::Noise => fastrand::f32(),
    };

    wave * 2. - 1.
  }

  fn linear_interp(&self, mix: f32) -> f32 {
    self.origin * (1. - mix) + self.target * mix
  }

  fn cosine_interp(&self, mix: f32) -> f32 {
    let cosine_mix = (1. - (mix * PI).fast_cos()) * 0.5;
    self.origin * (1. - cosine_mix) + self.target * cosine_mix
  }
}
