use adventurers::{game::MyGame, player::Player};
use std::error::Error;
use std::time::Duration;
use termgame::{run_game, GameSettings, KeyCode, SimpleEvent};

fn main() -> Result<(), Box<dyn Error>> {
    let mut controller = MyGame::default();

    run_game(
        &mut controller,
        GameSettings::new()
            // The below are the defaults, but shown so you can edit them.
            .tick_duration(Duration::from_millis(50))
            .quit_event(Some(SimpleEvent::WithControl(KeyCode::Char('c')).into())),
    )?;

    println!("Game Ended!");

    Ok(())
}
