use crate::{
    frame::{self, new_frame, Drawable, Frame},
    invaders::Invaders,
    level::Level,
    menu::Menu,
    player::Player,
    render,
    score::Score,
};

use rusty_audio::Audio;

use std::{
    error::Error,
    sync::mpsc::{self, Receiver},
    time::{Duration, Instant},
    {io, thread},
};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

pub struct Game {
    player: Player,
    invaders: Invaders,
    score: Score,
    level: Level,
    menu: Menu,
    curr_frame: Frame,
    audio: Audio,
}

impl Game {
    pub fn new() -> Self {
        let mut audio = Audio::new();
        for item in &["explode", "lose", "move", "pew", "startup", "win"] {
            audio.add(item, &format!("audio/original/{}.wav", item));
        }
        Self {
            audio,
            player: Player::new(),
            invaders: Invaders::new(),
            score: Score::new(),
            level: Level::new(),
            menu: Menu::new(),
            curr_frame: new_frame(),
        }
    }

    pub fn play(&mut self) -> Result<(), Box<dyn Error>> {
        self.audio.play("startup");

        // Terminal
        let mut stdout = io::stdout();
        terminal::enable_raw_mode()?;
        stdout.execute(EnterAlternateScreen)?;
        stdout.execute(Hide)?;

        // Render loop in a separate thread
        let (render_tx, render_rx) = mpsc::channel();
        let render_handle = thread::spawn(move || {
            render_screen(render_rx);
        });

        let mut in_menu = true;
        let mut instant = Instant::now();

        'gameloop: loop {
            // Per-frame init
            
            let elapsed_time = instant.elapsed();
            instant = Instant::now();
            self.curr_frame = new_frame();
        
            if in_menu {
                // Input handlers for the menu
                while event::poll(Duration::default())? {   // what does event poll do?   ASK
                    if let false = self.handle_menu(&mut in_menu) { break 'gameloop; }
                }
                self.menu.draw(&mut self.curr_frame);
                let _ = render_tx.send(self.curr_frame);
                thread::sleep(Duration::from_millis(1));
                continue;
            }
        
            // Input handlers for the game
            while event::poll(Duration::default())? {
                self.handle_playing(&mut in_menu);
            }
        
            // Updates
            self.update_actors(elapsed_time);
            
            // Draw & render
            self.draw_actors_to_frame();
            let _ = render_tx.send(self.curr_frame);
            thread::sleep(Duration::from_millis(1));
        
            // Win or lose?
            let max_level_reached = self.handle_results(&mut in_menu);
            if max_level_reached { break 'gameloop; }
        }
        // Cleanup
        drop(render_tx);
        render_handle.join().unwrap();
        self.audio.wait();
        stdout.execute(Show)?;
        stdout.execute(LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    fn handle_menu(&mut self, in_menu: &mut bool) -> bool {
        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                KeyCode::Up => self.menu.change_option(true),
                KeyCode::Down => self.menu.change_option(false),
                KeyCode::Char(' ') | KeyCode::Enter => {
                    *in_menu = false;
                    if self.menu.selection == 1 { return false; }
                }
                _ => {}
            }
        }
        true
    }
    
    fn handle_playing(&mut self, in_menu: &mut bool) {
        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                KeyCode::Left => self.player.move_left(),
                KeyCode::Right => self.player.move_right(),
                KeyCode::Char(' ') | KeyCode::Enter => {
                    if self.player.shoot() {
                        self.audio.play("pew");
                    }
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.audio.play("lose");
                    reset_game(in_menu, &mut self.player, &mut self.invaders);
                }
                _ => {}
            }
        }
    }
    
    fn update_actors(&mut self, elapsed_time: Duration) {
        self.player.update(elapsed_time);
        if self.invaders.update(elapsed_time) {
            self.audio.play("move");
        }
        let hits: u16 = self.player.detect_hits(&mut self.invaders);
        if hits > 0 {
            self.audio.play("explode");
            self.score.add_points(hits);
        }
    }
    
    fn handle_results(&mut self, in_menu: &mut bool) -> bool {
        if self.invaders.all_killed() {
            if self.level.increment_level() {
                self.audio.play("win");
                return true
            } 
            self.invaders = Invaders::new();
        } else if self.invaders.reached_bottom() {
            self.audio.play("lose");
            reset_game(in_menu, &mut self.player, &mut self.invaders);
        }
        false
    }
    
    fn draw_actors_to_frame(&mut self) {
        let drawables: Vec<&dyn Drawable> = vec![&mut self.player, &mut self.invaders, &mut self.score, &mut self.level];
        for drawable in drawables {
            drawable.draw(&mut self.curr_frame);
        }
    }
}


fn render_screen(render_rx: Receiver<Frame>) {
    let mut last_frame = frame::new_frame();
    let mut stdout = io::stdout();
    render::render(&mut stdout, &last_frame, &last_frame, true);
    while let Ok(curr_frame) = render_rx.recv() {
        render::render(&mut stdout, &last_frame, &curr_frame, false);
        last_frame = curr_frame;
    }
}

fn reset_game(in_menu: &mut bool, player: &mut Player, invaders: &mut Invaders) {
    *in_menu = true;
    *player = Player::new();
    *invaders = Invaders::new();
}