extern crate lfo;
extern crate lv2;
use lfo::{Lfo, LfoShape};
use lv2::prelude::*;

#[derive(PortCollection)]
struct Ports {
  freq: InputPort<Control>,
  depth: InputPort<Control>,
  shape: InputPort<Control>,
  offset: InputPort<Control>,
  curve: InputPort<Control>,
  chance: InputPort<Control>,
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
      5. => LfoShape::Rectangle,
      6. => LfoShape::SampleAndHold,
      7. => LfoShape::Random,
      8. => LfoShape::CurvedRandom,
      9. => LfoShape::Noise,
      _ => panic!("Shape is invalid."),
    }
  }

  fn get_parameters(&self, ports: &mut Ports) -> (f32, f32, LfoShape, f32, f32, f32) {
    (
      *ports.freq,
      *ports.depth * 0.001,
      Self::map_shape(*ports.shape),
      *ports.offset * 0.01,
      *ports.chance * 0.01,
      2_f32.powf(*ports.curve * 0.02),
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
    let (freq, depth, shape, offset, curve, chance) = self.get_parameters(ports);

    if !self.is_active {
      self.lfo.initialize_params(freq, depth, curve, chance);
      self.is_active = true;
    }

    for output in ports.output.iter_mut() {
      *output = self.lfo.process(freq, depth, shape, offset, curve, chance);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmLFO);
