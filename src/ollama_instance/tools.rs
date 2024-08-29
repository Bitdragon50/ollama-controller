use super::Ollama;
use serde::{Deserialize, Serialize};
use serde_json::*;


#[derive(Clone, Serialize,Deserialize)]
pub enum ToolCall {
    FunctionCall(Function),
    //HelperBot(Ollama)
}

impl ToolCall {
    fn call(self) {
        match self {
            Self::FunctionCall(_)   => (Function)        
        }
    }
}



#[derive(Clone, Serialize,Deserialize)]
struct HelperBot {
    bot: Ollama
}