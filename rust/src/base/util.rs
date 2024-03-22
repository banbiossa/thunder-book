use std::fs::OpenOptions;
use std::io::Write;

pub fn log_and_print(message: &str) {
    println!("{}", message);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("output.md")
        .unwrap();
    writeln!(file, "{}", message).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_log_and_print() {
        // Remove the output file if it exists
        let _ = fs::remove_file("output.md");

        // Call the log_and_print function with a test message
        let test_message = "This is a test message.";
        log_and_print(test_message);

        // Read the contents of the output file
        let file_contents = fs::read_to_string("output.md").unwrap();

        // Assert that the file contains the test message
        assert!(file_contents.contains(test_message));

        // Clean up the output file
        fs::remove_file("output.md").unwrap();
    }
}
