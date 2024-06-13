#![feature(portable_simd)]
mod lfo;
mod smooth_parameters;
mod shared {
  pub mod float_ext;
}
pub use lfo::LfoShape;
use {
  lfo::Lfo,
  smooth_parameters::SmoothParameters,
  std::simd::{f32x2, num::SimdFloat},
};

pub struct LfoMatrix {
  smooth_parameters: SmoothParameters,
  lfo1: Lfo,
  lfo2: Lfo,
  feedback: [f32; 2],
}

impl LfoMatrix {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      smooth_parameters: SmoothParameters::new(sample_rate),
      lfo1: Lfo::new(sample_rate),
      lfo2: Lfo::new(sample_rate),
      feedback: [0.; 2],
    }
  }

  pub fn initialize_params(
    &mut self,
    lfo1_to_lfo1: f32,
    lfo1_to_lfo2: f32,
    lfo1_to_out1: f32,
    lfo1_to_out2: f32,
    lfo2_to_lfo1: f32,
    lfo2_to_lfo2: f32,
    lfo2_to_out1: f32,
    lfo2_to_out2: f32,
  ) {
    self.smooth_parameters.initialize(
      lfo1_to_lfo1,
      lfo1_to_lfo2,
      lfo1_to_out1,
      lfo1_to_out2,
      lfo2_to_lfo1,
      lfo2_to_lfo2,
      lfo2_to_out1,
      lfo2_to_out2,
    );
  }

  pub fn process(
    &mut self,
    lfo1_on: bool,
    lfo1_freq: f32,
    lfo1_shape: LfoShape,
    lfo1_to_lfo1: f32,
    lfo1_to_lfo2: f32,
    lfo1_to_out1: f32,
    lfo1_to_out2: f32,
    lfo2_on: bool,
    lfo2_freq: f32,
    lfo2_shape: LfoShape,
    lfo2_to_lfo1: f32,
    lfo2_to_lfo2: f32,
    lfo2_to_out1: f32,
    lfo2_to_out2: f32,
  ) -> (f32, f32) {
    let lfo1 = if lfo1_on {
      self.lfo1.process(lfo1_freq, lfo1_shape, self.feedback[0])
    } else {
      0.
    };
    let lfo2 = if lfo2_on {
      self.lfo2.process(lfo2_freq, lfo2_shape, self.feedback[1])
    } else {
      0.
    };
    let lfos = f32x2::from_array([lfo1, lfo2]);

    let (feedback_matrix, output_matrix) = self.smooth_parameters.process(
      lfo1_to_lfo1,
      lfo1_to_lfo2,
      lfo1_to_out1,
      lfo1_to_out2,
      lfo2_to_lfo1,
      lfo2_to_lfo2,
      lfo2_to_out1,
      lfo2_to_out2,
    );

    self.feedback = [
      (lfos * feedback_matrix[0]).reduce_sum(),
      (lfos * feedback_matrix[1]).reduce_sum(),
    ];

    (
      (lfos * output_matrix[0]).reduce_sum(),
      (lfos * output_matrix[1]).reduce_sum(),
    )
  }
}
