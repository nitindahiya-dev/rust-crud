use std::collections::HashMap;
use firebase_rs::*;
use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize,Debug)]

struct User{
    name: String,
    age: u32,
    email: String
}

#[derive(Serialize, Deserialize, Debug)]

struct Response{
    name: String
}

#[tokio::main]

async fn main(){
    let user = User{
        name : "Nitin Dahiya".to_string(),
        age: 30,
        email: "nitindahiya00000@gmail".to_string()
    };

    let firebase = Firebase::new().unwrap();
    let response = set_user(&firebase, &user).await?;

    let user = get_user(&firebase, &response.name).await?;
    println!("{:?}", user);

    let users = get_users(&firebase).await?;
    println!("{:?}", users);
    
    user.email = "updated.mail@gmail.com".to_string();
    let updated_user = update_user(&firebase, &response.name, &user).await?;
    println!("{:?}", updated_user);

    delete_user(&firebase, &response.name).await?;
    println!("User Deleted..!");
    
}