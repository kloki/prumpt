use std::env;

use reqwest::{blocking::Client, header::HeaderMap};
use serde::{Deserialize, Serialize};

use crate::{Prompter, PrompterError};
const GEN_INSTRUCTIONS: &str = "* Your are a helpfull assistent helping a developer.
* You will be ask to generate new file or snippet in different programming languages.
* You ONLY respond with the actual content of the file.
* Don't wrap your response in markdown syntax.
* Add explaination using comment blocks
";

const EDIT_INSTRUCTIONS: &str = "* Your are a helpfull assistent helping a developer.
* You will be ask to edit some code with some instructions.
* To user will provide to be code in a markdown code block. The instructions after that.
* You ONLY respond with the actual content of the file.
* Don't wrap your response in markdown syntax.
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
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct RespMessage {
    content: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    message: RespMessage,
}

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    choices: Vec<Entry>,
}

pub struct OpenAI {
    client: Client,
    model: String,
}

impl OpenAI {
    pub fn init() -> Result<Self, PrompterError> {
        let secret = env::var("OPENAI_API_KEY")
            .map_err(|_| PrompterError::InitError("OPENAI_API_KEY not set".to_string()))?;
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert(
            "Authorization",
            ["Bearer ", &secret]
                .concat()
                .parse()
                .map_err(|_| PrompterError::InitError("Invalid api key".to_string()))?,
        );
        let client = Client::builder().default_headers(headers).build().unwrap();
        Ok(Self {
            client,
            model: "gpt-3.5-turbo".to_string(),
        })
    }

    fn call(&self, prompt_req: &PromptRequest) -> Result<String, PrompterError> {
        let res = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .json(&prompt_req)
            .send();
        let payload: Payload = res?.json()?;
        Ok(payload.choices[0].message.content.clone())
    }
}

impl Prompter for OpenAI {
    fn edit(&self, prompt: &str) -> Result<String, PrompterError> {
        let prompt_req = PromptRequest::build(EDIT_INSTRUCTIONS, prompt, &self.model);
        self.call(&prompt_req)
    }

    fn generate(&self, prompt: &str) -> Result<String, PrompterError> {
        let prompt_req = PromptRequest::build(GEN_INSTRUCTIONS, prompt, &self.model);
        self.call(&prompt_req)
    }
}
