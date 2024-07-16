fn main() {
    // Read a json file in the root folder
    let json_str = std::fs::read_to_string("data.json").unwrap();
    // Get the contents of the json file
    let contents: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    if contents["users"].is_null() {
        println!("name is null");
    } else {
        println!("{}", contents["users"][0]["name"]);
    }
}
