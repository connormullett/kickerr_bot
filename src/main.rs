use dotenv::dotenv;

use serde::{Deserialize, Serialize};
use teloxide::{dispatching2::UpdateFilterExt, prelude2::*};

#[derive(Serialize, Deserialize)]
struct CasResponse {
    ok: bool,
    description: Option<String>,
    result: Option<String>,
}

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
                    let result: CasResponse =
                        reqwest::get(format!("https://api.cas.chat/check?user_id={}", user.id))
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
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
