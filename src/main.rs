mod tdjson;

use serde::Serialize;
use serde_json::Value;
use std::{env, io};
use tdjson::{new_client, receive, send};

#[derive(Serialize)]
struct TDLibParams {
    use_test_dc: bool,
    database_directory: Option<String>,
    files_directory: Option<String>,
    use_file_database: bool,
    use_chat_info_database: bool,
    use_message_database: bool,
    use_secret_chats: bool,
    api_id: i32,
    api_hash: String,
    system_language_code: String,
    device_model: String,
    system_version: Option<String>,
    application_version: String,
    enable_storage_optimizer: bool,
    ignore_file_names: bool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let client_id = new_client();

    tokio::spawn(async move {
        while let res = receive(10.0) {
            println!("in receive");
            match res {
                Some(x) => println!("{}", x),
                None => println!("nothing here"),
            }
        }
    });

    let root = project_root::get_project_root().unwrap();
    let artefacts_dir = format!("{}/tdlib_artefacts", root.to_str().unwrap());

    // set tdlib params
    let params = TDLibParams {
        use_test_dc: true,
        database_directory: Some(artefacts_dir),
        files_directory: None,
        use_file_database: false,
        use_chat_info_database: false,
        use_message_database: false,
        use_secret_chats: false,
        api_id: env::var("TD_API_ID").unwrap().parse().unwrap(),
        api_hash: env::var("TD_API_HASH").unwrap(),
        system_language_code: "en".to_string(),
        device_model: "MacBook Pro".to_string(),
        system_version: None,
        application_version: "0.1.0".to_string(),
        enable_storage_optimizer: false,
        ignore_file_names: false,
    };

    let params_value = serde_json::to_value(params).unwrap();
    let params_json = match params_value {
        Value::Object(m) => {
            let mut m = m.clone();
            m.insert(
                "@type".to_string(),
                Value::String("setTdlibParameters".to_string()),
            );
            Value::Object(m)
        }
        v => v.clone(),
    }
    .to_string();

    println!("{params_json}");
    send(client_id, &params_json);

    loop {
        let input = input().unwrap();
        send(client_id, &input);
    }
}

fn input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}
