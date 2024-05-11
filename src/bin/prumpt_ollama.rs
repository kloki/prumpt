use std::time::Duration;

use indicatif::ProgressBar;
use prumpt::{ollama::Ollama, prompt};

fn main() {
    let bar = ProgressBar::new_spinner();

    bar.enable_steady_tick(Duration::from_millis(100));
    let answer = prompt(Ollama::init()).unwrap();
    bar.finish();
    println!("{}", answer);
}
