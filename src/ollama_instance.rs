use std::{collections::HashMap, fmt::{self}, sync::Arc};
use chrono::{DateTime, Utc};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use thiserror::Error;
use tokio::runtime::Runtime;
use tools::Tool;

mod tools;

#[derive(Error, Debug)]
pub enum MyError {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Custom(CustomError)
}

#[derive(Error, Debug)]
pub struct CustomError {
    details: String
}

impl CustomError {
    fn new(msg: &str) -> CustomError {
        CustomError{details: msg.to_string()}
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Reqwest(e) => write!(f, "Request error: {}", e),
            MyError::Serde(e) => write!(f, "Serialization error: {}", e),
            MyError::Custom(e) => write!(f, "Custom error look closer: {}", e),
        }
    }
}

impl From<reqwest::Error> for MyError {
    fn from(err: reqwest::Error) -> MyError {
        MyError::Reqwest(err)
    }
}

impl From<serde_json::Error> for MyError {
    fn from(err: serde_json::Error) -> MyError {
        MyError::Serde(err)
    }
}

#[derive(Debug,Clone)]
pub(crate) struct Ollama {
    host: String,
    model: String
}

impl Ollama {
    pub fn default() -> Ollama {
        Ollama { 
            host: "http://localhost:11434/api".to_owned(),
            model: "llama3.1".to_owned()
        }
    }
    #[warn(dead_code)]
    pub fn new(instance: String, model: String) -> Ollama {
        Ollama { host: format!("{}/api",instance),
                 model
                }
    }
    
    pub async fn completion(&self, prompt: &str) -> Result < String, MyError >{
        let url = format!("{}/generate",self.host);
    
        
        let data = json!({
            "model": self.model,
            "prompt": prompt,
            "stream": false
        });
    
        // Make the POST request
        let response_text = reqwest::Client::new()
                .post(url)
                .json(&data)
                .send()
                .await.map_err(MyError::from).unwrap()
                .text() 
                .await.map_err(MyError::from).unwrap();
        match serde_json::from_str::<CompletionResponse>(response_text.trim()) {
            Ok(response) => Ok(response.response),
            Err(e) => Err(MyError::from(e)), 
        }
    }
    
    pub async fn chat(&self, prompt: &str) -> Result < String, MyError > {
        let url = format!("{}/chat",self.host);
    
        // Create a JSON object with the desired data
        let data = json!({
            "model": self.model,
            "messages": [{
                  "role": "user",
                  "content": prompt
                }],
            "stream": false
        });
    
        // Make the POST request
        let response_text = reqwest::Client::new()
                .post(url)
                .json(&data)
                .send().await
                .map_err(MyError::from).unwrap()
                .text().await 
                .map_err(MyError::from).unwrap();
            
        match serde_json::from_str::<ChatResponse>(response_text.trim()) {
            Ok(chat_response) => Ok(chat_response.message.content),
            Err(e) => Err(MyError::from(e)), // Wrap the MyError in Err
        }
    }

    pub async fn create_embeddings(&self, text: &Vec<&str> ) -> Result< Vec<Vec<f32>> , MyError> {
        let url = format!("{}/embed",self.host);
    
        // Create a JSON object with the desired data
        let data = json!({
            "model": self.model,
            "input": text,
            "stream": false
        });
    
        // Make the POST request
        let response_text = reqwest::Client::new()
                .post(url)
                .json(&data)
                .send()
                .await.map_err(MyError::from).unwrap()
                .text() 
                .await.map_err(MyError::from).unwrap();

        match serde_json::from_str::<EmbeddingsResponse>(response_text.trim()) {
            Ok(json) => Ok(json.embeddings),
            Err(e) => Err(MyError::from(e)),
            }        
    }
}

#[derive(Serialize,Deserialize)]
struct EmbeddingsResponse {
    model: String,
    embeddings: Vec<Vec<f32>>
}

#[derive(Serialize,Deserialize)]
struct ChatResponse {
    model: String,
    created_at: String,
    message: ChatMessage,
    done: bool,
    // total_duration: u64,
    // load_duration: u64,
    // prompt_eval_count: u64,
    // prompt_eval_duration: u64,
    // eval_count: u64,
    // eval_duration: u64
  
}

#[derive(Serialize,Deserialize)]
struct CompletionResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
    context: Vec<u32>,
    total_duration: u64,
    load_duration: u64,
    prompt_eval_count: u64,
    prompt_eval_duration: u64,
    eval_count: u64,
    eval_duration: u64
}

#[derive(Clone, Serialize,Deserialize)]
struct ChatMessage {
    role: String,
    content: String
}

#[derive(Clone, Serialize,Deserialize)]
enum Message {
    Bot(ChatBotMessage),
    User(ChatMessage)
}
pub struct ChatBot {
    mind: Arc<Ollama>,
    messages: Vec<Message>,
    tools: Vec<serde_json::Value>
}

#[derive(Clone, Serialize,Deserialize)]
struct ChatBotResponse {
    model: String,
    created_at: String,
    message: ChatBotMessage  
}



#[derive(Clone, Serialize,Deserialize)]
struct ChatBotMessage {
    role: String,
    content: String,
    tool_calls: Vec<HashMap<String,serde_json::Value>>
}

impl ChatBotMessage {
    fn is_tool_call(&self) -> bool {
        self.tool_calls.last().is_some()
    }
    fn call_a_tool(self) -> Result<(),MyError> {
        todo!()
    }
}

#[derive(Clone, Serialize,Deserialize)]
struct FunctionCall {
    name: String,
    arguments: Vec<String>
}

#[derive(Clone, Serialize,Deserialize)]
pub struct Function {
    name: String,
    description: String,
    parameters: serde_json::Value
}

impl ChatBot {
    fn new(ollama: Ollama, tools: Value) -> ChatBot {
        let empty_chat_message = ChatBotMessage {
                    role : "".into(),
                    content : "".into(),
                    tool_calls: vec![HashMap::new()]
                };
        let boxed_chat_message = Message::Bot(empty_chat_message);
        let messages = vec![boxed_chat_message];
        ChatBot { mind: Arc::new(ollama.clone()), messages , tools: vec![tools] }
    }

    pub async fn chat(&mut self, message: Message) -> Result<ChatBotResponse, MyError > {
        let url = format!("{}/chat",self.mind.host);
        let new_chat_message = ChatMessage {
                    role : "user".into(),
                    content : "message".into()
                };
        let messages = self.messages.push(Message::User(new_chat_message));

    
        // Create a JSON object with the desired data
        let data = json!({
            "model": self.mind.model.clone(),
            "messages": messages,
            "stream": false
        });
    
        // Make the POST request
        let response_text = reqwest::Client::new()
                .post(url)
                .json(&data)
                .send().await
                .map_err(MyError::from).unwrap()
                .text().await 
                .map_err(MyError::from).unwrap();
            
        match serde_json::from_str::<ChatBotResponse>(response_text.trim()) {
            Ok(chat_response) => {
                self.messages.push(Message::Bot(chat_response.message.clone()));
                Ok(chat_response)
            },
            Err(e) => Err(MyError::from(e)), // Wrap the MyError in Err
        }
    }

    fn open_last_message(self) -> Option<Message> {
        let rt = Runtime::new().unwrap();
        let mut last_message = self.messages.last();
        if last_message.is_some() {
            last_message = self.messages.last();
            let message: &Message = last_message.expect("Expected a last_message");
            match message {
                Message::Bot(message_from_bot) => match message_from_bot.is_tool_call() {
                    true => {
                        message_from_bot.call_a_tool();
                        return Some(Message::Bot(message_from_bot.clone()));
                     },
                    false => Some(Message::Bot(message_from_bot.clone()))
                },
                Message::User(message_from_you) => {
                    rt.block_on( self.chat(Message::User(*message_from_you)));
                     Some(Message::User(message_from_you.clone()));
                     rt.block_on(self.chat(Message::User(message_from_you.clone())))
                }
            }

        }
        
todo!()
    }

    fn chat_response_to_fn_call(&mut self) {
        let mut fail = "chat_response_to_fn_call failed at ";
        let mut calls =  self.messages.clone();
        let mut failure = fail.to_string() + "getting last_call";
        let mut last_call = calls.into_iter().last().expect(&failure);
    }
}



fn get_weather(location: &str, time: DateTime<Utc>) {
    let _ = time;
    let input = "get_weather(London, 2024-08-28T14:00:00Z)";
    parse_and_call(input);
}

fn parse_and_call(input: &str) {
    if let Some(start) = input.find('(') {
        if let Some(end) = input.find(')') {
            let func_name = &input[..start];
            let args = &input[start + 1..end];
            let args: Vec<&str> = args.split(',').map(|s| s.trim()).collect();

            match func_name {
                "get_weather" => {
                    if args.len() == 2 {
                        let location = args[0];
                        let time = args[1].parse::<DateTime<Utc>>().unwrap();
                        get_weather(location, time);
                    } else {
                        println!("Invalid number of arguments for get_weather");
                    }
                }
                _ => println!("Unknown function: {}", func_name),
            }
        }
    }
}



/* 
model: (required) the model name
messages: the messages of the chat, this can be used to keep a chat memory
tools: tools for the model to use if supported. Requires stream to be set to false
The message object has the following fields:

role: the role of the message, either system, user, assistant, or tool
content: the content of the message
images (optional): a list of images to include in the message (for multimodal models such as llava)
tool_calls (optional): a list of tools the model wants to use
Advanced parameters (optional):

format: the format to return a response in. Currently the only accepted value is json
options: additional model parameters listed in the documentation for the Modelfile such as temperature
stream: if false the response will be returned as a single response object, rather than a stream of objects
keep_alive: controls how long the model will stay loaded into memory following the request (default: 5m)
Examples
Chat Request (Streaming)
Request
Send a chat message with a streaming response.


curl http://localhost:11434/api/chat -d '{
  "model": "llama3.1",
  "messages": [
    {
      "role": "user",
      "content": "What is the weather today in Paris?"
    }
  ],
  "stream": false,
  "tools": [
    {
      "type": "function",
      "function": {
        "name": "get_current_weather",
        "description": "Get the current weather for a location",
        "parameters": {
          "type": "object",
          "properties": {
            "location": {
              "type": "string",
              "description": "The location to get the weather for, e.g. San Francisco, CA"
            },
            "format": {
              "type": "string",
              "description": "The format to return the weather in, e.g. 'celsius' or 'fahrenheit'",
              "enum": ["celsius", "fahrenheit"]
            }
          },
          "required": ["location", "format"]
        }
      }
    }
  ]
}'

*/
// enum OllamaResponse {
//     EmbeddingsResponse,
//     ChatResponse,
//     CompletionResponse
// }


    // fn list_model(){}
    // fn model_info(){}
    // fn copy_model(){}
    // fn delete_model(){}
    // fn pull_model(){}
    // fn push_model(){}
    // fn list_running_models(){}

