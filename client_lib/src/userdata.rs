#[derive(Debug, Clone)]
pub struct UserLogin {
    pub username: String,
}

#[derive(Debug, Clone)]
pub struct UserAuth {
    pub token: String,
}

/// Create this using `auth()`
#[derive(Debug, Clone)]
pub struct User {
    pub login: UserLogin,
    pub auth: UserAuth,
}

pub fn auth() -> User {
    const CID: &str = "a0303825-96c6-48fd-b66a-21e88b968f44";
    let client = &reqwest::blocking::Client::new();
    let device_code = ms_auth_mc::DeviceCode::new(CID, None, client).unwrap();

    match &device_code.inner {
        None => (),
        Some(inner) => {
            println!("{}", inner.message)
        }
    }

    let mca = device_code.authenticate(client).unwrap(); //TODO: Don't unwrap here, we need to handle authentication errors

    User {
        login: UserLogin {
            username: mca.name,
        },
        auth: UserAuth {
            token: mca.token,
        },
    }
}
