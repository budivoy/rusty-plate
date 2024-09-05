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
