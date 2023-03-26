use crate::coordinate::{Coordinate, CoordinateController};
use crate::player::Player;
use termgame::{
    Controller, Game, GameEvent, KeyCode, SimpleEvent, StyledCharacter, ViewportLocation,
};

pub struct MyGame {
    player: Player,
}

impl Default for MyGame {
    fn default() -> Self {
        Self::new()
    }
}

impl MyGame {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
        }
    }

    pub fn game_move(&mut self, game: &mut Game, keycode: KeyCode) {
        let coordinate_movement = match keycode {
            KeyCode::Up => Coordinate::move_up(),
            KeyCode::Down => Coordinate::move_down(),
            KeyCode::Left => Coordinate::move_left(),
            KeyCode::Right => Coordinate::move_right(),
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
     // Move the player according to the coordinate movement
     fn move_player(&mut self, game: &mut Game, coordinate_movement: &Coordinate) {
        // 1. Remove the player from the current position
        let current_styled_char = game.get_screen_char(self.player.get_x(), self.player.get_y());
        if let Some(styled_char) = current_styled_char {
            game.set_screen_char(
                self.player.get_x(),
                self.player.get_y(),
                Some(styled_char.character(' ')),
            );
        }

        // 2. Move the player
        self.player.move_by(coordinate_movement);

        // 3. Add the player to the new position
        game.set_screen_char(
            self.player.get_x(),
            self.player.get_y(),
            Some(StyledCharacter::new(self.player.icon)),
        );
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
        self.move_player(game, &Coordinate::stay_still());
    }

    fn on_event(&mut self, game: &mut Game, event: GameEvent) {
        if let SimpleEvent::Just(key_code) = event.into() {
            self.game_move(game, key_code);
        }
    }

    fn on_tick(&mut self, _game: &mut Game) {}
}
