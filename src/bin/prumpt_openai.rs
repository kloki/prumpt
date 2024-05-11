use std::time::Duration;

use indicatif::ProgressBar;
use prumpt::{openai::OpenAI, prompt};

fn main() {
    let bar = ProgressBar::new_spinner();

    bar.enable_steady_tick(Duration::from_millis(100));
    let answer = prompt(OpenAI::init().unwrap()).unwrap();
    bar.finish();
    println!("{}", answer);
}
