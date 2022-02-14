use dotenv::dotenv;
use reqwest::StatusCode;
use teloxide::{dispatching2::UpdateFilterExt, prelude2::*};

#[tokio::main]
async fn main() {
    run().await
}

async fn run() {
    teloxide::enable_logging!();

    dotenv().ok();

    let bot = Bot::from_env().auto_send();

    let handler = Update::filter_message().branch(dptree::endpoint(
        |msg: Message, bot: AutoSend<Bot>| async move {
            if let Some(users) = msg.new_chat_members() {
                for user in users {
                    let id = user.id;
                    let result = reqwest::get(format!("https://api.cas.chat/check?user_id={}", id))
                        .await
                        .unwrap();
                    if result.status() != StatusCode::NOT_FOUND {
                        bot.ban_chat_member(msg.chat_id(), id).await.unwrap();
                        let name = match user.username.clone() {
                            Some(value) => value,
                            None => String::from("unknown"),
                        };
                        bot.send_message(
                            msg.chat_id(),
                            format!("user '{}' with id {} has been CAS banned", name, id),
                        )
                        .await
                        .unwrap();
                    }
                }
            }
            respond(())
        },
    ));

    Dispatcher::builder(bot, handler)
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;
}
