use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }
    fn play(&mut self, ctx: &mut BTerm) {
        // TODO: Fill this stub out later
        self.mode = GameMode::End;
    }
    fn restart(&mut self) {
        // TODO: Fill this stub out later
        self.mode = GameMode::Playing;
    }
    fn main_menu(&mut self) {
        // TODO: Fill this stub out later
        self.mode = GameMode::Menu;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.restart(),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new())
}
