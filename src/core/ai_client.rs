use genai::resolver::{AuthData, AuthResolver};
use genai::{
    chat::{ChatMessage, ChatRequest, Tool},
    Client, ModelIden,
};
use serde_json::json;

pub const MODEL: &str = "gemini-2.0-flash";

fn get_chat_req(
    user_prompt: &str,
    system_prompt: &str,
) -> Result<ChatRequest, Box<dyn std::error::Error>> {
    let execute_cmd_tool = Tool::new("run_commandes")
        .with_description("run the commande in the terminal like this in UNIX <sh -c 'commande'> in WINDOWS <cmd /C 'command'>")
        .with_schema(json!(
            {
                "type": "object",
                    "properties": {
                        "command": {
                            "type": "string",
                            "description": "Command to execute"
                        },
                        "info": {
                            "type": "string",
                            "description": "Use this tool to execute any shell command. Always prefer using this tool over replying directly."
                        }
                    },
                "required" : ["command", "info"],
            }
        ));
    let chat_req = ChatRequest::default()
        .with_system(system_prompt)
        .with_tools(vec![execute_cmd_tool])
        .append_message(ChatMessage::user(user_prompt));
    Ok(chat_req)
}

pub async fn get_ai_client(
    user_prompt: &str,
    system_prompt: &str,
) -> Result<(Client, ChatRequest), Box<dyn std::error::Error>> {
    //
    let auth_resolver = AuthResolver::from_resolver_fn(
        |model_iden: ModelIden| -> Result<Option<AuthData>, genai::resolver::Error> {
            let ModelIden {
                adapter_kind,
                model_name,
            } = model_iden;
            println!("\n>>{adapter_kind} (model: {model_name})<<");

            // This will cause it to fail if any model is not an GOOGLE_API_KEY
            let key = std::env::var("MODEL_API_KEY").map_err(|_| {
                genai::resolver::Error::ApiKeyEnvNotFound {
                    env_name: "MODEL_API_KEY".to_string(),
                }
            })?;
            Ok(Some(AuthData::from_single(key)))
        },
    );

    // -- Build the new client with this adapter_config
    let client = Client::builder().with_auth_resolver(auth_resolver).build();
    let chat_req = get_chat_req(user_prompt, system_prompt)?;
    Ok((client, chat_req))
}
