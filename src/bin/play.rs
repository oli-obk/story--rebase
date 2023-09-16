use color_eyre::{
    eyre::{bail, eyre},
    Result,
};
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use story_rebase::{parsing::parse, span::Spanned};

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .ok_or_else(|| eyre!("first argument must be a filename to process"))?;
    let story = Spanned::read_from_file(path)?;
    let mut steps = vec![];
    'start: loop {
        let mut story = parse(story.as_ref())?;
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
                    .map(|choice| &choice.value.message.content)
                    .collect();
                let default = if let Some(choice) = steps.get(story.choices.len()) {
                    usize::from(*choice)
                } else {
                    0
                };
                let idx = Select::with_theme(&ColorfulTheme::default())
                    .items(&items)
                    .default(default)
                    .interact_on_opt(&Term::stderr())?;
                if let Some(idx) = idx {
                    if idx != default {
                        steps.clear();
                    }
                    story.choose(idx)?;
                    break;
                }
            }
        }
        println!("Game Over! Would you like to start over? The choices you took last time will be selected by default");
        loop {
            let idx = Select::with_theme(&ColorfulTheme::default())
                .items(&["Try Again", "Let me out of here!"])
                .default(0)
                .interact_on_opt(&Term::stderr())?;
            match idx {
                Some(0) => {
                    steps = story.choices;
                    continue 'start;
                }
                Some(1) => break 'start,
                Some(other) => {
                    bail!("somehow got selection {other}, but there were only two options")
                }
                None => {}
            }
        }
    }
    Ok(())
}
