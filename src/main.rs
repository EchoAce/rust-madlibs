// Example story: He was so [adjective] that he [verb]ed John in the face,
// sending him into a/an [noun].

use std::io;

fn main() {
    let mut story = "";
    println!("Please enter a line of the story. Enter a blank line to stop");
    for line in io::stdin().lines() {
        match line.to_string().as_slice() {
            "Ok(\n)" => break,
            _ => {
                println!("Please enter a line of the story. Enter a blank line to stop");
                //story = concat!(story, _);
            },
        }
    }
}
