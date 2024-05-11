use std::{env, io};

pub mod ollama;
pub mod openai;

fn read_from_stdin() -> String {
    io::stdin().lines().map(|l| l.unwrap()).collect()
}

#[derive(Debug)]
pub enum PrompterError {
    InitError(String),
    RequestError(reqwest::Error),
}

impl From<reqwest::Error> for PrompterError {
    fn from(err: reqwest::Error) -> Self {
        PrompterError::RequestError(err)
    }
}

pub trait Prompter {
    fn generate(&self, prompt: &str) -> Result<String, PrompterError>;
    fn edit(&self, prompt: &str) -> Result<String, PrompterError>;
}

pub fn prompt(client: impl Prompter) -> Result<String, PrompterError> {
    let prompt: String = env::args().collect::<Vec<_>>()[1..].to_vec().join(" ");
    let answer = {
        if prompt.starts_with("--") {
            let snippet = read_from_stdin();
            let prompt = format!("```{}``` {}", snippet, prompt);
            client.edit(&prompt)?
        } else {
            client.generate(&prompt)?
        }
    };

    Ok(answer)
}
