use serenity::client::Context;
use serenity::model::channel::Message;

pub async fn ping(context: Context, message: Message) {
  if let Err(why) = message.channel_id.say(context.http, "Pong!").await {
    println!("Error sending message: {:?}", why);
  };
}
