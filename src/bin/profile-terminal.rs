use std::io;

struct User {
    bio: String,
    display_name: String,
    username: String,
    email: String,
    login_amount: u32,
}

impl User {
    fn shout(&self) {
        println!("
\n-----------------
Username: {}
Display Name: {}
Bio: {}
-----------------
Email: {}
Logins: {}
-----------------\n
        ", self.username, self.display_name, self.bio, self.email, self.login_amount);
    }

    fn change_user(&mut self) {
        let mut new_user = String::new();
        println!("[Lowercase only] Enter new username:");
        io::stdin().read_line(&mut new_user).expect("Error");
        self.username = new_user.trim().to_lowercase();
    }

    fn change_d_name(&mut self) {
        let mut new_d_name = String::new();
        println!("Enter new display name:");
        io::stdin().read_line(&mut new_d_name).expect("Error");
        self.display_name = new_d_name.trim().to_string();
    }

    fn change_bio(&mut self) {
        let mut new_bio = String::new();
        println!("Enter new bio:");
        io::stdin().read_line(&mut new_bio).expect("Error");
        self.bio = new_bio.trim().to_string();
    }

    fn change_email(&mut self) {
        loop {
            let mut new_email = String::new();
            println!("[Must contain '@'] Enter new email:");
            io::stdin().read_line(&mut new_email).expect("Error");

            if new_email.trim().contains("@") {
                self.email = new_email.trim().to_lowercase();
                break;
            } else {
                println!("Error, not a valid email.");
            }
        }
    }

    fn logins(&mut self) {
        self.login_amount += 1;
    }

    fn choices(&mut self) {
        let mut choice = String::new();
        println!("
\n---------------------------
1 - Change Username
2 - Change Display Name
3 - Change Bio
4 - Change Email 
--------------------------- 
5 - View Profile (Scroll up if already clicked)
---------------------------\n
        ");

        io::stdin().read_line(&mut choice).expect("Error");

        match choice.trim() {
            "1" => self.change_user(),
            "2" => self.change_d_name(),
            "3" => self.change_bio(),
            "4" => self.change_email(),
            "5" => self.shout(),
            _ => println!("Error"),
        }
    }
}

fn main() {
    let mut user1 = User {
        bio: String::from("No Bio yet"),
        display_name: String::from("No Name yet"),
        username: String::from("No Username yet"),
        email: String::from("No Email yet"),
        login_amount: 0,
    };

    loop {
        user1.choices();
    }
}
