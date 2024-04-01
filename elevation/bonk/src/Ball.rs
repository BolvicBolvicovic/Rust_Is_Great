use crates::SpaceVector::SpaceVector;

pub struct Ball {
	    position	: SpaceVector,
	    ray			: u16,
	    has_shield	: bool,
	    grap		: Grap,
    pub score       : u16,
        name        : String,
}

impl Ball {
    pub fn new(_name: String, x: f32, y: f32, _ray: u16) -> Ball {
        Ball {
            position    : SpaceVector::new(x, y, false),
            ray         : _ray,
            has_shield  : false,
            grap        : Grap::new(),
            score       : 0,
            name        : _name;
        }
    }

    fn move(&mut self, hit: bool, other: SpaceVector) {
        if hit {
            self.position.hit(other);
        } else {
            self.position.set_acceleration(other.get_acceleration());
            self.position.update_speed();
        }
        if !self.position.is_static {
            self.position.update_pos();
        }
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
