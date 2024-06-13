mod ramp_smooth;
use {ramp_smooth::RampSmooth, std::simd::f32x2};

pub struct SmoothParameters {
  lfo1_to_lfo1: RampSmooth,
  lfo1_to_lfo2: RampSmooth,
  lfo1_to_out1: RampSmooth,
  lfo1_to_out2: RampSmooth,
  lfo2_to_lfo1: RampSmooth,
  lfo2_to_lfo2: RampSmooth,
  lfo2_to_out1: RampSmooth,
  lfo2_to_out2: RampSmooth,
}

impl SmoothParameters {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      lfo1_to_lfo1: RampSmooth::new(sample_rate, 12.),
      lfo1_to_lfo2: RampSmooth::new(sample_rate, 12.),
      lfo1_to_out1: RampSmooth::new(sample_rate, 12.),
      lfo1_to_out2: RampSmooth::new(sample_rate, 12.),
      lfo2_to_lfo1: RampSmooth::new(sample_rate, 12.),
      lfo2_to_lfo2: RampSmooth::new(sample_rate, 12.),
      lfo2_to_out1: RampSmooth::new(sample_rate, 12.),
      lfo2_to_out2: RampSmooth::new(sample_rate, 12.),
    }
  }

  pub fn initialize(
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
    self.lfo1_to_lfo1.initialize(lfo1_to_lfo1);
    self.lfo1_to_lfo2.initialize(lfo1_to_lfo2);
    self.lfo1_to_out1.initialize(lfo1_to_out1);
    self.lfo1_to_out2.initialize(lfo1_to_out2);
    self.lfo2_to_lfo1.initialize(lfo2_to_lfo1);
    self.lfo2_to_lfo2.initialize(lfo2_to_lfo2);
    self.lfo2_to_out1.initialize(lfo2_to_out1);
    self.lfo2_to_out2.initialize(lfo2_to_out2);
  }

  pub fn process(
    &mut self,
    lfo1_to_lfo1: f32,
    lfo1_to_lfo2: f32,
    lfo1_to_out1: f32,
    lfo1_to_out2: f32,
    lfo2_to_lfo1: f32,
    lfo2_to_lfo2: f32,
    lfo2_to_out1: f32,
    lfo2_to_out2: f32,
  ) -> ([f32x2; 2], [f32x2; 2]) {
    (
      [
        f32x2::from_array([
          self.lfo1_to_lfo1.process(lfo1_to_lfo1),
          self.lfo2_to_lfo1.process(lfo2_to_lfo1),
        ]),
        f32x2::from_array([
          self.lfo1_to_lfo2.process(lfo1_to_lfo2),
          self.lfo2_to_lfo2.process(lfo2_to_lfo2),
        ]),
      ],
      [
        f32x2::from_array([
          self.lfo1_to_out1.process(lfo1_to_out1),
          self.lfo2_to_out1.process(lfo2_to_out1),
        ]),
        f32x2::from_array([
          self.lfo1_to_out2.process(lfo1_to_out2),
          self.lfo2_to_out2.process(lfo2_to_out2),
        ]),
      ],
    )
  }
}
