use color_eyre::Result;
use story_rebase::{parse, span::Spanned};

fn main() -> Result<()> {
    let path = std::env::args().nth(1).unwrap();
    let story = Spanned::read_from_file(path)?;
    let mut story = parse(story)?;
    let mut lines = std::io::stdin().lines();
    loop {
        story.print_room();
        let idx = match lines.next() {
            Some(line) => line?.parse()?,
            None => break,
        };
        story.choose(idx)?;
    }
    Ok(())
}
