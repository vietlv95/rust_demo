pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl User {
    pub fn new(first_name: String, last_name: String, email: String) -> User {
        User {
            first_name,
            last_name,
            email,
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn main() {
    let user = User::new("viet".to_string(), "le".to_string(), "vietlv@gscfin.com".to_string());
    println!("full name: {}", user.full_name());
}