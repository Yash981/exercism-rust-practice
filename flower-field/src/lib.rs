pub fn annotate(garden: &[&str]) -> Vec<String> {
    // todo!(
    //     "\nAnnotate each square of the given garden with the number of flowers that surround said square (blank if there are no surrounding flowers):\n{garden:#?}\n"
    // );
    for &x in garden{
        println!("{:?}",x.to_string());
    }
    return ["12".to_string()].to_vec();

}
