
use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::types::InputFile;
use crate::equation_renderer::render;

mod equation_renderer;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
enum Command {
    #[command(description = "start the bot")]
    Start,
    #[command(description = "show this help")]
    Help,
    #[command(description = "throw a dice")]
    Dice,
    #[command(description = "format an equation")]
    Equation,
}

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let args: Option<&str> = msg.text().and_then(|t| {
        t.split_once(char::is_whitespace)   // Extract the argument included with the command.
            .map(|(_, rest)| rest.trim())
    });

    match cmd {
        Command::Start => {
            bot.send_message(msg.chat.id,
                             "Hello! I'm the real zaydi hehehehehe 😈")
                .await?;
        }

        Command::Help => {
            bot.send_message(msg.chat.id,
                             Command::descriptions().to_string())
                .await?;
        }

        Command::Dice => {
            bot.send_dice(msg.chat.id)
                .await?;
        }

        Command::Equation => {
            let png = render(args.unwrap());

            bot.send_photo(msg.chat.id, InputFile::memory(png).file_name("equation.png"))
                .await?;
        }
    }

    Ok(())
}