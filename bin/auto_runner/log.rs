// Imports /////////////////////////////////////////////////////////////////////
use crate::{env::Env, error::Error};

// Logger //////////////////////////////////////////////////////////////////////
pub struct Logger {
    pushover: Pushover,
}

impl Logger {
    pub fn new(env: &Env) -> Self {
        // Initialize pushover
        let pushover =
            Pushover::new(env.pushover_api.clone(), env.pushover_user.clone());

        Self { pushover }
    }

    pub fn err(&self, s: &str) {
        self.pushover.send("Error", s, true);
    }
}

// Pushover ////////////////////////////////////////////////////////////////////
struct Pushover {
    api_key: String,
    user_key: String,

    api: pushover::API,
}

impl Pushover {
    fn new(api_key: String, user_key: String) -> Self {
        let api = pushover::API::new();

        Self { api_key, user_key, api }
    }

    fn send(&self, title: &str, body: &str, error: bool) -> Result<(), Error> {
        // Create message
        let mut msg = pushover::requests::message::SendMessage::new(
            self.api_key.clone(),
            self.user_key.clone(),
            body,
        );

        // Modify message
        msg.set_title(title);

        msg.set_url("https://github.com/biwecka/gax-plots");
        msg.set_url_title("GAX-Plots on GitHub");

        if error {
            msg.set_priority(pushover::Priority::Normal);
        } else {
            msg.set_priority(pushover::Priority::Low);
        }

        // Send message
        self.api.send(&msg)?;

        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////
