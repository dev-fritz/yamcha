use teloxide::{prelude::*, types::InputFile, utils::command::BotCommands};
use std::fs;

use super::pix::Pix;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "return admin social media by name. Example: number, linkedin, github. Default: number.")]
    Admin(String),
    #[command(description = "return bot repository")]
    BotRepository,
    #[command(description = "return admin curriculum by selected language. Example: pt, en, es. Default: pt-br.")]
    Curriculum(String),
    #[command(description = "generate a pix qr-code. Example: name, pix_key, value, city, txt_id.", parse_with = "split")]
    Pix(String, String, String),
    #[command(description = "qr code to donate 1 reall.")]
    Donate,
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            let help_text = Command::descriptions().to_string();
            bot.send_message(msg.chat.id, help_text).await?;
        },
        Command::Admin(social) => {
            let social = social.to_lowercase();
            let response = match social.as_str() {
                "linkedin" => "https://linkedin.com/in/fritz-henrique",
                "github" => "https://github.com/dev-fritz",
                _ => {
                    bot.send_contact(msg.chat.id, "5595991561987", "Fritz Henrique").await?;
                    return Ok(());
                }
            };
            bot.send_message(msg.chat.id, response).await?;
        },
        Command::BotRepository => {
            bot.send_message(msg.chat.id, "https://github.com/dev-fritz/yamcha").await?;
        },
        Command::Curriculum(language) => {
            let language = language.to_lowercase();
            let url = match language.as_str() {
                "en" | "es" => "https://drive.google.com/uc?export=download&id=1nAG-zmxyKnlASANplI0ddmNY1y55f0B3",
                _ => "https://drive.google.com/uc?export=download&id=1nAG-zmxyKnlASANplI0ddmNY1y55f0B3",
            };
            bot.send_message(msg.chat.id, url).await?;
        },
        Command::Pix(name, pix_key, value) => {
            let qr = Pix::generate_pix(&mut Pix {
                name,
                pix_key,
                value,
                city: "Boa Vista".to_string(),
                txt_id: "yamcha".to_string()
            });
            bot.send_message(msg.chat.id, qr).await?;
            bot.send_document(msg.chat.id, InputFile::file("./temp_files/out.png")).await?;
            
            fs::remove_file("./temp_files/out.png")?;
        },
        Command::Donate => {
            bot.send_document(msg.chat.id, InputFile::file("./temp_files/donate.png")).await?;
        }
    };

    Ok(())
}
