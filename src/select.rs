use crate::list::FunctionType;
use crate::list::list;

pub fn select(index: Option<usize>) {
    let themes = match list(false) {
        FunctionType::Value(th) => th,
        FunctionType::Json => return,
    };

    let sel_index: usize = match index {
        None => get_index(themes.themes.len()),
        Some(ind) => {
            if ind > themes.themes.len() - 1 {
                println!(
                    "Invalid Index Provide. Please provide from (0 - {})",
                    themes.themes.len() - 1
                );
                get_index(themes.themes.len())
            } else {
                ind
            }
        }
    };

    println!("{}", sel_index);
}

fn get_index(len: usize) -> usize {
    loop {
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).expect("Dumb ass!");

        match input.trim().parse() {
            Ok(num) => {
                if num > len - 1 {
                    println!(
                        "Invalid Index Provide. Please provide from (0 - {})",
                        len - 1
                    );
                    continue;
                } else {
                    return num;
                }
            }
            Err(_) => {
                println!(
                    "Invalid Index Provide. Please provide from (0 - {})",
                    len - 1
                );
                continue;
            }
        };
    }
}
