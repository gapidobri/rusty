use serenity::{client::Context, model::channel::Message};

pub async fn help(context: Context, message: Message) {
  message
    .channel_id
    .send_message(context.clone().http, |m| {
      m.embed(|e| {
        e.title("RustyBot Help")
          .field("r!help", "Shows this message", false)
          .field("r!ping", "Replies with pong", false)
          .field("r!nick", "Changes nickname", false)
          .field("r!author", "Shows the bot author", false)
          .field("r!source", "Shows the link to source code", false)
          .field("r!roll", "Provides you with a link to rickroll", false)
          .field("r!say", "Repeats after you", false)
      });
      m
    })
    .await
    .expect("Error sending message");
}
