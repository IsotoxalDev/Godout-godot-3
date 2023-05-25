use gdnative::prelude::*;
use webp_animation::prelude::*;

struct FrameData(Vec<u8>, i32);

#[derive(NativeClass)]
#[inherit(Reference)]
#[no_constructor]
pub struct WebpEncoder {
    dimentions: (u32, u32),
    frame_data: Vec<FrameData>,
}

#[methods]
impl WebpEncoder {
    pub fn new(dimentions: (u32, u32)) -> WebpEncoder {
        WebpEncoder {
            dimentions,
            frame_data: vec![],
        }
    }

    #[export]
    fn add_frame(&mut self, _owner: &Reference, image_data: PoolArray<u8>, delay_millis: i32) {
        self.frame_data
            .push(FrameData(image_data.to_vec(), delay_millis));
    }

    #[export]
    fn get_file_data(&mut self, _owner: &Reference) -> PoolArray<u8> {
        let mut encoder = Encoder::new(self.dimentions).unwrap();
        let mut time: i32 = 0;
        for data in &mut self.frame_data {
            time += data.1;
            encoder.add_frame(&data.0[..], time).unwrap();
        }
        let data = encoder.finalize(time).unwrap();
        PoolArray::from_vec(data.to_vec())
    }
}
