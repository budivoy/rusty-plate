use std::fs;
use std::path::Path;
use std::io;

// Define a struct to hold project information
pub struct RustyPlate<'a> {
    pub template: &'a str,
    pub destination: &'a str,
}

impl<'a> RustyPlate<'a> {
    // Recursive function to copy directories
    fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());
            if src_path.is_dir() {
                RustyPlate::copy_dir_all(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }
        Ok(())
    }

    // Implement the method for initializing the project
    pub fn initialize_project(&self) -> Result<(), std::io::Error> {
        let template = Path::new(self.template);
        let destination = Path::new(self.destination);

        if template.exists() {
            RustyPlate::copy_dir_all(template, destination)?;
            println!("Project initialized successfully at: {}", self.destination);
        } else {
            println!("Template path does not exist");
        }
        Ok(())
    }
}

// Unit tests module
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_initialize_project_with_valid_template_p() {
        // Create a temporary directory for the template and destination
        let temp_dir = tempdir().unwrap();
        let template_dir = temp_dir.path().join("template");
        let destination_dir = temp_dir.path().join("destination");

        // Create a mock template file
        fs::create_dir_all(&template_dir).unwrap();
        let template_file_path = template_dir.join("example.txt");
        let mut file = File::create(&template_file_path).unwrap();
        writeln!(file, "This is a test file").unwrap();

        // Create an instance of RustyPlate
        let rusty_plate = RustyPlate {
            template: template_dir.to_str().unwrap(),
            destination: destination_dir.to_str().unwrap(),
        };

        // Call the method and assert the result
        let result = rusty_plate.initialize_project();
        assert!(result.is_ok(), "Expected Ok but got an error: {:?}", result);

        // Verify that the file was copied to the destination directory
        let copied_file_path = destination_dir.join("example.txt");
        assert!(copied_file_path.exists(), "File not copied to destination");

        // Verify file contents
        let contents = fs::read_to_string(copied_file_path).unwrap();
        assert_eq!(contents, "This is a test file\n", "File contents mismatch");
    }

    #[test]
    fn test_initialize_project_with_invalid_template_n() {
        // Create a temporary directory for the destination
        let temp_dir = tempdir().unwrap();
        let destination_dir = temp_dir.path().join("destination");

        // Non-existent template path
        let non_existent_template = "non_existent_template";

        // Create an instance of RustyPlate
        let rusty_plate = RustyPlate {
            template: non_existent_template,
            destination: destination_dir.to_str().unwrap(),
        };

        // Call the method and assert the result
        let result = rusty_plate.initialize_project();
        assert!(result.is_ok());

        // Since the template doesn't exist, no files should be created in the destination
        assert!(fs::read_dir(&destination_dir).is_err());
    }
}
