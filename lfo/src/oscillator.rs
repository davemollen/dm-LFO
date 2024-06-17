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
  Square,
  SampleAndHold,
  Random,
  CurvedRandom,
  Noise,
}

pub struct Oscillator {
  phasor: Phasor,
  delta: Delta,
  origin: f32,
  target: f32,
}

impl Oscillator {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      phasor: Phasor::new(sample_rate),
      delta: Delta::new(),
      origin: 0.,
      target: 0.,
    }
  }

  pub fn process(&mut self, freq: f32, shape: LfoShape, phase_offset: f32) -> f32 {
    let phase = Self::calculate_phase(self.phasor.process(freq), phase_offset);

    match shape {
      LfoShape::Sine => (phase * TAU).fast_sin(),
      LfoShape::Triangle => {
        if phase > 0.5 {
          phase * 4. - 1.
        } else {
          (phase - 0.5) * -4. + 1.
        }
      }
      LfoShape::SawDown => phase * 2. - 1.,
      LfoShape::SawUp => phase * -2. + 1.,
      LfoShape::Square => {
        if phase > 0.5 {
          1.
        } else {
          -1.
        }
      }
      LfoShape::SampleAndHold => {
        let trigger = self.delta.process(phase) < 0.;
        if trigger {
          self.target = fastrand::f32() * 2. - 1.;
        }
        self.target
      }
      LfoShape::Random => {
        let trigger = self.delta.process(phase) < 0.;
        if trigger {
          self.origin = self.target;
          self.target = fastrand::f32() * 2. - 1.;
        }
        self.linear_interp(phase)
      }
      LfoShape::CurvedRandom => {
        let trigger = self.delta.process(phase) < 0.;
        if trigger {
          self.origin = self.target;
          self.target = fastrand::f32() * 2. - 1.;
        }
        self.cosine_interp(phase)
      }
      LfoShape::Noise => fastrand::f32() * 2. - 1.,
    }
  }

  fn calculate_phase(phase: f32, offset: f32) -> f32 {
    let phase_offset = offset * 0.5 + 0.5;
    let new_phase = phase + phase_offset;
    if new_phase >= 1. {
      new_phase - 1.
    } else {
      new_phase
    }
  }

  fn linear_interp(&self, mix: f32) -> f32 {
    self.origin * (1. - mix) + self.target * mix
  }

  fn cosine_interp(&self, mix: f32) -> f32 {
    let cosine_mix = (1. - (mix * PI).fast_cos()) * 0.5;
    self.origin * (1. - cosine_mix) + self.target * cosine_mix
  }
}
