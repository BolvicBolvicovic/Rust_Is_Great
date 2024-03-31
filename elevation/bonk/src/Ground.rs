use crates::SpaceVector::SpaceVector;

pub struct Ground {
	position	: SpaceVector,
	height		: u16,
	length		: u16,
	is_bouncing	: bool,
}

impl Ground {
    pub fn new(x: f32, y: f32, _height: u16, _length: u16, bounce: bool) -> Ground {
        Ground {
            position    : SpaceVector::new(x, y, true),
            height      : _height,
            length      : _length,
            is_bouncing : bounce,
        }
    }
}
