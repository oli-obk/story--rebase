use color_eyre::{eyre::eyre, Result};
use story_rebase::{parse, span::Spanned};

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .ok_or_else(|| eyre!("first argument must be a filename to process"))?;
    let story = Spanned::read_from_file(path)?;
    let mut story = parse(story.as_ref())?;
    let mut lines = std::io::stdin().lines();
    loop {
        story.print_room();
        let idx = match lines.next() {
            Some(line) => line?.parse()?,
            None => break,
        };
        story.choose(idx)?;
    }
    if std::env::args().skip(2).any(|arg| arg == "--dump-save") {
        println!("{}", story);
    }
    Ok(())
}
