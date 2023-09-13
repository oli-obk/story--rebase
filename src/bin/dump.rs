use color_eyre::Result;
use story_rebase::{parse, span::Spanned};

fn main() -> Result<()> {
    let path = std::env::args().nth(1).unwrap();
    let story = Spanned::read_from_file(path)?;
    let story = parse(story)?;
    println!("{story:#?}");
    Ok(())
}
