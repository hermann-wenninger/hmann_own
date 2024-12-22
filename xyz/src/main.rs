use std::fs::File;
use std::io::{self, Write};
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    // Specify the output file
    let output_file = "directory_d.txt";

    // Open the output file (it will be created if it doesn't exist)
    let mut file = File::create(output_file)?;

    // Start walking from the root directory or your desired starting point
    let root_dir = "D:"; // You can change this to any directory you want to start from

    // Walk the directory structure
    for entry in WalkDir::new(root_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        
        // Write the path to the file, adding a newline for separation
        writeln!(file, "{}", path.display())?;
    }

    println!("Directory structure saved to {}", output_file);

    Ok(())
}
