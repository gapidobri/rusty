use serenity::{client::Context, model::id::ChannelId};

/// Send message to channel
pub async fn send_message(
  context: Context,
  channel_id: ChannelId,
  content: impl std::fmt::Display,
) {
  if let Err(why) = channel_id.say(context.http, content).await {
    println!("Error sending message: {:?}", why);
  }
}
