use crates::SpaceVector::SpaceVector;

pub struct Ball {
	position	: SpaceVector,
	ray			: u16,
	has_shield	: bool,
	grap		: Grap,
}

impl Ball {
    pub fn new(x: f32, y: f32, _ray: u16) -> Ball {
        Ball {
            position    : SpaceVector::new(x, y, false),
            ray         : _ray,
            has_shield  : false,
            grap        : Grap::new(),
        }
    }

    fn move(&mut self, hit: bool, x_speed: f32, y_speed: f32, acceleration: u16) {
        if hit {
            self.position.hit(x_speed, y_speed, acceleration);
        } else {
            self.position.set_acceleration(acceleration);
            self.position.update_speed();
        }
        self.position.update_pos();
    }
}

pub struct Grap {
	    max_len		: u16,
	    len			: f16,
	pub activated	: bool,
}

impl Grap {
    pub fn new() -> Grap {
        Grap {
            max_len     : 5,
            len         : 0,
            activated   : false,
        }
    }

    fn set_len(&self, n: u16) {
        if n <= self.max_len {
            self.len = n;
        } else {
            self.len = max_len;
        }
    }

    pub fn get_max_len(&self) -> const u16 {
        const _max_len = self.max_len;
        _max_len
    }
}
