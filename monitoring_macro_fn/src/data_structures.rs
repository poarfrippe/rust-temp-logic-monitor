mod parse;

use crate::data_structures::parse::parse_recursive;
use std::fmt::Display;
use syn::parse::{Parse, ParseStream};


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PtLTL {
    And(Box<PtLTL>, Box<PtLTL>),
    Or(Box<PtLTL>, Box<PtLTL>),
    Implies(Box<PtLTL>, Box<PtLTL>),
    Equiv(Box<PtLTL>, Box<PtLTL>),
    Not(Box<PtLTL>),
    Prev(Box<PtLTL>),
    Since(Box<PtLTL>, Box<PtLTL>),
    Once(Box<PtLTL>),
    Glob(Box<PtLTL>),
    Prop(syn::Ident)
}

#[derive(Debug)]
pub struct Subformulae {
    pub formulae: Vec<PtLTL>
}

impl Display for PtLTL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            PtLTL::And(x, y) => {
                let mut s = String::from("And(");
                s.push_str(&*x.to_string());
                s.push_str(",");
                s.push_str(&*y.to_string());
                s.push_str(")");
                s
            },
            PtLTL::Or(x, y) => {
                let mut s = String::from("Or(");
                s.push_str(&*x.to_string());
                s.push_str(",");
                s.push_str(&*y.to_string());
                s.push_str(")");
                s
            },
            PtLTL::Implies(x, y) => {
                let mut s = String::from("Implies(");
                s.push_str(&*x.to_string());
                s.push_str(",");
                s.push_str(&*y.to_string());
                s.push_str(")");
                s
            },
            PtLTL::Equiv(x, y) => {
                let mut s = String::from("Equiv(");
                s.push_str(&*x.to_string());
                s.push_str(",");
                s.push_str(&*y.to_string());
                s.push_str(")");
                s
            },
            PtLTL::Not(x) => {
                let mut s = String::from("Not(");
                s.push_str(&*x.to_string());
                s.push_str(")");
                s
            },
            PtLTL::Prev(x) => {
                let mut s = String::from("Prev(");
                s.push_str(&*x.to_string());
                s.push_str(")");
                s
            },
            PtLTL::Since(x, y) => {
                let mut s = String::from("Since(");
                s.push_str(&*x.to_string());
                s.push_str(",");
                s.push_str(&*y.to_string());
                s.push_str(")");
                s
            },
            PtLTL::Once(x) => {
                let mut s = String::from("Once(");
                s.push_str(&*x.to_string());
                s.push_str(")");
                s
            },
            PtLTL::Glob(x) => {
                let mut s = String::from("Glob(");
                s.push_str(&*x.to_string());
                s.push_str(")");
                s
            },
            PtLTL::Prop(x) => String::from(&(*x).to_string()),
        };
        write!(f, "{}", str)
    }
}



impl Parse for Subformulae {
    fn parse(input: ParseStream) -> syn::Result<Self> {

        let main_formula = parse_recursive(&input)?;

        Ok(Subformulae::get_enumeration(main_formula))
    }
}

impl Subformulae {
    pub fn get_enumeration (formula: PtLTL) -> Self {
        let subformulae = Self::get_subfurmulae(formula);

        Subformulae{formulae: subformulae}
    }

    fn get_subfurmulae (formula: PtLTL) -> Vec<PtLTL> {
        let mut sub: Vec<PtLTL> = Vec::new();
        match formula.clone() {
            PtLTL::And(x, y) => {
                sub.push(formula);
                sub.append(&mut Self::get_subfurmulae(*x));
                sub.append(&mut Self::get_subfurmulae(*y));
            }
            PtLTL::Or(x, y) => {
                sub.push(formula);
                sub.append(&mut Self::get_subfurmulae(*x));
                sub.append(&mut Self::get_subfurmulae(*y));
            }
            PtLTL::Implies(x, y) => {
                sub.push(formula);
                sub.append(&mut Self::get_subfurmulae(*x));
                sub.append(&mut Self::get_subfurmulae(*y));
            }
            PtLTL::Equiv(x, y) => {
                sub.push(formula);
                sub.append(&mut Self::get_subfurmulae(*x));
                sub.append(&mut Self::get_subfurmulae(*y));
            }
            PtLTL::Not(x) => {
                sub.push(formula);
                sub.append(&mut Self::get_subfurmulae(*x));
            }
            PtLTL::Prev(x) => {
                sub.push(formula);
                sub.append(&mut Self::get_subfurmulae(*x));
            }
            PtLTL::Since(x, y) => {
                sub.push(formula);
                sub.append(&mut Self::get_subfurmulae(*x));
                sub.append(&mut Self::get_subfurmulae(*y));
            }
            PtLTL::Once(x) => {
                sub.push(formula);
                sub.append(&mut Self::get_subfurmulae(*x));
            }
            PtLTL::Glob(x) => {
                sub.push(formula);
                sub.append(&mut Self::get_subfurmulae(*x));
            }
            PtLTL::Prop(_x) => {
                sub.push(formula);
            }
        }
        sub
    }
}
