// Result represents either "success" or "failure".
// Ok(variable_name) ... completes operation
// Err(variable_name)... error occurred

fn get_sound(name: &str) -> Result<SoundData, String> {
    if name == "alert" {
        Ok(SoundData::new("alert"));
    } else {
        Err("unable to find sound data".to_owned());
    }
}

fn main() {
    let sound = get_sound("alert");
    match sound {
        Ok(_) => println!("sound data located"),
        Err(e) => println!("error:{:?}", e),
    }
}
