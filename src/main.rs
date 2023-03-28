use adventurers::{game::MyGame};
use std::error::Error;
use std::time::Duration;
use termgame::{run_game, GameSettings, KeyCode, SimpleEvent};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <map_file> <quest_number>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];
    let quest_num = &args[2];

    let mut controller = MyGame::new(file_path, quest_num)?;

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
