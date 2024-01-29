mod tdjson;

use tdjson::{execute, new_client};

fn main() {
    let id = new_client();
    println!("Client ID: {}", id);

    // test based on official example:
    // https://github.com/tdlib/td/blob/master/example/python/tdjson_example.py
    let test_str = execute("{\"@type\": \"getTextEntities\", \"text\": \"@telegram /test_command https://telegram.org telegram.me\", \"@extra\": [\"5\", 7.0, \"a\"]}").unwrap();
    println!("From telegram: {}", test_str);
}
