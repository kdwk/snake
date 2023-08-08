pub type Frame = Vec<Vec<&'static str>>;

pub fn new_frame() -> Frame {
    let frame = (0..20).map(|_| (0..20).map(|_| " ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    frame
}

pub trait Drawable {
    fn draw(self, frame: &mut Frame);
}