use crate::block::Block;
use crate::coordinate::{Coordinate, CoordinateController};
use crate::map::map_from_file;
use crate::player::Player;
use std::collections::HashMap;
use std::{io::Error as IoError, path::Path};
use termgame::{
    Controller, Game, GameColor, GameEvent, GameStyle, KeyCode, SimpleEvent, StyledCharacter,
    ViewportLocation,
};

pub struct MyGame {
    player: Player,
    map: HashMap<(i32, i32), Block>,
}

impl MyGame {
    pub fn new(file_path: impl AsRef<Path>) -> Result<Self, IoError> {
        let map = map_from_file(file_path)?;

        Ok(Self {
            player: Player::default(),
            map,
        })
    }

    pub fn game_move(&mut self, game: &mut Game, keycode: KeyCode) {
        let coordinate_movement = match keycode {
            KeyCode::Up => Coordinate::up_offset(),
            KeyCode::Down => Coordinate::down_offset(),
            KeyCode::Left => Coordinate::left_offset(),
            KeyCode::Right => Coordinate::right_offset(),
            _ => return,
        };

        // Move the player
        self.move_player(game, &coordinate_movement);

        // Check if the player is in the viewport
        if !self.check_if_in_viewport(game) {
            // If not, move the viewport
            let mut viewport_location = game.get_viewport();
            viewport_location.x += coordinate_movement.x;
            viewport_location.y += coordinate_movement.y;
            game.set_viewport(viewport_location);
        }
    }
}

// Private methods
impl MyGame {
    // Initialize the map
    fn init_map(&mut self, game: &mut Game) {
        for ((x, y), block) in &self.map {
            let styled_char = match block {
                Block::Grass => StyledCharacter::new(' ')
                    .style(GameStyle::new().background_color(Some(GameColor::Green))),
                Block::Sand => StyledCharacter::new(' ')
                    .style(GameStyle::new().background_color(Some(GameColor::Yellow))),
                Block::Rocks => StyledCharacter::new(' ')
                    .style(GameStyle::new().background_color(Some(GameColor::Gray))),
                Block::Cinderblock => StyledCharacter::new(' ')
                    .style(GameStyle::new().background_color(Some(GameColor::Red))),
                Block::Flowers => StyledCharacter::new(' ')
                    .style(GameStyle::new().background_color(Some(GameColor::Magenta))),
            };
            game.set_screen_char(*x, *y, Some(styled_char));
        }
    }

    // Move the player according to the coordinate movement
    fn init_player(&mut self, game: &mut Game) {
        self.add_player_to_screen(game);
    }

    fn move_player(&mut self, game: &mut Game, coordinate_movement: &Coordinate) {
        self.remove_player_from_screen(game);
        self.player.move_by(coordinate_movement);
        self.add_player_to_screen(game);
    }

    fn remove_player_from_screen(&self, game: &mut Game) {
        let styled_char = game.get_screen_char(self.player.get_x(), self.player.get_y());
        if let Some(style) = styled_char {
            game.set_screen_char(
                self.player.get_x(),
                self.player.get_y(),
                Some(style.character(' ')),
            );
        } else {
            game.set_screen_char(self.player.get_x(), self.player.get_y(), None);
        }
    }

    fn add_player_to_screen(&self, game: &mut Game) {
        let styled_char = game.get_screen_char(self.player.get_x(), self.player.get_y());
        if let Some(style) = styled_char {
            game.set_screen_char(
                self.player.get_x(),
                self.player.get_y(),
                Some(style.character(self.player.icon)),
            );
        } else {
            game.set_screen_char(
                self.player.get_x(),
                self.player.get_y(),
                Some(StyledCharacter::from(self.player.icon)),
            );
        }
    }

    // Check if the player is in the viewport after moving
    fn check_if_in_viewport(&self, game: &Game) -> bool {
        let ViewportLocation { x: vp_x, y: vp_y } = game.get_viewport();
        let Coordinate {
            x: player_x,
            y: player_y,
        } = self.player.get_position();
        (vp_x..vp_x + 77).contains(&player_x) && (vp_y..vp_y + 21).contains(&player_y)
    }
}

impl Controller for MyGame {
    fn on_start(&mut self, game: &mut Game) {
        // Initialize the map
        self.init_map(game);
        // Initialize the player
        self.init_player(game);
    }

    fn on_event(&mut self, game: &mut Game, event: GameEvent) {
        if let SimpleEvent::Just(key_code) = event.into() {
            self.game_move(game, key_code);
        }
    }

    fn on_tick(&mut self, _game: &mut Game) {}
}
