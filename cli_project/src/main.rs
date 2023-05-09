use std::error::Error;
use std::fs;
use std::path::Path;


fn main() -> Result<(), Box<dyn Error>> {
    
    // Moves Single File
    let file_to_move = "test_file_1.txt";

    let new_location = format!("./out/{}", file_to_move);

    println!("Moving {} to: {}", file_to_move, new_location);

    let file_existance_check = Path::new(&new_location).exists();

    if file_existance_check {
        println!("File exists, so not moving");
    } else {
        println!("File does not exist, moving!");

        // Since rename runs with "force" by default,
        // we need to manually check if it's already there.
        fs::rename(file_to_move, new_location)?;
    }

    Ok(())
}
