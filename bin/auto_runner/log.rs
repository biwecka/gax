// Imports /////////////////////////////////////////////////////////////////////
use crate::{env::Env, error::Error};

// Logger //////////////////////////////////////////////////////////////////////
/// Custom logging functionality for the use-case of the auto runner.
pub struct Logger {
    pushover: Pushover,
}

impl Logger {
    /// Create a new logger, which requires the [`Env`] struct to provide
    /// credentials for [`Pushover`].
    pub fn new(env: &Env) -> Self {
        // Initialize pushover
        let pushover =
            Pushover::new(env.pushover_api.clone(), env.pushover_user.clone());

        Self { pushover }
    }

    /// Log an error event.
    pub fn err(&self, s: &str) {
        // Try to send Push-Notification
        let mut res = self.pushover.send("Error", s, true);

        // Retry-Counter
        let mut counter = 0;

        // If senting the notification didn't work, try again.
        // -> every 10 seconds for a total of 10 minutes
        while res.is_err() && counter <= 60 {
            println!("[WRN] Retrying to send Push-Notification");
            std::thread::sleep(std::time::Duration::from_secs(10));

            res = self.pushover.send("Error", s, true);
            counter += 1;
        }
    }

    /// Log a success event.
    pub fn success(&self, s: &str) {
        // Try to send Push-Notification
        let mut res = self.pushover.send("Success", s, false);

        // Retry-Counter
        let mut counter = 0;

        // If senting the notification didn't work, try again.
        // -> every 10 seconds for a total of 10 minutes
        while res.is_err() && counter <= 60 {
            println!("[WRN] Retrying to send Push-Notification");
            std::thread::sleep(std::time::Duration::from_secs(10));

            res = self.pushover.send("Success", s, false);
            counter += 1;
        }
    }
}

// Pushover ////////////////////////////////////////////////////////////////////
/// This struct provides a thin wrapper around the `pushover` create, to only
/// only the functionality needed in this use-case.
struct Pushover {
    api_key: String,
    user_key: String,

    api: pushover::API,
}

impl Pushover {
    /// Create a new `Pushover` wrapper.
    fn new(api_key: String, user_key: String) -> Self {
        let api = pushover::API::new();

        Self { api_key, user_key, api }
    }

    /// Send a push-notification with the given parameters.
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
