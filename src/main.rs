// Example story: He was so [adjective] that he [verb]ed John in the face, sending him into a/an [noun].


use std::io::File;

fn main() {
    let story = File::open(&Path::new("story.txt")).read_to_end();
    match story {
        Ok(story) => println!("{}", story),
        Err(msg) => println!("Oops: {}", msg),
    }
}
