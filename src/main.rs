// Example story: He was so [adjective] that he [verb]ed John in the face,
// sending him into a/an [noun].

extern crate "rust-madlibs" as madlibs;
use madlibs::read_input;
use madlibs::fill_blanks;

fn main() {
    let mut template = read_input();
    // println!("{}", template);

    template = fill_blanks(template.as_slice());

    println!("{}", template);
}
