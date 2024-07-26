use std::collections::HashMap;
use firebase_rs::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
}

#[tokio::main]
async fn main() {
    let user = User {
        name: "Nitin Dahiya".to_string(),
        age: 30,
        email: "nitindahiya00000@gmail".to_string(),
    };

    let firebase = Firebase::new("ENTER YOUR FIREBASE LINK HERE").unwrap();
    let response = set_user(&firebase, &user).await;

    match response {
        Ok(res) => {
            let mut user = get_user(&firebase, &res.name).await.unwrap();
            println!("{:?}", user);

            let users = get_users(&firebase).await.unwrap();
            println!("{:?}", users);

            user.email = "updated.mail@gmail.com".to_string();
            let updated_user = update_user(&firebase, &res.name, &user).await.unwrap();
            println!("{:?}", updated_user);

            delete_user(&firebase, &res.name).await.unwrap();
            println!("User Deleted..!");
        },
        Err(e) => eprintln!("Failed to set user: {}", e),
    }
}

async fn set_user(firebase_client: &Firebase, user: &User) -> Result<Response, String> {
    let firebase = firebase_client.at("users");
    let result = firebase.set::<User>(&user).await;
    
    match result {
        Ok(res) => Ok(string_to_response(&res.data)),
        Err(e) => Err(format!("Error setting user: {}", e)),
    }
}

async fn get_users(firebase_client: &Firebase) -> Result<HashMap<String, User>, String> {
    let firebase = firebase_client.at("users");
    let result = firebase.get::<HashMap<String, User>>().await;

    match result {
        Ok(res) => Ok(res),
        Err(e) => Err(format!("Error getting users: {}", e)),
    }
}

async fn get_user(firebase_client: &Firebase, id: &String) -> Result<User, String> {
    let firebase = firebase_client.at("users").at(&id);
    let result = firebase.get::<User>().await;

    match result {
        Ok(res) => Ok(res),
        Err(e) => Err(format!("Error getting user: {}", e)),
    }
}

async fn update_user(firebase_client: &Firebase, id: &String, user: &User) -> Result<User, String> {
    let firebase = firebase_client.at("users").at(&id);
    let result = firebase.update::<User>(&user).await;

    match result {
        Ok(res) => Ok(string_to_user(&res.data)),
        Err(e) => Err(format!("Error updating user: {}", e)),
    }
}

async fn delete_user(firebase_client: &Firebase, id: &String) -> Result<(), String> {
    let firebase = firebase_client.at("users").at(&id);
    let result = firebase.delete().await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error deleting user: {}", e)),
    }
}

// convert string to response
fn string_to_response(s: &str) -> Response {
    serde_json::from_str(s).unwrap_or_else(|e| {
        eprintln!("Failed to parse response: {}", e);
        Response {
            name: "".to_string(),
        }
    })
}

// convert string to user
fn string_to_user(s: &str) -> User {
    serde_json::from_str(s).unwrap_or_else(|e| {
        eprintln!("Failed to parse user: {}", e);
        User {
            name: "".to_string(),
            age: 0,
            email: "".to_string(),
        }
    })
}
