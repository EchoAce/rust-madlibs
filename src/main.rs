// Example story: He was so [adjective] that he [verb]ed John in the face,
// sending him into a/an [noun].

extern crate "rust-madlibs" as madlibs;
use madlibs::read_input;
use madlibs::fill_blanks;

fn main() {
    let template = read_input();
    // println!("{}", template);
    let mut story = fill_blanks(template.as_slice());
    
    loop {
        println!("{}", story.as_slice());
        if story.match_indices("[").next() == None {
            break;
        }
        story = fill_blanks(story.as_slice());
    }

    println!("{}", story);
}
