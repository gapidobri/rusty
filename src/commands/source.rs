use serenity::client::Context;
use serenity::model::channel::Message;

use crate::helper::send_message;

const AUTHOR_MESSAGE: &str = "
**I'm open source.**
You can find my code here:
https://github.com/gapidobri/rusty
";

pub async fn source(context: Context, message: Message) {
  send_message(context, message.channel_id, AUTHOR_MESSAGE).await;
}
