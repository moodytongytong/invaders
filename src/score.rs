use crate::frame::{Drawable, Frame};

#[derive(Default)]
pub struct Score {
    count: u16,
}

impl Score {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn add_points(&mut self, amount: u16) {
        self.count += amount;
    }
}

impl Drawable for Score {
    fn draw(&self, frame: &mut Frame) {
        // format our score string
        let formatted = format!("SCORE: {:0>4}", self.count);

        // iterate over all characters
        for (i, c) in formatted.chars().enumerate() {
            // put them in the first row
            frame[i][0] = c;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::frame::{new_frame, self};

    #[test]
    fn score_correctly_drawn_to_the_frame() {
        let mut frame = new_frame();
        let mut score = Score::new();
        score.draw(&mut frame);
        assert_eq!(frame[10][0], '0');

        let mut frame = new_frame();
        score.add_points(5);
        score.draw(&mut frame);
        assert_eq!(frame[10][0], '5');
    }
}
