#[derive(Debug)]

pub struct Speed {
    pub x: f32,
    pub y: f32,
}

pub struct SpaceVector {
	x			: f32,
	y			: f32,
	speed		: Speed,
	acceleration: i32,
    is_static   : bool,
}

impl SpaceVector {
    pub fn new(_x: f32, _y: f32, _is_static: bool) -> SpaceVector {
        SpaceVector {
            x           : _x,
            y           : _y,
            speed       : Speed{x: 0, y: 0},
            acceleration: 0,
            is_static   : _is_static,
        }
    }

    fn set_acceleration(mut& self, n: i32) {
        if !self.is_static {
            self.acceleration = n;
        }
    }

    fn update_speed(mut& self) {
        if !self.is_static {
            self.speed.x *= self.acceleration;
            self.speed.y *= self.acceleration;
        }
    }

    fn update_pos(mut& self) {
        if !self.is_static {
            self.x += self.speed.x;
            self.y += self.speed.y;
        }
    }
    
    fn hit(&mut self, x_speed :f32, y_speed: f32) {
        
        #TODO

        self.update_speed();
    }
}
