// SPDX-License-Identifier: EUPL-1.2

// Copyright litma leung

// TODO: In the future copy more stuff over from https://github.com/openSUSE/osc/blob/master/osc/credentials.py
//       so that we can just use their existing oscrc files and whatnot

use dialoguer::{Input, Password};

pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn ask_credentials() -> Result<Credentials, dialoguer::Error> {
    let username = Input::new().with_prompt("Username").interact_text()?;
    let password = Password::new().with_prompt("Password").interact()?;
    Ok(Credentials { username, password })
}
