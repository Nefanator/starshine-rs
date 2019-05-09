extern crate starshine;

#[test]
fn add_user_to_database() {
    let add_user_args: Vec<String> = String::new("Starshine.exe add Nathan").split_whitespace().collect();
    let check_user_args: Vec<String> = String::new("Starshine.exe check Nathan").split_whitespace().collect();
    let add_user_command = starshine::Command::new(&add_user_args).unwrap();
    let check_user_command = starshine::Command::new(&check_user_args).unwrap();
    
    starshine::run(&add_user_command).unwrap();

    assert!(starshine::run(&check_user_command).is_ok());
}

fn delete_user_from_database() {
    unimplemented!()    
}