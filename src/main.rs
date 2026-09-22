
use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::types::{InputFile, ParseMode};
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
    #[command(description = "show equation writing help (English)")]
    HelpEqEn,
    #[command(description = "show equation writing help (Arabic)")]
    HelpEqAr
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

        Command::HelpEqEn => {
            const HELP_EN: &str = r#"<b>📐 LaTeX Cheat Sheet</b>

Send <code>/equation</code> followed by your formula.

<b>Basic examples</b>
<blockquote><code>/equation x^2 + 2x + 1 = 0</code>
<code>/equation E = mc^2</code>
<code>/equation a_n = a_1 + (n-1)d</code></blockquote>

<b>Common symbols</b>
<blockquote><code>\frac{a}{b}</code> — fraction  a/b
<code>\sqrt{x}</code> — square root
<code>\sqrt[n]{x}</code> — n-th root
<code>x^{2}</code> — power
<code>x_{i}</code> — subscript
<code>\pm</code> — plus/minus  ±
<code>\times</code> — multiply  ×
<code>\le</code> <code>\ge</code> <code>\ne</code> — ≤ ≥ ≠
<code>\alpha \beta \pi \theta</code> — Greek letters</blockquote>

<b>Big operators</b>
<blockquote><code>\sum_{i=1}^{n} i</code> — sum
<code>\int_{0}^{1} x\,dx</code> — integral
<code>\lim_{x \to 0} f(x)</code> — limit</blockquote>

<b>Tip</b>
Use <code>{ }</code> around anything with more than one character:
<code>x^12</code> renders as x¹2, but <code>x^{12}</code> renders as x¹²."#;

            bot.send_message(msg.chat.id, HELP_EN)
                .parse_mode(ParseMode::Html)
                .await?;
        }

        Command::HelpEqAr => {
            const HELP_AR: &str = "\u{202B}<b>📐 دليل كتابة معادلات LaTeX</b>

أرسل <code>/equation</code> متبوعًا بالمعادلة.

<b>أمثلة أساسية</b>
<blockquote><code>/equation x^2 + 2x + 1 = 0</code>
<code>/equation E = mc^2</code>
<code>/equation a_n = a_1 + (n-1)d</code></blockquote>

<b>رموز شائعة</b>
<blockquote><code>\\frac{a}{b}</code> — كسر  a/b
<code>\\sqrt{x}</code> — جذر تربيعي
<code>\\sqrt[n]{x}</code> — جذر من الرتبة n
<code>x^{2}</code> — أُس
<code>x_{i}</code> — دليل سفلي
<code>\\pm</code> — زائد/ناقص  ±
<code>\\times</code> — ضرب  ×
<code>\\le</code> <code>\\ge</code> <code>\\ne</code> — ≤ ≥ ≠
<code>\\alpha \\beta \\pi \\theta</code> — حروف يونانية</blockquote>

<b>عمليات كبيرة</b>
<blockquote><code>\\sum_{i=1}^{n} i</code> — مجموع
<code>\\int_{0}^{1} x\\,dx</code> — تكامل
<code>\\lim_{x \\to 0} f(x)</code> — نهاية</blockquote>

<b>نصيحة</b>
ضع <code>{ }</code> حول أي شيء يحتوي على أكثر من رمز:
<code>x^12</code> تظهر x¹2، بينما <code>x^{12}</code> تظهر x¹².\u{202C}";

            bot.send_message(msg.chat.id, HELP_AR)
                .parse_mode(ParseMode::Html)
                .await?;
        }
    }

    Ok(())
}