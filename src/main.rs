#[derive(Debug, PartialEq)]
enum Val {
    Symbol(String),
    List(Vec<Val>),
    Int(i32),
    Float(f64),
}

fn tokenize(s: &str) -> String {
    s.replace('(', " ( ")
     .replace(')', " ) ")
     .split_whitespace()
     .collect()
}

// TODO 
fn parse(program: &str) -> Val { 
    Val::Int(0)
    // read_from_tokens(tokenize(program))
}

fn read_from_tokens(tokens: &mut Vec<String>) -> Val {
    if tokens.len() == 0 {
        panic!("Unexpected EOF while reading token stream.");
    }

    let token = tokens.remove(0);

    if token == "(" {
        let mut l = Vec::new();

        while tokens[0] != ")" {
            l.push(read_from_tokens(tokens));
        }
        tokens.remove(0); // Get rid of ")"
        return Val::List(l);
    } else if ")" == token {
        panic!("Unexpected rparen while reading token stream.");
    } else {
        return atom(&token);
    }
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
