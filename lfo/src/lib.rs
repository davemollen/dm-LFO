mod delta;
mod params;
mod phasor;
mod shared {
  pub mod float_ext;
}
pub use params::Params;
use {
  crate::shared::float_ext::FloatExt,
  delta::Delta,
  params::{LfoShape, Smoother},
  phasor::Phasor,
  std::f32::consts::{PI, TAU},
};

pub struct Lfo {
  phasor: Phasor,
  delta: Delta,
  is_enabled: bool,
  origin: f32,
  target: f32,
}

impl Lfo {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      phasor: Phasor::new(sample_rate),
      delta: Delta::new(),
      is_enabled: true,
      origin: 0.5,
      target: 0.5,
    }
  }

  pub fn process(&mut self, params: &mut Params) -> f32 {
    let Params {
      shape,
      offset,
      chance,
      ..
    } = *params;
    let freq = params.freq.next();
    let depth = params.depth.next();

    let phase = self.phasor.process(freq);
    let trigger = self.delta.process(phase) < 0.;
    if trigger {
      self.is_enabled = fastrand::f32() <= chance;
    }

    (match shape {
      LfoShape::Sine => {
        if !self.is_enabled {
          return 0.;
        }

        (phase * TAU).fast_sin()
      }
      LfoShape::Triangle => {
        if !self.is_enabled {
          return 0.;
        }

        let phase = Self::wrap(phase + 0.25);
        if phase > 0.5 {
          (phase - 0.5) * -4. + 1.
        } else {
          phase * 4. - 1.
        }
      }
      LfoShape::SawUp => {
        if !self.is_enabled {
          return 0.;
        }

        Self::wrap(phase + 0.5) * 2. - 1.
      }
      LfoShape::SawDown => {
        if !self.is_enabled {
          return 0.;
        }

        Self::wrap(1.5 - phase) * 2. - 1.
      }
      LfoShape::Rectangle => {
        if !self.is_enabled {
          return 0.;
        }

        if phase > 0.5 {
          1.
        } else {
          -1.
        }
      }
      LfoShape::SampleAndHold => {
        if trigger {
          self.target = if self.is_enabled {
            fastrand::f32() * 2. - 1.
          } else {
            0.
          };
        }
        self.target
      }
      LfoShape::Random => {
        if trigger {
          self.origin = self.target;
          self.target = if self.is_enabled {
            fastrand::f32() * 2. - 1.
          } else {
            0.
          };
        }
        self.linear_interp(phase)
      }
      LfoShape::CurvedRandom => {
        if trigger {
          self.origin = self.target;
          self.target = if self.is_enabled {
            fastrand::f32() * 2. - 1.
          } else {
            0.
          };
        }
        self.cosine_interp(phase)
      }
      LfoShape::Noise => {
        if fastrand::f32() <= chance {
          fastrand::f32() * 2. - 1.
        } else {
          0.
        }
      }
    } * depth
      + offset)
      .clamp(-1., 1.)
      * 20.
  }

  fn linear_interp(&self, mix: f32) -> f32 {
    self.origin + (self.target - self.origin) * mix
  }

  fn cosine_interp(&self, mix: f32) -> f32 {
    let cosine_mix = (1. - (mix * PI).fast_cos()) * 0.5;
    self.origin + (self.target - self.origin) * cosine_mix
  }

  fn wrap(x: f32) -> f32 {
    if x >= 1. {
      x - 1.
    } else {
      x
    }
  }
}
