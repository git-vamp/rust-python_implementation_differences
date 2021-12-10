use std::fs::create_dir;
pub fn mkdir(s: &str) -> () {
    let f = create_dir(s);
    if let Err(e) = f {
        println!("Error: {} {}",e, s);
    } else {
        ()
    }
}