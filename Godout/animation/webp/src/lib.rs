mod encoder;

use gdnative::prelude::*;
use encoder::WebpEncoder;

#[derive(NativeClass)]
#[user_data(user_data::RwLockData<Webp>)]
#[inherit(Node)]
pub struct Webp;

#[methods]
impl Webp {
    fn new(_owner: &Node) -> Self {
        Webp
    }

    #[export]
    fn get_encoder(&self, _owner: &Node, width: u32, height: u32) -> Instance<WebpEncoder, Unique> {
        WebpEncoder::new((width, height)).emplace()
    }
}

// Init Stuff

fn init(handle: InitHandle) {
    handle.add_class::<Webp>();
    handle.add_class::<WebpEncoder>();
}


godot_init!(init);
