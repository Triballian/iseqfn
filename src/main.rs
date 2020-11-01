
//take experssion, gather exponents, if exponent 2 has an expression, not function
//types of equations
// Quadratic Equation
// Linear Equation
// Radical Equation
// Exponential Equation
// Rational Equation
//term
#[derive(Debug)]
struct Equation {
    equation: String,
    isfunc: bool,
    term: Option<Term>,
}

impl Equation {
    fn new(mut eqtion: String) -> Self {
        eqtion.retain(|c| !c.is_Whitespace());
        Self {
            equation: eqtion,
            isfunc: true,
            term: None,
        }
    }
    fn process(&self) {
        plintln!("isfunc {}", self.isfunc);
    }
}
struct Term {
    ttype: char, // x or y or n, y , x or nuymber type
    term: String,
}
impl Term {
    fn new (eqtion: String) -> Self {
        //let i = 1;
        let t = vec![String];
        for (i,c) in eqtion.chars().enum(){
            t.add(c);
            let nexti = i + 1;
            if String.as_byte()[nexti] != ("+" | "-" | "*" | "=") {
                
            } else {
                t
            }

        }
        Self {

        }
    }
}
// struct Expression {
//     base: i32,
//     var: char,
//     operator: char, // ^ or nothing
//     exponent: int,
// }
// struct Lside {
//     Lside: Vec<Expression>,
//     size: i32,
//     operator: char, // *, -, *,
// }
fn main() {
    let equation = Equation::new(String::from("2x^2 + 3y^2 = 15"));
    equation.process();
    println!("Equation: {:?}", equation);
    println!("is in a function {}", equation.isfunc);
}
