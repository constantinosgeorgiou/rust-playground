struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn print_user(user: User) {
    // println!("{:ident$}user:\n",ident=ident);
    println!("user:");
    println!("  username: {}", user.username);
    println!("  email: {}", user.email);
    println!("  active: {}", user.active);
    println!("  sign_in_count: {}", user.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Tuple structs

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Define user2 from user1.
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("anotheremail@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    // Define user2 from user1 better way.
    let user2 = User {
        email: String::from("anotheremail@example.com"),
        ..user1 // Must come in last
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
