// Here is where we will define a basic rust api to reach out to the chat completion api
// of openai as explained here: https://platform.openai.com/docs/api-reference/chat/create
// From what I see we want to define a basic url, a way to pass the authorization token,
// specify the model to use, a system prompt and the user initial chat. Then we want to be able
// to handle the output. From this I think we only want the content of the response for now at least.
//

mod open_ai {
    use http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        HeaderMap, Request,
    };

    const BASE_URL: &str = "https://api.openai.com/v1/chat";

    struct Api {
        api_key: String,
    }

    impl Api {
        fn respond_to_chat(&self, user_chat: &str) {
            let completion_url_suffix = "/completions";
            let chat_completion_url = format!("{:}/{:}", BASE_URL, completion_url_suffix);

            // We want to use reqwest to make the request here, also let's build the body using a map and reqwest json's feature
            let body = format!(
                r#"
                {{ "model": "gpt-4",
                "messages": [
                    {{
                        "role": "system",
                        "content": "You are a helpful assistant."
                    }},
                    {{
                        "role": "user",
                        "content": "{:}"
                    }}
                ]
                }}
                "#,
                user_chat
            );
            let client = reqwest::blocking::Client::new();
            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
            headers.insert(
                AUTHORIZATION,
                format!("Bearer {:}", self.api_key).parse().unwrap(),
            );
            let response = client
                .post(chat_completion_url)
                .body(body)
                .headers(headers)
                .send()
                .expect("Couldn't receive a response from openai");

            print!("Request value is {:?}", response.text()); // Why can't I use the question mark?
        }
    }
}
