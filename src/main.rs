use std::io::Write;

#[derive(serde_derive::Serialize, Clone)]
struct ApiChatMessage {
    role: String,
    content: String,
}

#[derive(serde_derive::Serialize)]
struct ApiChat {
    model: String,
    messages: Vec<ApiChatMessage>,
    stream: bool,
}

#[derive(serde_derive::Deserialize)]
struct ApiTagsModelDetails {
    parent_model: String,
    format: String,
    family: String,
    families: Vec<String>,
    parameter_size: String,
    quantization_level: String,
}

#[derive(serde_derive::Deserialize)]
struct ApiTagsModel {
    name: String,
    model: String,
    modified_at: String,
    size: u64,
    digest: String,
    details: ApiTagsModelDetails,
}

#[derive(serde_derive::Deserialize)]
struct DataApiTagsModel {
    models: Vec<ApiTagsModel>,
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
        let prompt = buf.trim_start().trim_end();
        if prompt.contains("quit") {
            break;
        } else if prompt.contains("get models") || prompt.contains("list models") {
            let msg = "chatbot: HttpRequestError";
            let res = client.get("http://localhost:11434/api/tags").send().expect(&msg);
            if res.status().is_success() {
                let msg = "chatbot: HttpRequestJSONUnWrapError";
                let data: DataApiTagsModel = res.json::<DataApiTagsModel>().expect(&msg);
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
            let msg = "chatbot: HttpPostRequestError";
            let res = client.post("http://localhost:11434/api/chat")
                .json(&ApiChat {
                    model: String::from("llama3.2"),
                    messages: Vec::from(&[ApiChatMessage {
                        role: String::from("user"),
                        content: String::from(prompt),
                    }]),
                    stream: false,
                })
                .send()
                .expect(&msg);
            if res.status().is_success() {
                let msg = "chatbot: HttpPostRequestTextError";
                println!("res: {:?}", res.text().expect(&msg));
            }
        }
    }
}
