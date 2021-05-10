#[macro_use] extern crate vst;

//use vst2::buffer::AudioBuffer;
use vst::plugin::{HostCallback, Plugin, Info, Category};

#[derive(Default)]
struct Notell;

impl Plugin for Notell {
    fn new(_host: HostCallback) -> Self {
        Notell
    }

    fn get_info(&self) -> Info {
        Info {
            name: "Notell".to_string(),
            vendor: "thelostcreatives".to_string(),
            unique_id: 25032017,
            inputs: 0,

            // We do need two outputs though.  This is default, but let's be
            // explicit anyways.
            outputs: 2,

            // Set our category
            category: Category::Synth,

            ..Info::default()
        }
    }
}

plugin_main!(Notell);
