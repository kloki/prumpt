use prumpt::{openai::OpenAI, prompt, Prompter};

fn main() {
    prompt(OpenAI::init())
}
