use color_eyre::{eyre::eyre, Result};
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use story_rebase::{parse, span::Spanned};

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .ok_or_else(|| eyre!("first argument must be a filename to process"))?;
    let story = Spanned::read_from_file(path)?;
    let mut story = parse(story)?;
    loop {
        let room = story.room();
        println!("{}", room.message.content);
        if room.choices.is_empty() {
            break;
        }
        loop {
            let items: Vec<_> = room
                .choices
                .iter()
                .map(|(msg, _, _)| &msg.content)
                .collect();
            let idx = Select::with_theme(&ColorfulTheme::default())
                .items(&items)
                .default(0)
                .interact_on_opt(&Term::stderr())?;
            if let Some(idx) = idx {
                story.choose(idx)?;
                break;
            }
        }
    }
    Ok(())
}
