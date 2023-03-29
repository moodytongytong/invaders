use crate::frame::{Drawable, Frame};
use rusty_time::timer::Timer;
use std::time::Duration;

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    timer: Timer,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            timer: Timer::from_millis(50),
        }
    }
    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready && !self.exploding {
            if self.y > 0 {
                self.y -= 1;
            }
            self.timer.reset();
        }
    }
    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::from_millis(250);
    }
    pub fn dead(&self) -> bool {
        (self.exploding && self.timer.ready) || (self.y == 0)
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding { '*' } else { '|' };
    }
}

#[cfg(test)]
mod tests {
    use crate::NUM_COLS;

    use super::*;

    #[test]
    fn shot_created_with_correct_attributes() {
        let shot = Shot::new(4, 5);
        assert_eq!(shot.x, 4);
        assert_eq!(shot.y, 5);
        assert_eq!(shot.exploding, false);
        assert_eq!(shot.timer, Timer::from_millis(50))
    }

    fn shot_explodes_if_goes_off_the_y_axis() {
        
    }

}

