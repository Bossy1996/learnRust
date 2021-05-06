
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    

    let mut user1 = User {
        username: String::from("Username"),
        email: String::from("email@email.com"),
        sign_in_count: 1,
        active: true,
    };

    user1.email = String::from("anotherEmail@email.com");

    let _user2 = create_user(String::from("test@test.com"), String::from("username2"));

}

fn create_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: false,
        sign_in_count: 0,
    }
}

