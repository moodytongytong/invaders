use crate::{
    frame::{Drawable, Frame},
    invaders::Invaders,
    shot::Shot,
    {NUM_COLS, NUM_ROWS},
};
use std::time::Duration;

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            shots: Vec::new(),
        }
    }
    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }
    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }
    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 2 {
            self.shots.push(Shot::new(self.x, self.y - 1));
            true
        } else {
            false
        }
    }
    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());
    }
    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> u16 {
        let mut hit_something = 0u16;
        for shot in self.shots.iter_mut() {
            if !shot.exploding {
                let hit_count = invaders.kill_invader_at(shot.x, shot.y);
                if hit_count > 0 {
                    hit_something += hit_count;
                    shot.explode();
                }
            }
        }
        hit_something
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = 'A';
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::invaders::Invader;
    use crate::frame::new_frame;
    use crate::SHOT_PERIOD;

    #[test]
    fn player_created_correctly() {
        let player = Player::new();
        assert_eq!(player.x, NUM_COLS / 2);
        assert_eq!(player.y, NUM_ROWS - 1);
        assert_eq!(player.shots.len(), 0);
    }

    #[test]
    fn player_moves_left_if_not_at_edge() {
        let mut player = Player::new();
        let starting_pos = player.x;
        player.move_left();
        assert_eq!(player.x, starting_pos - 1);
    }

    #[test]
    fn player_does_not_move_left_at_edge() {
        let starting_pos = 0;
        let mut player = Player::new();
        player.x = starting_pos;
        player.move_left();
        assert_eq!(player.x, starting_pos);
    }

    #[test]
    fn player_moves_right_if_not_at_edge() {
        let mut player = Player::new();
        let starting_pos = player.x;
        player.move_right();
        assert_eq!(player.x, starting_pos + 1);
    }

    #[test]
    fn player_does_not_move_right_at_edge() {
        let starting_pos = NUM_COLS;
        let mut player = Player::new();
        player.x = starting_pos;
        player.move_right();
        assert_eq!(player.x, starting_pos);
    }

    #[test]
    fn shot_created_when_player_shoots() {
        let mut player = Player::new();
        assert!(player.shoot());
        assert_eq!(player.shots.len(), 1);
        let shot = player.shots.pop().unwrap();
        assert_eq!(shot.x, player.x);
        assert_eq!(shot.y, player.y - 1);
    }

    #[test]
    fn shot_not_created_when_player_shoots_more_than_two() {
        let mut player = Player::new();
        player.shoot();
        player.shoot();
        assert!(!player.shoot());
        assert_eq!(player.shots.len(), 2);
    }

    #[test]
    fn shot_expires_after_specified_time() {
        let mut player = Player::new();
        player.shoot();
        let shot = player.shots.last_mut().unwrap();
        shot.explode();
        player.update(Duration::new(1, 0));
        assert_eq!(player.shots.len(), 0);
    }

    #[test]
    fn player_hits_invader_directly_above_when_shooting() {
        let mut player = Player::new();
        let mut invaders: Invaders = Invaders::new();
        let x = player.x;
        let y = player.y - 1;
        let invader = Invader::new(x, y);
        invaders.army.push(invader);

        player.shoot();
        player.update(Duration::from_millis(SHOT_PERIOD));
        
        let score = player.detect_hits(&mut invaders);
        assert_eq!(score, 1);
    }

    #[test]
    fn player_does_not_hit_invaders_too_far_away() {
        let mut player = Player::new();
        let mut invaders: Invaders = Invaders::new();
        let score: u16;
        player.shoot();
        player.update(Duration::from_millis(SHOT_PERIOD));
        
        score = player.detect_hits(&mut invaders);
        assert_eq!(score, 0);
    }

    #[test]
    fn player_can_draw_itself_into_a_frame_correctly() {
        let mut frame = new_frame();
        let mut player = Player::new();
        player.shoot();
        player.update(Duration::from_millis(SHOT_PERIOD));
        player.draw(&mut frame);
        assert_eq!(frame[player.x][player.y],'A');
        assert_eq!(frame[player.x][player.y - 1],'|');
    }
}

