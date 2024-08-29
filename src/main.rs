mod ollama_instance;
use ollama_instance::{Ollama,MyError};
use tokio::runtime::Runtime;
use crate::vecstore::*;
mod v2_confluence;
mod vecstore;
mod html_to_base64;
use crate::html_to_base64::html_to_base64;
mod tools;
// use your custom error MyError and other dependencies

#[tokio::main]
async fn main() -> Result<(), MyError>  {
        let url = "www.google.com";
        let image = html_to_base64(url).await.expect("main failed at image");
        println!("The image: {}", &image);
        Ok(())
}

        // let ollama = Ollama::default();//Ollama::new( "http://localhost:11434".into(),"bge-large".into());//
        // // Create a new Tokio runtime
        // let rt = Runtime::new().unwrap();
        // //println!("{}",rt.block_on(ollama.chat("Why is the sky blue?")).unwrap());
        // //println!("\n\n==New Chat====\n\n{:#?}",rt.block_on(ollama.create_embeddings(&vec!["Why is the sky blue?"])).unwrap());
        // let text_vec = vec!["Why is the sky blue?", "Why is the sea blue?", "Why do cats meow?", "Why do dogs bark?", "What does the fox say?", "boy", "girl","big wild dog","small dog"];
        // let embeddings = rt.block_on(ollama.create_embeddings(&text_vec))?;
        // // dbg!(embeddings);
        // let text_vec_string = text_vec.into_iter().map(|s|s.to_owned()).collect::<Vec<String>>();
        // let dimensions = embeddings[0].len() as u64;
        // dbg!(dimensions);
        // let store_name = "test_collection";
        // println!("{:#?}",rt.block_on(save_embedding(embeddings, text_vec_string, store_name, dimensions)));

        // let look_for = rt.block_on(ollama.create_embeddings(&vec!["dangerous"])).unwrap();
        // let search_wolf = rt.block_on(find_sim(look_for[0].clone(), store_name));
        
        // Ok(())