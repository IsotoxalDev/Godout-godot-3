use gdnative::prelude::*;
use gif::{DisposalMethod, Encoder, Frame, Repeat};

struct FrameData(Vec<u8>, u16);

#[derive(NativeClass)]
#[inherit(Reference)]
#[no_constructor]
pub struct GifEncoder {
    width: u16,
    height: u16,
    frame_data: Vec<FrameData>,
}

#[methods]
impl GifEncoder {
    pub fn new(width: u16, height: u16) -> GifEncoder {
        GifEncoder {
            width,
            height,
            frame_data: vec![],
        }
    }

    #[export]
    fn add_frame(&mut self, _owner: &Reference, image_data: PoolArray<u8>, delay_millis: u16) {
        self.frame_data
            .push(FrameData(image_data.to_vec(), delay_millis));
    }

    #[export]
    fn get_file_data(&mut self, _owner: &Reference) -> PoolArray<u8> {
        let mut buffer = Vec::new();
        let mut encoder = Encoder::new(&mut buffer, self.width, self.height, &[])
            .expect("Couldn't create encoder");
        encoder.set_repeat(Repeat::Infinite).unwrap();
        for data in &mut self.frame_data {
            let mut frame = Frame::from_rgba(self.width, self.height, &mut data.0);
            frame.delay = data.1 / 10;
            frame.dispose = DisposalMethod::Background;
            encoder
                .write_frame(&frame)
                .expect("Error while writing frame");
        }
        drop(encoder);
        PoolArray::from_vec(buffer)
    }
}
