// Example story: He was so [adjective] that he [verb]ed John in the face,
// sending him into a/an [noun].

extern crate "rust-madlibs" as madlibs;
use madlibs::read_input;
use madlibs::fill_blanks;

fn main() {
    let template = read_input();
    // println!("{}", template);

    loop {
        let result = fill_blanks(template);
        if result.as_slice() == template.as_slice() {
            break;
        }
    }

    println!("{}", template);
}
