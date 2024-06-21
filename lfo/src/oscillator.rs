mod delta;
mod phasor;
use {
  crate::{shared::float_ext::FloatExt, LfoOutputMode, LfoShape},
  delta::Delta,
  phasor::Phasor,
  std::f32::consts::{PI, TAU},
};

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

  pub fn process(
    &mut self,
    freq: f32,
    shape: LfoShape,
    chance: f32,
    depth: f32,
    offset: f32,
    mode: LfoOutputMode,
  ) -> f32 {
    let phase = self.phasor.process(freq);
    let trigger = self.delta.process(phase) < 0.;
    if trigger {
      self.is_enabled = fastrand::f32() <= chance;
    }

    (match shape {
      LfoShape::Sine => {
        let phase_offset = match mode {
          LfoOutputMode::Bipolar => 0.,
          _ => 0.75,
        };
        let phase = if self.is_enabled { phase } else { 0.5 };
        ((phase + phase_offset) * TAU).fast_sin() * 0.5 + 0.5
      }
      LfoShape::Triangle => {
        let phase_offset = match mode {
          LfoOutputMode::Bipolar => 0.25,
          _ => 0.,
        };
        let phase = if self.is_enabled {
          Self::wrap(phase + phase_offset)
        } else {
          phase_offset
        };

        if phase > 0.5 {
          (phase - 0.5) * -2. + 1.
        } else {
          phase * 2.
        }
      }
      LfoShape::SawUp => {
        let (phase, phase_offset) = match mode {
          LfoOutputMode::Bipolar => (phase, 0.5),
          LfoOutputMode::UnipolarPositive => (phase, 0.),
          LfoOutputMode::UnipolarNegative => (1. - phase, 0.),
        };
        let phase = if self.is_enabled {
          Self::wrap(phase + phase_offset)
        } else {
          phase_offset
        };
        phase
      }
      LfoShape::SawDown => {
        let (phase, phase_offset) = match mode {
          LfoOutputMode::Bipolar => (1. - phase + 0.5, 0.5),
          LfoOutputMode::UnipolarPositive => (1. - phase, 0.),
          LfoOutputMode::UnipolarNegative => (phase, 0.),
        };

        if self.is_enabled {
          Self::wrap(phase + phase_offset)
        } else {
          phase_offset
        }
      }
      LfoShape::Rectangle => {
        let default_phase = match mode {
          LfoOutputMode::Bipolar => 0.5,
          _ => 0.,
        };

        if !self.is_enabled {
          return default_phase;
        }
        if phase > 0.5 {
          1.
        } else {
          0.
        }
      }
      LfoShape::SampleAndHold => {
        let default_phase = match mode {
          LfoOutputMode::Bipolar => 0.5,
          _ => 0.,
        };

        if trigger {
          self.target = if self.is_enabled {
            fastrand::f32()
          } else {
            default_phase
          };
        }
        self.target
      }
      LfoShape::Random => {
        let default_phase = match mode {
          LfoOutputMode::Bipolar => 0.5,
          _ => 0.,
        };

        if trigger {
          self.origin = self.target;
          self.target = if self.is_enabled {
            fastrand::f32()
          } else {
            default_phase
          };
        }
        self.linear_interp(phase)
      }
      LfoShape::CurvedRandom => {
        let default_phase = match mode {
          LfoOutputMode::Bipolar => 0.5,
          _ => 0.,
        };

        if trigger {
          self.origin = self.target;
          self.target = if self.is_enabled {
            fastrand::f32()
          } else {
            default_phase
          };
        }
        self.cosine_interp(phase)
      }
      LfoShape::Noise => {
        let default_phase = match mode {
          LfoOutputMode::Bipolar => 0.5,
          _ => 0.,
        };

        if fastrand::f32() <= chance {
          fastrand::f32()
        } else {
          default_phase
        }
      }
    } * depth
      + offset)
      .clamp(0., 1.)
      * 20.
      - 10.
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
