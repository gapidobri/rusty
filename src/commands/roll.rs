use serenity::client::Context;
use serenity::model::channel::Message;

use crate::helper::send_message;

const ROLL_MESSAGE: &str = "http://okn3.orb.si/rickroll.mp4";

pub async fn roll(context: Context, message: Message) {
  send_message(context, message.channel_id, ROLL_MESSAGE).await;
}
