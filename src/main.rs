fn main() {

    println!("Please, enter your name: ");


    let mut name : String = String::new();

    //std: machine     io: inputs and outputs    stdin: in
    std::io::stdin().read_line(&mut name).unwrap();
}
