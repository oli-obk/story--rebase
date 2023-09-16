use std::path::PathBuf;

use color_eyre::{
    eyre::{bail, Context},
    Result,
};
use story_rebase::{parse, span::Spanned};

#[test]
fn roundtrip_all() -> Result<()> {
    for file in glob::glob("**/*.story")? {
        let file = file?;
        roundtrip(file.clone()).context(file.display().to_string())?;
    }
    Ok(())
}

fn roundtrip(path: PathBuf) -> Result<()> {
    let story = Spanned::read_from_file(&path)?;
    let story = parse(story)?;
    let save = story.to_string();
    let file_content = std::fs::read_to_string(path)?;
    if file_content != save {
        println!("{} != {}", file_content.lines().count(), save.lines().count());
        bail!(
            "{}",
            pretty_assertions::StrComparison::new(&file_content, &save)
        );
    }
    Ok(())
}
