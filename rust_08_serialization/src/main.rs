use serde:: {ser, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct  User {
    username: String,
    password: String,
}

fn main() {
    let u = User {
        username: String:: from("Hey"),
        password: String:: from("223232232")
    };

    let serialized_string = serde_json::to_string(&u);
    // match serialized_string {
    //     Ok(str) => println!("{}", str),
    //     Err(_) => {
    //         println!("Error while converting to string");
    //     }
    // }

    let user_string = serialized_string.unwrap();
    println!("{}", user_string);
}
