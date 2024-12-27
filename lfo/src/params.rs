mod smooth;
use smooth::LinearSmooth;
pub use smooth::Smoother;

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

pub struct Params {
  pub freq: LinearSmooth,
  pub depth: LinearSmooth,
  pub shape: LfoShape,
  pub offset: f32,
  pub chance: f32,
  is_initialized: bool,
}

impl Params {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      freq: LinearSmooth::new(sample_rate, 12.),
      depth: LinearSmooth::new(sample_rate, 12.),
      shape: LfoShape::Sine,
      offset: 0.,
      chance: 1.,
      is_initialized: false,
    }
  }

  pub fn set(&mut self, freq: f32, shape: f32, chance: f32, depth: f32, offset: f32) {
    self.shape = Self::map_shape(shape);
    self.chance = chance;
    self.offset = offset;

    if self.is_initialized {
      self.freq.set_target(freq);
      self.depth.set_target(depth);
    } else {
      self.freq.reset(freq);
      self.depth.reset(depth);
      self.is_initialized = true;
    }
  }

  fn map_shape(shape: f32) -> LfoShape {
    match shape {
      1. => LfoShape::Sine,
      2. => LfoShape::Triangle,
      3. => LfoShape::SawUp,
      4. => LfoShape::SawDown,
      5. => LfoShape::Rectangle,
      6. => LfoShape::SampleAndHold,
      7. => LfoShape::Random,
      8. => LfoShape::CurvedRandom,
      9. => LfoShape::Noise,
      _ => panic!("Shape is invalid."),
    }
  }
}
