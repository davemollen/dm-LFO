extern crate lfo_matrix;
extern crate lv2;
use lfo_matrix::{LfoMatrix, LfoShape};
use lv2::prelude::*;

#[derive(PortCollection)]
struct Ports {
  lfo1_on: InputPort<Control>,
  lfo1_freq: InputPort<Control>,
  lfo1_shape: InputPort<Control>,
  lfo1_to_lfo1: InputPort<Control>,
  lfo1_to_lfo2: InputPort<Control>,
  lfo1_to_out1: InputPort<Control>,
  lfo1_to_out2: InputPort<Control>,
  lfo2_on: InputPort<Control>,
  lfo2_freq: InputPort<Control>,
  lfo2_shape: InputPort<Control>,
  lfo2_to_lfo1: InputPort<Control>,
  lfo2_to_lfo2: InputPort<Control>,
  lfo2_to_out1: InputPort<Control>,
  lfo2_to_out2: InputPort<Control>,
  out1: OutputPort<CV>,
  out2: OutputPort<CV>,
}

#[uri("https://github.com/davemollen/dm-LfoMatrix")]
struct DmLfoMatrix {
  lfo_matrix: LfoMatrix,
  is_active: bool,
}

impl DmLfoMatrix {
  fn map_shape(shape: f32) -> LfoShape {
    match shape {
      1. => LfoShape::Sine,
      2. => LfoShape::Triangle,
      3. => LfoShape::SawUp,
      4. => LfoShape::SawDown,
      5. => LfoShape::Square,
      6. => LfoShape::SampleAndHold,
      7. => LfoShape::Random,
      8. => LfoShape::CurvedRandom,
    }
  }

  fn get_parameters(
    &self,
    ports: &mut Ports,
  ) -> (
    bool,
    f32,
    LfoShape,
    f32,
    f32,
    f32,
    f32,
    bool,
    f32,
    LfoShape,
    f32,
    f32,
    f32,
    f32,
  ) {
    (
      *ports.lfo1_on == 1.,
      *ports.lfo1_freq,
      Self::map_shape(*ports.lfo1_shape),
      *ports.lfo1_to_lfo1 * *ports.lfo1_to_lfo1 * *ports.lfo1_to_lfo1,
      *ports.lfo1_to_lfo2 * *ports.lfo1_to_lfo2 * *ports.lfo1_to_lfo2,
      *ports.lfo1_to_out1 * *ports.lfo1_to_out1 * *ports.lfo1_to_out1,
      *ports.lfo1_to_out2 * *ports.lfo1_to_out2 * *ports.lfo1_to_out2,
      *ports.lfo2_on == 1.,
      *ports.lfo2_freq,
      Self::map_shape(*ports.lfo2_shape),
      *ports.lfo2_to_lfo1 * *ports.lfo2_to_lfo1 * *ports.lfo2_to_lfo1,
      *ports.lfo2_to_lfo2 * *ports.lfo2_to_lfo2 * *ports.lfo2_to_lfo2,
      *ports.lfo2_to_out1 * *ports.lfo2_to_out1 * *ports.lfo2_to_out1,
      *ports.lfo2_to_out2 * *ports.lfo2_to_out2 * *ports.lfo2_to_out2,
    )
  }
}

impl Plugin for DmLfoMatrix {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    Some(Self {
      lfo_matrix: LfoMatrix::new(_plugin_info.sample_rate() as f32),
      is_active: false,
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    let (
      lfo1_on,
      lfo1_freq,
      lfo1_shape,
      lfo1_to_lfo1,
      lfo1_to_lfo2,
      lfo1_to_out1,
      lfo1_to_out2,
      lfo2_on,
      lfo2_freq,
      lfo2_shape,
      lfo2_to_lfo1,
      lfo2_to_lfo2,
      lfo2_to_out1,
      lfo2_to_out2,
    ) = self.get_parameters(ports);

    if !self.is_active {
      self.lfo_matrix.initialize_params(
        lfo1_to_lfo1,
        lfo1_to_lfo2,
        lfo1_to_out1,
        lfo1_to_out2,
        lfo2_to_lfo1,
        lfo2_to_lfo2,
        lfo2_to_out1,
        lfo2_to_out2,
      );
      self.is_active = true;
    }

    for (outputs) in ports.out1.iter_mut().zip(ports.out2.iter_mut()) {
      let lfos = self.lfo_matrix.process(
        lfo1_on,
        lfo1_freq,
        lfo1_shape,
        lfo1_to_lfo1,
        lfo1_to_lfo2,
        lfo1_to_out1,
        lfo1_to_out2,
        lfo2_on,
        lfo2_freq,
        lfo2_shape,
        lfo2_to_lfo1,
        lfo2_to_lfo2,
        lfo2_to_out1,
        lfo2_to_out2,
      );
      *outputs.0 = lfos.0;
      *outputs.1 = lfos.1;
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmLfoMatrix);
