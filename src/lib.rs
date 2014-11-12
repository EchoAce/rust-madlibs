use std::io;

pub fn read_input() -> String {
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
    story.to_string()
}
