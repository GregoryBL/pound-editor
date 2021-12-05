mod lib;

use lib::Editor;

fn main() -> crossterm::Result<()> {
    let mut editor = Editor::new();

    while editor.run()? {}

    Ok(())
}
