mod note;
use note::Note;

fn main() {
    let note = Note::new(440.0, 1.0);
    note.play();
}
