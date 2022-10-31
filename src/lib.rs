mod qcodes;
mod error_models;
mod env;
mod decoder;
mod libs;
mod logger;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_simple_run() {
    }
}
