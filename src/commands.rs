use serenity::{client::Context, model::channel::Message};

use crate::helper::send_message;

use crate::commands::help::help;
use crate::commands::ping::ping;

use self::nick::nick;

mod help;
mod nick;
mod ping;

const PREFIX: &str = "r!";

/// Run command based on message
pub async fn run_command(context: Context, message: Message) {
  let content = message.clone().content;

  if content.len() < PREFIX.len() {
    return;
  }
  let prefix = &content[..PREFIX.len()];
  if prefix != PREFIX {
    return;
  }
  let mut args = content.split_ascii_whitespace().collect::<Vec<&str>>();
  let command = &args[0][PREFIX.len()..];

  args.remove(0);
  match command {
    "ping" => ping(context, message).await,
    "nick" => nick(context, message, args).await,
    "help" => help(context, message).await,
    _ => no_command(context, message).await,
  }
}

const NO_COMMAND_MESSAGE: &str = "
**This command doesn't exist!**
Please run `r!help` to get the list of commands.
";

/// Send no command message
async fn no_command(context: Context, message: Message) {
  send_message(context, message.channel_id, NO_COMMAND_MESSAGE).await;
}
