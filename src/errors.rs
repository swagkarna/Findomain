use crate::{args, structs::Args};
pub use failure::{Error, ResultExt};
pub type Result<T> = ::std::result::Result<T, Error>;

pub fn check_request_errors(error: reqwest::Error, api: &str) {
    let args = args::get_args();
    if !args.quiet_flag && args.verbose {
        eprintln!("❌ Error in {} API. {} ", &api, &error)
    }
}

pub fn check_json_errors(error: reqwest::Error, api: &str) {
    let args = args::get_args();
    if !args.quiet_flag && args.verbose {
        eprintln!("❌ An error occurred while parsing the JSON obtained from the {} API. Error description: {:?}.", &api, error)
    }
}

pub fn check_monitoring_parameters(args: &mut Args) -> Result<()> {
    if !args.telegram_bot_token.is_empty() && !args.telegram_chat_id.is_empty() {
        args.telegram_webhook = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            args.telegram_bot_token
        )
    }
    if args.discord_webhook.is_empty()
        && args.slack_webhook.is_empty()
        && args.telegram_webhook.is_empty()
    {
        eprintln!("You must provide a webhook to use the monitoring API.");
        std::process::exit(1)
    }
    Ok(())
}
