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
      origin: 0.5,
      target: 0.5,
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

    match shape {
      LfoShape::Sine => {
        let phase = if self.is_enabled { phase } else { 0.5 };
        (phase * TAU).fast_sin() * 0.5 + 0.5
      }
      LfoShape::Triangle => {
        let phase = if self.is_enabled {
          Self::wrap(phase + 0.25)
        } else {
          0.25
        };

        if phase > 0.5 {
          (phase - 0.5) * -2. + 1.
        } else {
          phase * 2.
        }
      }
      LfoShape::SawUp => {
        let phase = if self.is_enabled {
          Self::wrap(phase + 0.5)
        } else {
          0.5
        };
        phase
      }
      LfoShape::SawDown => {
        if self.is_enabled {
          Self::wrap(1.5 - phase)
        } else {
          0.
        }
      }
      LfoShape::Rectangle => {
        let phase = if self.is_enabled { phase } else { 0.5 };

        if phase > 0.5 {
          1.
        } else {
          0.
        }
      }
      LfoShape::SampleAndHold => {
        if trigger {
          self.target = if self.is_enabled {
            fastrand::f32()
          } else {
            0.5
          };
        }
        self.target
      }
      LfoShape::Random => {
        if trigger {
          self.origin = self.target;
          self.target = if self.is_enabled {
            fastrand::f32()
          } else {
            0.5
          };
        }
        self.linear_interp(phase)
      }
      LfoShape::CurvedRandom => {
        if trigger {
          self.origin = self.target;
          self.target = if self.is_enabled {
            fastrand::f32()
          } else {
            0.5
          };
        }
        self.cosine_interp(phase)
      }
      LfoShape::Noise => {
        if fastrand::f32() <= chance {
          fastrand::f32()
        } else {
          0.5
        }
      }
    }
  }

  fn linear_interp(&self, mix: f32) -> f32 {
    self.origin * (1. - mix) + self.target * mix
  }

  fn cosine_interp(&self, mix: f32) -> f32 {
    let cosine_mix = (1. - (mix * PI).fast_cos()) * 0.5;
    self.origin * (1. - cosine_mix) + self.target * cosine_mix
  }

  fn wrap(x: f32) -> f32 {
    if x >= 1. {
      x - 1.
    } else {
      x
    }
  }
}
