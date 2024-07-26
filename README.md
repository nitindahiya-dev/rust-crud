<h2># 📚 **Firebase CRUD with Rust**</h2>

<p>Welcome to **Firebase CRUD with Rust**! This project demonstrates how to perform basic Create, Read, Update, and Delete (CRUD) operations using Firebase Realtime Database with Rust. 🚀</p>

<p>## 💡 **About**</p>

<p>**Firebase CRUD with Rust** is a Rust-based application that interacts with Firebase Realtime Database to manage user data. This project showcases how to:</p>
<li> Add a user to Firebase.</li>
<li> Retrieve user details.</li>
<li> Update user information.</li>
<li> Delete a user.</li>
<br>
<p>### 📌 **Key Features**</p>

<li> **Add User:** Create and store user details in Firebase.</li>
<li> **Retrieve User:** Fetch user data by ID.</li>
<li> **Update User:** Modify existing user data.</li>
<li> **Delete User:** Remove a user from the database.</li>
<br>
<p>## 🚀 **Installation**</p>

<p>To get started with **Firebase CRUD with Rust**, follow these steps:</p>

<p>### Prerequisites</p>

<li> [Rust](https://www.rust-lang.org/tools/install) installed on your system.</li>
<li> [Firebase Realtime Database](https://firebase.google.com/products/realtime-database) set up.</li>
<br>
<p>### Steps</p>

1. **Clone the Repository**

    ```bash
    git clone https://github.com/yourusername/firebase-crud-rust.git
    ```

2. **Navigate to the Project Directory**

    ```bash
    cd firebase-crud-rust
    ```

3. **Add Your Firebase URL**

    Open `src/main.rs` and replace `"ENTER YOUR FIREBASE LINK HERE"` with your Firebase Realtime Database URL.

    ```rust
    let firebase = Firebase::new("https://your-firebase-url.firebaseio.com/").unwrap();
    ```

4. **Build and Run the Application**

    ```bash
    cargo run
    ```
