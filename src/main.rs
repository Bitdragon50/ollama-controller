mod ollama_instance;
use ollama_instance::{Ollama,MyError};
use tokio::runtime::Runtime;
mod vecstore;
use crate::vecstore::*;
// use your custom error MyError and other dependencies

//#[tokio::main]
fn main() -> Result<(), MyError>  {
        //let ollama = Ollama::default();
        // Create a new Tokio runtime
        //let rt = Runtime::new().unwrap();
        //println!("{}",rt.block_on(ollama.chat("Why is the sky blue?")).unwrap());
        //println!("\n\n==New Chat====\n\n{}",rt.block_on(ollama.completion("Why is the sky blue?")).unwrap());
        println!("{:#?}",save_embedding());
        Ok(())
}
