#[derive(Clone)]
pub struct UserLogin {
    pub username: String,
}

#[derive(Clone)]
pub struct UserAuth {

}

#[derive(Clone)]
pub struct User {
    pub login: UserLogin,
    pub auth: UserAuth,
}

impl User {
    pub fn new(username: String) -> Self {
        let mut auth_data = UserAuth {

        };
        Self {
            login: UserLogin {
                username: username,
            },
            auth: auth_data,
        }
    }
}

pub fn auth() {
    const CID: &str = "a0303825-96c6-48fd-b66a-21e88b968f44";
    let client = &reqwest::blocking::Client::new();
    let device_code = ms_auth_mc::DeviceCode::new(CID, None, client).unwrap();

    match &device_code.inner {
        None => (),
        Some(inner) => {
            println!("{}", inner.message)
        }
    }

    let mca = device_code.authenticate(client).unwrap(); // Never use unwrap here, it's used in this example for simplicity
    println!("{:?}", mca);
}
