pub fn select(index: Option<usize>) {
    // let themes = match list(false, true) {
    //     FunctionType::Value(th) => th,
    //     FunctionType::Format(_) | FunctionType::Json(_) => return,
    // };

    let sel_index: usize = match index {
        None => get_index(),
        Some(ind) => ind,
    };

    println!("{}", sel_index);
}

fn get_index() -> usize {
    let mut input: String = String::new();
    loop {
        std::io::stdin().read_line(&mut input).expect("Dumb ass!");

        if input.trim() == "e" {
            println!("Exited!");
            continue;
        }

        let parsed_index: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Index");
                continue;
            }
        };

        return parsed_index;
    }
}
