//take experssion, gather exponents, if exponent 2 has an expression, not function
//types of equations
// Quadratic Equation
// Linear Equation
// Radical Equation
// Exponential Equation
// Rational Equation
//term
// #[derive(Debug)]
struct Equation {
    equation: String,
    isfunc: bool,
    terms: Vec<Term>,
    //pterms: Vec<Term>,
    opps: String,
}

impl Equation {
    fn new(mut eqtion: String) -> Self {
        eqtion.retain(|c| !c.is_whitespace());
        let mut t: String = String::from("");
        let mut o = String::from("");
        // let mut vt: Vec<Term> = vec![None];
        let mut vt: &Option<Vec<Term>> = &None;
        // let mut vt: Vec<Term>;
        // let mut vt: Option<> = None;
        for c in eqtion.chars() {
            // for c in eqtion{
            //let nexti = i + 1;
            //if t::as_byte()[nexti] != ('+' | '-' | '*' | '=') {
            // if c.as_byte() != ('+' | '-' | '*' | '=') {
            if (c != '+') && (c != '-') && (c != '*') && (c != '=') {
                t.push(c);
            } else {
                o.push(c);
                // if !vt.is_some() {
                if vt.is_none() {
                    let mut tt = Term::new();
                    println!("t is :{:?}", t);
                    tt.process(t);
                    // vt.push(tt);
                    vt = &Some(vec![tt]);
                } else {
                    let mut tt = Term::new();
                    println!("t is :{:?}", t);
                    tt.process(t);
                    vt.unwrap().push(tt);
                }
                // if let Some(v) = vt {
                //     let mut vect = v;
                //     vect.push(Term::new(t));
                // } else {
                //     let vt = Some(Term::new(t));
                //     println! {"vt.0 is {:?}", vt.unwrap()};
                // }
                t = String::from("");
            }
        }
        // let mvt = move || {vt.unwrap()};
        Self {
            equation: eqtion,
            isfunc: true,
            terms: mvt.unwrap(),
            opps: o,
        }
    }
    // fn process(&self) {
    //     for t in self.terms {

    //     }

    //     println!("isfunc {}", self.isfunc);
    // }
}
#[derive(Debug, PartialEq)]
struct Term {
    exponent: Option<Option<i32>>, //use options here
    variable: Option<char>,        // x or y or n, y , x or nuymber type
    coefficient: String,
}
impl Term {
    fn new() -> Self {
        Self {
            exponent: None,
            variable: None,
            coefficient: String::from(""),
        }
    }
    fn process(&mut self, t: String) {
        let mut coeff = String::from("");
        let mut vr = 'n';
        // let cexp: String = String::from("");
        let mut exp: Option<i32> = None;
        let mut gtvar = false;
        for c in t.chars() {
            if gtvar == true {
                exp = Some(coeff.parse::<i32>().unwrap());
                continue;
            }
            if (c != 'x') && (c != 'y') && (gtvar == false) {
                coeff.push(c);
                continue;
            }
            vr = c;
            gtvar = true;
        }

        self.exponent = Some(exp);
        self.variable = Some(vr);
        self.coefficient = coeff;
    }
}
// impl Copy for Term {}
// impl Clone for Term {
//     fn clone(&self) -> Term {
//         *self
//     }
// }

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
    // equation.process();
    // println!("Equation: {:?}", equation);
    println!(
        "is it a function: {}, terms: {:?}",
        equation.isfunc, equation.terms
    );
}
