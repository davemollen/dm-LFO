extern crate lfo;
extern crate lv2;
use lfo::{InputMode, Lfo, LfoShape};
use lv2::prelude::*;

#[derive(PortCollection)]
struct Ports {
  input: InputPort<CV>,
  input_mode: InputPort<Control>,
  freq: InputPort<Control>,
  depth: InputPort<Control>,
  shape: InputPort<Control>,
  output: OutputPort<CV>,
}

#[uri("https://github.com/davemollen/dm-LFO")]
struct DmLFO {
  lfo: Lfo,
  is_active: bool,
}

impl DmLFO {
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
      9. => LfoShape::Noise,
      _ => panic!("Shape is invalid."),
    }
  }

  fn map_input_mode(input_mode: f32) -> InputMode {
    match input_mode {
      1. => InputMode::Add,
      2. => InputMode::SubtractA,
      3. => InputMode::SubtractB,
      4. => InputMode::Multiply,
      5. => InputMode::FM,
      6. => InputMode::PM,
      _ => panic!("Input mode is invalid."),
    }
  }

  fn get_parameters(&self, ports: &mut Ports) -> (InputMode, f32, f32, LfoShape) {
    (
      Self::map_input_mode(*ports.input_mode),
      *ports.freq,
      *ports.depth * 0.1,
      Self::map_shape(*ports.shape),
    )
  }
}

impl Plugin for DmLFO {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    Some(Self {
      lfo: Lfo::new(_plugin_info.sample_rate() as f32),
      is_active: false,
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    let (input_mode, freq, depth, shape) = self.get_parameters(ports);

    if !self.is_active {
      self.lfo.initialize_params(freq, depth);
      self.is_active = true;
    }

    for (input, output) in ports.input.iter_mut().zip(ports.output.iter_mut()) {
      *output = self
        .lfo
        .process(*input * 0.1, input_mode, freq, depth, shape);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmLFO);
