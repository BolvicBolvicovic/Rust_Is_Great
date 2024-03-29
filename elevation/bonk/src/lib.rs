const gravity: f32 = 9.81;


pub struct SpaceVector {
	x: i32,
	y: i32,
}

pub struct Ground {
	position	: SpaceVector,
	height		: u16,
	length		: u16,
	is_bouncing	: bool,
}

pub struct Ball {
	position	: SpaceVector,
	ray			: u16,
	has_shield	: bool,
	grap		: Grap,
}

pub struct Grap {
	max_len		: u16,
	len			: u16,
	activated	: bool,
}
