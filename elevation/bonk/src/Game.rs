use crate::Ball::*;
use crate::Ground::*;
use crate::SpaceVector::SpaceVector;

pub struct Game {
    balls   : Vec<Ball>,
    grounds : Vec<Ground>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            balls   : Vec::new(),
            grounds : Vec::new(),
        }
    }

    fn add_ground(&mut self, ground: Ground) {
        self.grounds.push(ground);
    }

    fn add_ball(&mut self, ball: Ball) {
        self.balls.push(ball);
    }

    fn delete_ball(&mut self, name: String {
        for ball, i in self.balls.iter() {
            if ball.name == name {
                balls.remove(i);
                break;
            }
        }
    }

    fn next_frame(&mut self) {
        for ball in self.balls.iter() {
           ball.move(false, SpaceVector::new(0, 0, true));
        }
    }
}
