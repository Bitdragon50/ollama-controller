use std::fmt::{self};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;


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
    total_duration: u64,
    load_duration: u64,
    prompt_eval_count: u64,
    prompt_eval_duration: u64,
    eval_count: u64,
    eval_duration: u64
  
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

#[derive(Serialize,Deserialize)]
struct ChatMessage {
    role: String,
    content: String
}

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

