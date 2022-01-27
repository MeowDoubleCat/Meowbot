use crate::ParseMode;
use anyhow::Result;
use std::env;
use teloxide::requests::ResponseResult;
use teloxide::utils::command::BotCommand;
use teloxide::{adaptors::DefaultParseMode, prelude::*};

#[derive(BotCommand, Debug)]
#[command(rename = "lowercase")]
pub enum Command {
    Start,
    Ping,
    #[command(parse_with = "split")]
    Send {
        userid: i64,
        msg: String,
    },
}

type Bot = AutoSend<DefaultParseMode<teloxide::Bot>>;
type Cx = UpdateWithCx<Bot, Message>;

pub async fn handler(cx: Cx) -> Result<()> {
    let admin_id = env::var("admin_id").expect("admin id undefined");
    let admin_id = admin_id.parse::<i64>().unwrap();

    let text = match cx.update.text() {
        None => {
            if cx.update.chat.username().is_some() {
                cx.forward_to(admin_id).await?;
                cx.requester
                    .send_message(
                        admin_id,
                        format!(
                            "@{}  ,  <code>/send {}</code>",
                            cx.update.chat.username().unwrap(),
                            cx.update.chat_id()
                        ),
                    )
                    .await?;
            } else {
                cx.answer("请设置你的 TG用户名 否则无法使用本机器人!")
                    .await?;
            }

            return Ok(());
        }
        Some(text) => text,
    };

    let command = match Command::parse(text, "meow-bot") {
        Err(_) => {
            if cx.update.chat_id().is_positive() && cx.update.chat_id() != admin_id {
                if cx.update.chat.username().is_some() {
                    cx.requester
                        .send_message(
                            admin_id,
                            format!(
                                "@{}\n\n{}\n\n<code>/send {}</code>",
                                cx.update.chat.username().unwrap(),
                                cx.update.text().unwrap(),
                                cx.update.chat_id()
                            ),
                        )
                        .parse_mode(ParseMode::Html)
                        .await?;
                } else {
                    cx.answer("请设置你的 TG用户名 否则无法使用本机器人!")
                        .await?;
                }
            }
            return Ok(());
        }
        Ok(command) => command,
    };
    let _ = match command {
        Command::Start => start(&cx).await,
        Command::Ping => ping(&cx).await,
        Command::Send { userid, msg } => send(&cx, userid, msg).await,
    };
    Ok(())
}

async fn start(cx: &Cx) -> ResponseResult<Message> {
    cx.answer("你好!\n\n您可以使用此机器人与我们联系.\n\n To: @MeowDoubleCat")
        .await
}

async fn ping(cx: &Cx) -> ResponseResult<Message> {
    cx.answer("pong... 跟我说话...").await
}

async fn send(cx: &Cx, id: i64, msg: String) -> ResponseResult<Message> {
    let admin_id = env::var("admin_id").unwrap();
    if cx.chat_id() == admin_id.parse::<i64>().unwrap() {
        cx.requester.send_message(id, msg).await
    } else {
        cx.answer("not an administrator").await
    }
}
