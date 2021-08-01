use serenity::{
  client::Context,
  model::{channel::Message, id::UserId},
  utils::parse_mention,
};

use crate::helper::send_message;

const USAGE_MESSAGE: &str = "```r!nick [@user] <nickname>```";

pub async fn nick(context: Context, message: Message, args: Vec<&str>) {
  if args.len() == 1 {
    let nickname = args[0];
    let user_id = message.author.id;
    change_nickame(context, message, user_id, nickname).await;
  } else if args.len() == 2 {
    let user_id = parse_mention(args[0]);
    let nickname = args[1];
    match user_id {
      Some(user_id) => {
        change_nickame(context, message, UserId(user_id), nickname).await;
      }
      None => {
        send_message(context, message.channel_id, "**Invalid user**").await;
      }
    }
  } else {
    send_message(context, message.channel_id, USAGE_MESSAGE).await;
  }
}

async fn change_nickame(context: Context, message: Message, user_id: UserId, nickname: &str) {
  let guild_id = message.guild_id.expect("Error fetching guild");
  if let Err(why) = guild_id
    .edit_member(context.clone().http, user_id, |m| m.nickname(nickname))
    .await
  {
    println!("Error editing members nickname: {:?}", why);
    send_message(context, message.channel_id, "I can't change your nickname").await;
  }
}
