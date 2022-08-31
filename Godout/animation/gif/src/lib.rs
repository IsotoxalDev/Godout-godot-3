mod encoder;

use gdnative::prelude::*;
use encoder::GifEncoder;

#[derive(NativeClass)]
#[user_data(user_data::RwLockData<Gif>)]
#[inherit(Node)]
pub struct Gif;

#[methods]
impl Gif {
    fn new(_owner: &Node) -> Self {
        Gif
    }

    #[export]
    fn get_encoder(&self, _owner: &Node, width: u16, height: u16) -> Instance<GifEncoder, Unique> {
        GifEncoder::new(width, height).emplace()
    }
}

// Init Stuff

fn init(handle: InitHandle) {
    handle.add_class::<Gif>();
    handle.add_class::<GifEncoder>();
}


godot_init!(init);