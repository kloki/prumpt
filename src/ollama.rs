use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::{Prompter, PrompterError};
const GEN_INSTRUCTIONS: &str = "* Your are a helpfull assistent helping a developer.
* You will be ask to generate new file or snippet in different programming languages.
* You ONLY respond with the actual content of the file.
* Don't wrap your response in markdown syntax as in no ``` ... ```.
* Add explaination using comment blocks
";

const EDIT_INSTRUCTIONS: &str = "* Your are a helpfull assistent helping a developer.
* You will be ask to edit some code with some instructions.
* To user will provide to be code in a markdown code block. The instructions after that.
* You ONLY respond with the actual content of the file.
* Don't wrap your response in markdown syntax as in no ``` ... ```.
* Add explaination using comment blocks
";
#[derive(Serialize, Deserialize, Debug)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
struct PromptRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
    stream: bool,
}

impl PromptRequest<'_> {
    fn build<'a>(sytem: &'a str, prompt: &'a str, model: &'a str) -> PromptRequest<'a> {
        PromptRequest {
            model,
            messages: vec![
                Message {
                    role: "system",
                    content: sytem,
                },
                Message {
                    role: "user",
                    content: prompt,
                },
            ],
            stream: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct RespMessage {
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    message: RespMessage,
}

pub struct Ollama {
    client: Client,
    model: String,
}

impl Ollama {
    pub fn init() -> Self {
        let client = Client::builder().build().unwrap();
        Self {
            client,
            model: "dolphin-llama3".to_string(),
        }
    }

    fn call(&self, prompt_req: &PromptRequest) -> Result<String, PrompterError> {
        let res = self
            .client
            .post("http://localhost:11434/api/chat")
            .json(&prompt_req)
            .send();
        let payload: Payload = res?.json()?;
        Ok(payload.message.content.clone())
    }
}

impl Prompter for Ollama {
    fn edit(&self, prompt: &str) -> Result<String, PrompterError> {
        let prompt_req = PromptRequest::build(EDIT_INSTRUCTIONS, prompt, &self.model);
        self.call(&prompt_req)
    }

    fn generate(&self, prompt: &str) -> Result<String, PrompterError> {
        let prompt_req = PromptRequest::build(GEN_INSTRUCTIONS, prompt, &self.model);
        self.call(&prompt_req)
    }
}
