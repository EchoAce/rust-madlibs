use std::io;

// read_input keeps taking in lines from stdin until a single newline
// is entered. It returns a String.
pub fn read_input() -> String {
    let mut story = "".to_string();
    println!("Please enter a line of the story. Enter a blank line to stop");
    loop {
        let input = io::stdin().read_line().ok().expect("Failed to read line.");
        let line = input.as_slice();
        if line == "\n" {
            break;
        }
        else {
            story.push_str(line);
        }
    }
    story.to_string()
}
