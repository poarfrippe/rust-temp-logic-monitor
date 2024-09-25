mod data_structures;
mod code_gen;

use crate::code_gen::{gen_monitor, gen_monitor2, gen_monitor_incr};
use crate::data_structures::{Subformulae};
use quote::quote;
use syn;
// einfach nur alias für proc_macro::TokenStream, aber so macht rustrover keine probleme ahnscheinend
use syn::__private::TokenStream;
// use proc_macro::TokenStream;

static TRACELENGHT_NAME: &str = "tracelength";
static PRE_NAME: &str = "pre";
static STATIC_PRE_NAME: &str = "PRE";
static NOW_NAME: &str = "now";
static INDEX_NAME: &str = "index";
static STATIC_START_AT_NAME: &str = "START_AT";
static START_AT_NAME: &str = "start_at";



#[proc_macro]
/// return a closure, that evaluates the ptLTL formula.
pub fn monitor(input: TokenStream) -> TokenStream {
    let subformulae = syn::parse_macro_input!(input as Subformulae);

    impl_monitor_macro(subformulae)
}

#[proc_macro]
/// return a closure, that evaluates the ptLTL formula. Closure reurns a Result<(). &str>.
pub fn monitor2(input: TokenStream) -> TokenStream {
    let subformulae = syn::parse_macro_input!(input as Subformulae);

    impl_monitor_macro2(subformulae)
}

#[proc_macro]
/// return a closure, that evaluates the ptLTL formula. Closure reurns a Result<(). &str>. <br>
/// Monitor remembers where it left of last time, so only newly added traces are analysed on consecutive calls. <br>
/// Reduces runntime overhead on consecutive monitor calls.
pub fn monitor_incr(input: TokenStream) -> TokenStream {
    let subformulae = syn::parse_macro_input!(input as Subformulae);

    impl_monitor_macro_incr(subformulae)
}

fn impl_monitor_macro(subformulae: Subformulae) -> TokenStream {
    let s = gen_monitor(subformulae);
    //into block
    let expr_block = format!("{{{}}}",s);
    // let ret = quote! {|| #expr_block};
    let parsed: syn::Expr = syn::parse_str(expr_block.as_str()).expect("invalide formula evaluation code");
    let ret = quote! {|| #parsed};
    ret.into()
}

fn impl_monitor_macro2(subformulae: Subformulae) -> TokenStream {
    let s = gen_monitor2(subformulae);
    //into block
    let expr_block = format!("{{{}}}",s);
    // let ret = quote! {|| #expr_block};
    let parsed: syn::Expr = syn::parse_str(expr_block.as_str()).expect("invalide formula evaluation code");
    let ret = quote! {|| -> Result<(), &str> #parsed};
    ret.into()
}

fn impl_monitor_macro_incr(subformulae: Subformulae) -> TokenStream {
    let s = gen_monitor_incr(subformulae);
    //into block
    let expr_block = format!("{{{}}}",s);
    // let ret = quote! {|| #expr_block};
    let parsed: syn::Expr = syn::parse_str(expr_block.as_str()).expect("invalide formula evaluation code");
    let ret = quote! {|| -> Result<(), &str> #parsed};
    ret.into()
}