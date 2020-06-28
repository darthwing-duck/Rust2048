mod game;
use std::io;

fn main() {
    let mut m = game::Game::start();
    let mut run = true;
    
    while run {
        println!("score: {} seed: {}", m.score(), m.seed());
        m.dump_board();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        let op = input.chars().nth(0).unwrap();

        if op == 'q' {
            run = false;
        } else if op == 'u' {
            m.up();
        } else if op == 'd' {
            m.down();
        } else if op == 'r' {
            m.right();
        } else if op == 'l' {
            m.left();
        }
    }
}
