use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

struct GameState {
    user_name: String,
}

impl GameState {
    fn new() -> Self {
        GameState { user_name: "Chigozie".to_string() }
    }
}

fn main() {
    let mut threads: Vec<JoinHandle<()>> = vec![];
    let game_state = Arc::new( GameState::new() );

    let g1 = Arc::clone(&game_state); // first clone
    threads.push(thread::spawn(move || {
        let username = &g1.user_name;
        println!("Username 1: {}", username);
    }));

    let g2 = Arc::clone(&game_state); // second clone
    threads.push(thread::spawn(move || {
        let username = &g2.user_name;
        println!("Username 2: {}", username);
        // ...
    }));

    let g3 = Arc::clone(&game_state); // third clone
    threads.push(thread::spawn(move || {
        let username = &g3.user_name;
        println!("Username 3: {}", username);
        // ...
    }));

    let g4 = Arc::clone(&game_state); // fourth clone
    threads.push(thread::spawn(move || {
        let username = &g4.user_name;
        println!("Username 4: {}", username);
        // ...
    }));

    let g5 = Arc::clone(&game_state); // fifth clone
    threads.push(thread::spawn(move || {
        let username = &g5.user_name;
        println!("Username 5: {}", username);
        // ...
    }));

    for th in threads {
        th.join().unwrap();
    }
}
