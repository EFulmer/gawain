#[derive(Debug, PartialEq)]
enum Val {
    Symbol(String),
    List(Vec<Val>),
    Int(i32),
    Float(f64),
}

fn atom(s: &str) -> Val {
    if let Ok(i) = s.parse::<i32>() {
        return Val::Int(i);
    }
    else if let Ok(f) = s.parse::<f64>() {
        return Val::Float(f);
    }
    else {
        return Val::Symbol(s.to_owned());
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse_int() {
        assert_eq!(super::Val::Int(4), super::atom("4"));
    }

    #[test]
    fn parse_float() {
        assert_eq!(super::Val::Float(2.8), super::atom("2.8"));
    }

}
