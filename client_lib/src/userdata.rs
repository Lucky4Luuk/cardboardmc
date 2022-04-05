#[derive(Clone)]
pub struct UserLogin {
    pub username: String,
}

#[derive(Clone)]
pub struct User {
    pub login: UserLogin,
}

impl User {
    pub fn new(username: String) -> Self {
        Self {
            login: UserLogin {
                username: username,
            }
        }
    }
}
