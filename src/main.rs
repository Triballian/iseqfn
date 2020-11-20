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
        let mut t = String::from("");
        let mut o = String::from("");
        // let mut vt: Vec<Term> = vec![None];
        //let mut vt: Option<Vec<Term>> = Option::as_ref(&None);
        let mut vt: Vec<Term> = vec![];
        //let mut vt: Vec<Term>;
        // let mut vt: Option<> = None;
        let mut i: usize = 0;
        for c in eqtion.chars() {
            t.push(c);
            i += 1;
            // for c in eqtion{
            //let nexti = i + 1;
            //if t::as_byte()[nexti] != ('+' | '-' | '*' | '=') {
            // if c.as_byte() != ('+' | '-' | '*' | '=') {
            if (c == '+') || (c == '-') || (c == '*') {
                o.push(c);
                let mut tt = Term::new();
                println!("t indexed is :{:?}", t[0..i -1].to_string().clone());
                    // tt.process(t.clone());
                tt.process(t[0..i - 1].to_string().clone());
                vt.push(tt);
                t = String::from("").clone();
                i = 0;
                continue              
            }
            if c == '=' {
                let mut tt = Term::new();
                println!("t indexed is :{:?}", t[0..i - 1].to_string().clone());
                tt.process(t[0..i - 1].to_string().clone());
                vt.push(tt);
                t = String::from("").clone();
                i = 0;
                continue
            }
            if c == eqtion.chars().last().unwrap() {
                
                let mut tt = Term::new();
                println!("t is :{:?}", t[0..i - 1].to_string().clone());
                tt.process(t[0..i - 1].to_string().clone());
                vt.push(tt);
                    // t = String::from("").clone();
            }

                // if !vt.is_some() {
                // if vt.as_ref() == None {
                // if vt.is_none() {
                // if None = vt {
                // if !vt.is_some() {
                // if let Some(vct) = vt {

                //   else {
                //     println!("t: {}", t);
                //     let mut tt = Term::new();
                //     tt.process(t.clone());
                //     vt.push(tt);
                // }
                
                // if let Some(v) = vt {
                //     let mut vect = v;
                //     vect.push(Term::new(t));
                // } else {
                //     let vt = Some(Term::new(t));
                //     println! {"vt.0 is {:?}", vt.unwrap()};
                // }
                
                // if c == eqtion.chars().last().unwrap() {
                    

                // }
                
            
        }

        // let mvt = move || {vt.unwrap()};
        Self {
            equation: eqtion,
            isfunc: true,
            terms: vt,
            opps: o,
        }
    }
    fn afunc(&mut self) {
        for ct in &self.terms {
        //    println!("this variable: {}, this exponent {}", ct.variable.unwrap(), ct.exponent.unwrap()); 
            // println!("&self: {:?}", self); 
            if !ct.variable.is_none(){   
                if (ct.variable.unwrap() == 'y') && (ct.exponent.unwrap() % 2 == 0)  {
            //    if ct.exponent.unwrap() % 2 == 0 {
                   self.isfunc = false;
            //    }
                }
            }
        }
        
    }
    // fn process(&self) {
    //     for t in self.terms {

    //     }

    //     println!("isfunc {}", self.isfunc);
    // }
}
#[derive(Debug,Clone)]
struct Term {
    // exponent: Option<Option<i32>>, //use options here
    coefficient: Option<i32>,
    variable: Option<char>, // x or y or n, y , x or nuymber type
    exponent: Option<i32>,
    
}
impl Term {
    fn new() -> Self {
        Self {
            coefficient: None,
            variable: None,
            exponent: None,
            
            // coefficient: String::from(""). 
        }
    }
    fn process(&mut self, t: String) {
        let buff = &mut String::from("");
        let mut coeff: Option<i32>  = None;
        let mut vr = None;
        // let cexp: String = String::from("");
        let mut exp: String = String::from("");
        // let mut gtvar = false;
        let mut i: usize = 0;
        for c in t.chars() {
            &buff.push(c);
                if (c == 'x') || (c == 'y') {
                vr = Some(c);
                // gtvar = true;
                if buff == "" {
                    coeff = None;
                } else {
                    let scoeff: String = buff[0..i].to_string();
                    coeff = Some(scoeff.parse::<i32>().unwrap());                    
                     
                }
                i = i+1;
                // *buff = "".to_string();
                *buff = String::from("");
                continue;
   
            }
            if c == '^' {
                //*buff = "".to_string();
                *buff = String::from("");
            }
            // exp would not neccisarily be the last character
            if c == t.chars().last().unwrap() {
                println!("buff: {}", buff);
                exp = buff.clone() ;

            }
           // i = i + 1;
           i += 1;
            
        }
        if exp != "" {
            println!("exp: {}", exp);
            self.exponent = Some(exp.parse::<i32>().unwrap());
        }
        
        self.variable = vr;
        // self.coefficient = coeff.clone();
        if coeff.is_none() {
            self.coefficient = None;
        } else {
        self.coefficient = coeff;
        // coeff = Some(scoeff.parse::<i32>().unwrap());
        }
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
    // let mut equation = Equation::new(String::from("2x^2 + 3y^2 = 15"));
    let mut equation = Equation::new(String::from("5x^8 + 3y^2 = 15"));
    equation.afunc();
    // equation.process();
    // println!("Equation: {:?}", equation);
    println!(
        "is it a function: {}, terms: {:?}",
        equation.isfunc, equation.terms
    );
}

// if gtvar == false {
//     match c {
//         'x' => {
//             vr = c;
//             gtvar = true;
//             // buff == String::from("");
//             // continue;                     
//         },
//         'y' => {
//             vr = c;
//             gtvar = true;
//             // buff == String::from("");
//             // continue;
//         },
//     }
// }
