use std::io::Write;

#[derive(serde_derive::Deserialize)]
struct ModelDetails {
    parent_model: String,
    format: String,
    family: String,
    families: Vec<String>,
    parameter_size: String,
    quantization_level: String,
}

#[derive(serde_derive::Deserialize)]
struct Model {
    name: String,
    model: String,
    modified_at: String,
    size: u64,
    digest: String,
    details: ModelDetails,
}

#[derive(serde_derive::Deserialize)]
struct DataModel {
    models: Vec<Model>,
}

fn main() {
    let client = reqwest::blocking::Client::new();
    let mut buf = String::new();
    let stdin = std::io::stdin();
    loop {
        print!("prompt:");
        std::io::stdout().flush().expect("chatbot: IOERR");
        buf.clear();
        stdin.read_line(&mut buf).expect("chatbot: IOERR");
        let prompt = buf.trim_start();
        if prompt.contains("quit") {
            break;
        } else if prompt.contains("get models") || prompt.contains("list models") {
            let msg = "chatbot: HttpRequestError";
            let res = client.get("http://localhost:11434/api/tags").send().expect(&msg);
            if res.status().is_success() {
                let msg = "chatbot: HttpRequestJSONUnWrapError";
                let data: DataModel = res.json::<DataModel>().expect(&msg);
                println!("chatbot: available models:\n\n");
                for model in data.models {
                    println!("name: {:?}", model.name);
                    println!("model: {:?}", model.model);
                    println!("modified_at: {:?}", model.modified_at);
                    println!("size: {:?}", model.size);
                    println!("digest: {:?}", model.digest);
                    println!("parent_model: {:?}", model.details.parent_model);
                    println!("format: {:?}", model.details.format);
                    println!("family: {:?}", model.details.family);
                    println!("families: {:#?}", model.details.families);
                    println!("parameter_size: {:?}", model.details.parameter_size);
                    println!("quantization_lvl: {:?}", model.details.quantization_level);
                    println!("\n");
                }
                println!("chatbot: done");
            }
        } else {
            // TODO: forward prompt to the LLM
        }
    }
}
