use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use async_std::task::block_on;


fn main() {
    loop{
        println!("Please enter a prompt");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
       
        let input = input.trim();
        match input {
            "exit" => break,
            _ => {
                block_on(generate(input.to_string()));
            }
        }
    }
}

async fn generate(prompt: String) {
    let ollama = Ollama::default();
    let model = "llama2".to_string();
    let res = ollama.generate(GenerationRequest::new(model, prompt)).await;
    
    if let Ok(res) = res {
        println!("{}", res.response);
    }
    else {
        println!("Error: {:?}", res.err().unwrap());
    }
}
