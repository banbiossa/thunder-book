#[macro_export]
macro_rules! log_and_print {
    ($($arg:tt)*) => {
        {
            use std::fs::OpenOptions;
            use std::io::Write;

            let fmt_string = format!($($arg)*);
            println!("{}", fmt_string);
            // Write the formatted string to a file or perform any other logging operation
            // For example:
            // write!(file_handle, "{}\n", fmt_string).unwrap();
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("output.md")
                .unwrap();
            writeln!(file, "{}", fmt_string).unwrap();
        }
    };
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_log_and_print() {
        // Remove the output file if it exists
        let _ = fs::remove_file("output.md");

        // Call the log_and_print function with a test message
        log_and_print!("This is a test message.");

        // Read the contents of the output file
        let file_contents = fs::read_to_string("output.md").unwrap();

        // Assert that the file contains the test message
        assert!(file_contents.contains("This is a test message."));

        // Clean up the output file
        fs::remove_file("output.md").unwrap();
    }
}
