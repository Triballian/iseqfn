// use std::{thread, time};
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
    // abterm: String,
}

impl Equation {
    fn new(mut eqtion: String) -> Self {
        eqtion.retain(|c| !c.is_whitespace());
        let mut t = String::from("");
        let mut o = String::from("");
      
        let mut vt: Vec<Term> = vec![];
        
        let elen = eqtion.len() as usize;
        let mut i: usize = 0;
        let mut cindex: usize = 0;
        let mut myabop = false;
        let mut myabterm = String::from("");
        let mut absstart = false;
        
        for c in eqtion.chars() {
            // if (c == '|') && (eqtion.chars().last().unwrap() == '|') || absstart == True {
            if (c == '|') || (absstart == true) {
                if (absstart == true) && (c == '|'){
                    absstart = false;
                    
                    let mut tt = Term::new(myabop);
                    tt.process(myabterm.clone());
                    vt.push(tt);
                    myabop = false;
                    continue
                }
                myabop = true;
                
                
                // let tempc: String = String::from(c);

                absstart = true;
                   
                if c != '|' {
                    println!("c is : {}", c);
                    myabterm.push(c);}
                continue

            
                
                             
            }
            t.push(c);
            i += 1;
            cindex += 1;
                        
            if (c == '+') || (c == '-') || (c == '*') {
                o.push(c);
                let mut tt = Term::new(myabop);
                println!("t indexed is :{:?}", t[0..i -1].to_string().clone());
                
                tt.process(t[0..i - 1].to_string().clone());
                vt.push(tt);
                t = String::from("").clone();
                i = 0;
                println!("vt: {:?}", vt);
                
                continue              
            }
            if c == '=' {
                let mut tt = Term::new(myabop);
                println!("t indexed is :{:?}", t[0..i - 1].to_string().clone());
                tt.process(t[0..i - 1].to_string().clone());
                vt.push(tt);
                t = String::from("").clone();
                i = 0;
                
                continue

            }
           
            if cindex == elen {
               
                let mut tt = Term::new(myabop);
                println!("t is :{:?}", t[0..i].to_string().clone());
        
                tt.process(t[0..i].to_string().clone());
                vt.push(tt);
                
    
            }
           
        }

        Self {
            equation: eqtion,
            isfunc: true,
            terms: vt,
            opps: o,
            
            // abterm: myabterm,
        }
    }
    fn afunc(&mut self) {
        for ct in &self.terms {
      
            if !ct.variable.is_none() && !ct.exponent.is_none() { 
                  
                if (ct.variable.unwrap() == 'y') && (ct.exponent.unwrap() % 2 == 0) && ct.exponent.unwrap() != 0 {
    
                   self.isfunc = false;
 
                }
            }
            if !ct.variable.is_none(){
                if (ct.variable.unwrap() == 'y') && (ct.abs == true) {
                    self.isfunc = false;
                }

            }
        }
        
    }
  
}
#[derive(Debug,Clone)]
struct Term {
    
    coefficient: Option<i32>,
    variable: Option<char>, // x or y or n, y , x or nuymber type
    exponent: Option<i32>,
    abs: bool,
    
}
impl Term {
    fn new(absv: bool) -> Self {
        Self {
            coefficient: None,
            variable: None,
            exponent: None,
            abs: absv,
          
        }
    }
    fn process(&mut self, t: String) {
        let buff = &mut String::from("");
        let mut coeff: Option<i32>  = None;
        let mut vr = None;
    
        let mut exp: String = String::from("");

        let mut i: usize = 0;
        for c in t.chars() {
            &buff.push(c);
                if (c == 'x') || (c == 'y') {
                vr = Some(c);
                
                if buff == "" {
                    coeff = None;
                } else {
                    let scoeff: String = buff[0..i].to_string();
                    
                    coeff = Some(scoeff.parse::<i32>().unwrap());                    
                     
                }
                i = i+1;
                
                *buff = String::from("");
                continue;
   
            }
            if c == '^' {
                
                *buff = String::from("");
            }
            
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
       
        if coeff.is_none() {
            self.coefficient = None;
        } else {
        self.coefficient = coeff;
        
        }
    }
}

fn main() {
    
    let mut equation = Equation::new(String::from("5x^8 + |3y| = 15"));
    equation.afunc();
    
    println!(
        "is it a function: {}, terms: {:#?}",
        equation.isfunc, equation.terms
    );
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn checkforfnfalse() {
        let mut tequation = Equation::new(String::from("5x^8 + |3y| = 15"));
        tequation.afunc();
        for tterm in &tequation.terms {
            
            if !tterm.variable.is_none() {    
                // (absstart == true) && (c == '|')
                if (tterm.variable.unwrap() == 'y') && (tterm.abs == true)  {
                   
                    assert!(!tequation.isfunc);
                    
                }
            }
        }

    }
    #[test]
    fn checkforfntrue() {
        let mut tnequation = Equation::new(String::from("5x^8 + 3y2 = 15"));
        tnequation.afunc();
        for tterm in &tnequation.terms {
            
            if tterm.variable != None {    
                
                if (tterm.variable.unwrap() == 'y') && (tterm.abs == false) {
                   
                    assert!(!tnequation.isfunc);
                    
                }
            }
        }

    }

}
