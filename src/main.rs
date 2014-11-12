// Example story: He was so [adjective] that he [verb]ed John in the face,
// sending him into a/an [noun].

extern crate "rust-madlibs" as madlibs;
use madlibs::read_input;

fn main() {
    let akiva = read_input();
    println!("{}", akiva);
}
