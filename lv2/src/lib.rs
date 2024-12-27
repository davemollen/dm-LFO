extern crate lfo;
extern crate lv2;
use lfo::{Lfo, Params};
use lv2::prelude::*;

#[derive(PortCollection)]
struct Ports {
  freq: InputPort<Control>,
  depth: InputPort<Control>,
  shape: InputPort<Control>,
  offset: InputPort<Control>,
  chance: InputPort<Control>,
  output: OutputPort<CV>,
}

#[uri("https://github.com/davemollen/dm-LFO")]
struct DmLFO {
  lfo: Lfo,
  params: Params,
}

impl Plugin for DmLFO {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    let sample_rate = plugin_info.sample_rate() as f32;

    Some(Self {
      lfo: Lfo::new(sample_rate),
      params: Params::new(sample_rate),
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    self.params.set(
      *ports.freq,
      *ports.shape,
      *ports.chance * 0.01,
      *ports.depth * 0.01,
      *ports.offset * 0.01,
    );

    for output in ports.output.iter_mut() {
      *output = self.lfo.process(&mut self.params);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmLFO);
