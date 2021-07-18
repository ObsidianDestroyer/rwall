use std::collections::HashMap;

struct Argument {
    pattern: String,
    value: String,
}
impl Argument {
    fn new(pattern: String, value: String) -> Argument {
        Argument {pattern, value}
    }
}

fn main() {
    use std::env;
    let arguments: Vec<Argument> = env::args().enumerate()
        .filter(|&(i, _)| i != 0)
            .map(
                |(_, args)| args
            )
        .collect::<Vec<String>>()
        .chunks(2)
            .map(
                |args| Argument::new(
                    args[0].clone(), args[1].clone()
                )
            ).collect();
    for arg in arguments {
        let mut hash = HashMap::new();
        hash.insert("ptr", arg.pattern);
        hash.insert("val", arg.value);
        dbg!(hash);
    }
}



