use crate::os::mkdir::mkdir;
mod os;
fn main() {

    let structure = [
        ("store", 1),
        ("aisle", 1),
        ("products", 5)
    ];

    let mut  base_path = "base_dir/".to_string();
    mkdir(&base_path);
    for i in structure {
        for  q in 1..i.1+1 {
            let formated_folder_structure  = format!("{}{}_{}", base_path, i.0, &q);
            if q == i.1 {
                mkdir(&formated_folder_structure);
                base_path.push_str(format!("{}_{}/", i.0, &q).as_str())
            } else {
                mkdir(&formated_folder_structure);
            }
            println!("{}", &formated_folder_structure);
        }
    }
}
