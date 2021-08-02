use serenity::client::Context;
use serenity::model::channel::Message;

use crate::helper::send_message;

pub async fn say(context: Context, message: Message, args: Vec<&str>) {
  let reply = args.join(" ");
  send_message(context, message.channel_id, reply).await;
}
