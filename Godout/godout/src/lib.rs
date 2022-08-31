// Rust GDNative implementation of this Godot tutorial:
// https://docs.godotengine.org/en/stable/tutorials/plugins/editor/making_plugins.html#a-custom-node

use gdnative::api::{EditorPlugin, Directory};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(EditorPlugin)]
struct Godout;

#[methods]
impl Godout {
    fn new(_owner: TRef<EditorPlugin>) -> Self {
        Godout
    }

    #[method]
    fn _enter_tree(&self, #[base] owner: TRef<EditorPlugin>) {
        let dir = Directory::new();
        dir.open("res://addons/godout").unwrap();
        dir.list_dir_begin(true, true).unwrap();
        let file_name = dir.get_next();
        while file_name != GodotString::new() {
            if dir.current_is_dir() {
                let curr = dir.get_current_dir();
                let sub_dir = Directory::new();
                sub_dir.open(format!("{}{}", curr, file_name)).unwrap();
                sub_dir.list_dir_begin(true, true).unwrap();
                let out_name = sub_dir.get_next();
            }
        }
    }

    #[method]
    fn _exit_tree(&self, #[base] owner: TRef<EditorPlugin>) {
    }
}

// unsafe fn load<T>(path: &str, hint: &str) -> Option<Ref<T, Shared>>
// where
//     T: GodotObject<Memory = RefCounted> + SubClass<Resource>,
// {
//     let resource = ResourceLoader::godot_singleton().load(path, hint, false)?;
//     let resource = resource.assume_safe().claim();
//     resource.cast::<T>()
// }

fn init(handle: InitHandle) {
    handle.add_tool_class::<Godout>();
}

godot_init!(init);
