use std::{env, io, time::Duration};

use indicatif::ProgressBar;
pub mod openai;

fn read_from_stdin() -> String {
    io::stdin().lines().map(|l| l.unwrap()).collect()
}

pub trait Prompter {
    fn init() -> Self;
    fn generate(&self, prompt: &str) -> String;
    fn edit(&self, prompt: &str) -> String;
}

pub fn prompt(client: impl Prompter) {
    let prompt: String = env::args().collect::<Vec<_>>()[1..].to_vec().join(" ");
    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(100));

    let answer = {
        if prompt.starts_with("--") {
            let snippet = read_from_stdin();
            let prompt = format!("```{}``` {}", snippet, prompt);
            client.edit(&prompt)
        } else {
            client.generate(&prompt)
        }
    };

    bar.finish();
    println!("{}", answer);
}
