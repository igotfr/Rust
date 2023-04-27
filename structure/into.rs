#[derive(Debug, Copy, Clone)]
pub struct Padding {
    pub top: f32
}

impl From<u16> for Padding {
    fn from(p: u16) -> Self {
        Padding {
            top: f32::from(p)
        }
    }
}

fn padding(p: impl Into<Padding> + std::fmt::Debug) {
    println!("{p:?}")
}

fn main() {
    padding(Padding::from(4));
    padding(4);
}
