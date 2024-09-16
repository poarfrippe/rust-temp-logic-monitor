use syn::parse::ParseStream;
use syn::token::{Paren, Token};
use crate::data_structures::PtLTL;
use crate::{INDEX_NAME, NOW_NAME, PRE_NAME, TRACELENGHT_NAME};

pub fn parse_recursive (input: ParseStream) -> syn::Result<PtLTL> {
    let id;
    //Parsestream::parse calles the parse fuction (from Parse Trait) implemented on Ident.
    id = input.parse::<syn::Ident>()?;

    //eighter operation or proposition
    match id.to_string().as_str() {
        //check weatehr id is some var name used in closure, if so return error!
        varname
        if varname == TRACELENGHT_NAME || varname == PRE_NAME || varname == NOW_NAME || varname == INDEX_NAME => {
            return Err(syn::Error::new(
                id.span(),
                format!("Illegal name '{varname}'. Conflicts with formula evaluation code.")
            ));
        },
        "And" => {
            let (lhs, rhs) = parse_binary_op(input)?;
            Ok(
                PtLTL::And(
                    Box::new(lhs),
                    Box::new(rhs)
                )
            )
        },
        "Or" => {
            let (lhs, rhs) = parse_binary_op(input)?;
            Ok(
                PtLTL::Or(
                    Box::new(lhs),
                    Box::new(rhs)
                )
            )
        },
        "Implies" => {
            let (lhs, rhs) = parse_binary_op(input)?;
            Ok(
                PtLTL::Implies(
                    Box::new(lhs),
                    Box::new(rhs)
                )
            )
        },
        "Equiv" => {
            let (lhs, rhs) = parse_binary_op(input)?;
            Ok(
                PtLTL::Equiv(
                    Box::new(lhs),
                    Box::new(rhs)
                )
            )
        },
        "Not" => {
            let inner = parse_unary_op(input)?;
            Ok(
                PtLTL::Not(Box::new(inner))
            )
        },
        "Since" => {
            let (lhs, rhs) = parse_binary_op(input)?;
            Ok(
                PtLTL::Since(
                    Box::new(lhs),
                    Box::new(rhs)
                )
            )
        },
        "Prev" => {
            let inner = parse_unary_op(input)?;
            Ok(
                PtLTL::Prev(Box::new(inner))
            )
        },
        "Once" => {
            let inner = parse_unary_op(input)?;
            Ok(
                PtLTL::Once(Box::new(inner))
            )
        },
        "Glob" => {
            let inner = parse_unary_op(input)?;
            Ok(
                PtLTL::Glob(Box::new(inner))
            )
        },
        _ => {
            if Paren::peek(input.cursor()) {
                return Err(syn::Error::new(id.span(), "Unknown Operator"));
            }
            Ok(PtLTL::Prop(id))
        }
    }
}

fn parse_binary_op (input: ParseStream) -> syn::Result<(PtLTL, PtLTL)> {
    //expect parenthesis after operator
    let content;
    syn::parenthesized!(content in input);

    let lhs = parse_recursive(&content)?;
    (&content).parse::<syn::Token![,]>()?;
    let rhs = parse_recursive(&content)?;

    Ok((lhs, rhs))
}

fn parse_unary_op (input: ParseStream) -> syn::Result<PtLTL> {
    let content;
    syn::parenthesized!(content in input);
    let inner = parse_recursive(&content)?;
    Ok(inner)
}