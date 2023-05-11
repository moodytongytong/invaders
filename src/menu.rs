use crate::frame::{Drawable, Frame};

pub struct Menu {
    pub options: Vec<String>,
    pub selection: usize,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            options: vec![String::from("New game"), String::from("Exit")],
            selection: 0,
        }
    }

    pub fn change_option(&mut self, upwards: bool) {
        if upwards && self.selection > 0 {
            self.selection -= 1;
        } else if !upwards && self.selection < self.options.len() - 1 {
            self.selection += 1;
        }
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}

// Reuse Frame grid to print the menu options
impl Drawable for Menu {
    fn draw(&self, frame: &mut Frame) {
        frame[0][self.selection] = '>';
        for (index, option) in self.options.iter().enumerate() {
            for i in 0..option.len() {
                frame[i + 1][index] = self.options[index].chars().nth(i).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::frame::new_frame;

    #[test]
    fn change_option_works_correctly() {
        let mut menu = Menu::new();
        menu.change_option(true);
        assert_eq!(menu.selection, 0);
        menu.change_option(false);
        assert_eq!(menu.selection, 1);
        menu.change_option(false);
        assert_eq!(menu.selection, 1);
        menu.change_option(true);
        assert_eq!(menu.selection, 0);
    }

    #[test]
    fn menu_draws_itself_to_frame_correctly() {
        let mut frame = new_frame();
        let mut menu = Menu::new();
        menu.draw(&mut frame);
        assert_eq!(frame[0][0], '>');
        assert_eq!(frame[0][1], ' ');
        let mut frame = new_frame();
        menu.change_option(false);
        menu.draw(&mut frame);
        assert_eq!(frame[0][0], ' ');
        assert_eq!(frame[0][1], '>');
    }
}
