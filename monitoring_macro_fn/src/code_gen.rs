use crate::data_structures::{PtLTL, Subformulae};
use crate::{INDEX_NAME, NOW_NAME, PRE_NAME, START_AT_NAME, STATIC_PRE_NAME, STATIC_START_AT_NAME, TRACELENGHT_NAME};

pub fn gen_monitor(subformulae: Subformulae) -> String {
    let mut code = String::new();
    let subformulae = subformulae.formulae;
    let m = subformulae.len();

    if let PtLTL::Prop(some_proposition) =  &subformulae[m-1] {
        let some_proposition = some_proposition.to_string();
        //requires all traces to be of same length!!
        code.push_str(&format!("let {TRACELENGHT_NAME} = {some_proposition}.get_trace().len();\n"));
    } else {
        panic!("last subformula should always be a proposition");
    }

    code.push_str(&format!("let mut {PRE_NAME} = vec![false; {m}];\n"));
    code.push_str(&format!("let mut {NOW_NAME} = vec![false; {m}];\n"));

    //init pre
    for j in 0..=(m-1) {
        let i = (m-1)-j;
        code.push_str(&format!("{PRE_NAME}[{i}] = "));
        code.push_str(&assign_from_formula(&subformulae, NOW_NAME, PRE_NAME, i, "0", true));
        code.push_str(";\n");
    }

    //evaluation loop
    code.push_str(&format!("for {INDEX_NAME} in 1..{TRACELENGHT_NAME} {{"));
    for j in 0..=(m-1) {
        let i = (m-1)-j;
        code.push_str(&format!("{NOW_NAME}[{i}] = "));
        code.push_str(&assign_from_formula(&subformulae, NOW_NAME, PRE_NAME, i, INDEX_NAME, false));
        code.push_str(";\n");
    }

    code.push_str(&format!("if {NOW_NAME}[0] == false {{\
        println!(\"propertiy violated\");\
    }}\n"));
    code.push_str(&format!("{PRE_NAME} = {NOW_NAME}.clone();\n"));
    code.push_str("}\n");

    code
}

pub fn gen_monitor2(subformulae: Subformulae) -> String {
    let mut code = String::new();
    let subformulae = subformulae.formulae;
    let m = subformulae.len();

    if let PtLTL::Prop(some_proposition) =  &subformulae[m-1] {
        let some_proposition = some_proposition.to_string();
        //requires all traces to be of same length!!
        code.push_str(&format!("let {TRACELENGHT_NAME} = {some_proposition}.get_trace().len();\n"));
    } else {
        panic!("last subformula should always be a proposition");
    }

    code.push_str(&format!("let mut {PRE_NAME} = vec![false; {m}];\n"));
    code.push_str(&format!("let mut {NOW_NAME} = vec![false; {m}];\n"));

    //init pre
    for j in 0..=(m-1) {
        let i = (m-1)-j;
        code.push_str(&format!("{PRE_NAME}[{i}] = "));
        code.push_str(&assign_from_formula(&subformulae, NOW_NAME, PRE_NAME, i, "0", true));
        code.push_str(";\n");
    }

    //evaluation loop
    code.push_str(&format!("for {INDEX_NAME} in 1..{TRACELENGHT_NAME} {{"));
        for j in 0..=(m-1) {
            let i = (m-1)-j;
            code.push_str(&format!("{NOW_NAME}[{i}] = "));
            code.push_str(&assign_from_formula(&subformulae, NOW_NAME, PRE_NAME, i, INDEX_NAME, false));
            code.push_str(";\n");
        }
        code.push_str(&format!("{PRE_NAME} = {NOW_NAME}.clone();\n"));
    code.push_str("}\n");

    //check PRE because when not entering loop, NOW is not set, and when entering, NOW is cloned into PRE
    code.push_str(&format!("if {PRE_NAME}[0] == false {{\
        Err(\"formula violated\")\
    }}\n"));
    code.push_str("else {");
        code.push_str("Ok(())");
    code.push_str("}");

    code
}

pub fn gen_monitor_incr(subformulae: Subformulae) -> String {
    let mut code = String::new();
    let subformulae = subformulae.formulae;
    let m = subformulae.len();

    code.push_str("use std::sync::atomic::{AtomicUsize, Ordering};\n");
    code.push_str("use std::cell::RefCell;\n");

    code.push_str(&format!("static {STATIC_START_AT_NAME}: AtomicUsize = AtomicUsize::new(1);\n"));

    //would need unsafe block on access, if multi threaded.
    //expands to some unsafe code...?
    code.push_str(&format!("thread_local! {{ static {STATIC_PRE_NAME}: RefCell<Vec<bool>> = RefCell::new(vec![false; {m}]); }}\n"));

    //load from statics
    code.push_str(&format!("let {START_AT_NAME} = {STATIC_START_AT_NAME}.load(Ordering::Relaxed);\n"));
    code.push_str(&format!("let mut {PRE_NAME} = {STATIC_PRE_NAME}.with(|v| v.borrow_mut().to_vec());\n"));

    if let PtLTL::Prop(some_proposition) =  &subformulae[m-1] {
        let some_proposition = some_proposition.to_string();
        //requires all traces to be of same length!!
        code.push_str(&format!("let {TRACELENGHT_NAME} = {some_proposition}.get_trace().len();\n"));
    } else {
        panic!("last subformula should always be a proposition");
    }

    code.push_str(&format!("let mut {NOW_NAME} = vec![false; {m}];\n"));

    //only in first call. after pre should be optained from static PRE
    code.push_str(&format!("if {START_AT_NAME} == 1 {{\n"));
    //init pre
    for j in 0..=(m-1) {
            let i = (m-1)-j;
            code.push_str(&format!("{PRE_NAME}[{i}] = "));
            code.push_str(&assign_from_formula(&subformulae, NOW_NAME, PRE_NAME, i, "0", true));
            code.push_str(";\n");
        }
    code.push_str("}");

    //evaluation loop
    code.push_str(&format!("for {INDEX_NAME} in {START_AT_NAME}..{TRACELENGHT_NAME} {{"));
    for j in 0..=(m-1) {
        let i = (m-1)-j;
        code.push_str(&format!("{NOW_NAME}[{i}] = "));
        code.push_str(&assign_from_formula(&subformulae, NOW_NAME, PRE_NAME, i, INDEX_NAME, false));
        code.push_str(";\n");
    }
    code.push_str(&format!("{PRE_NAME} = {NOW_NAME}.clone();\n"));
    code.push_str("}\n");

    //write back to statics
    code.push_str(&format!("{STATIC_START_AT_NAME}.store({TRACELENGHT_NAME}, Ordering::Relaxed);\n"));
    code.push_str(&format!("{STATIC_PRE_NAME}.with(|v| v.replace({PRE_NAME}.clone()));\n"));

    //chech PRE instead of NOW to avoid missing violation in traces with lenght=1. on traces >1 NOW is coppied into PRE anyways.
    code.push_str(&format!("if {PRE_NAME}[0] == false {{\
        Err(\"formula violated\")\
    }}\n"));
    code.push_str("else {");
    code.push_str("Ok(())");
    code.push_str("}");

    code
}

//loop assignment and initial assigment almost same, but different for the past-time operations. in this case only for Since, but could be extended with other pt-operators
fn assign_from_formula(subformulae: &Vec<PtLTL>, now_name: &str, pre_name: &str, formula_index: usize, index_name:&str, is_init: bool) -> String {
    let to_assign_name;
    if is_init {
        to_assign_name = pre_name;
    } else {
        to_assign_name = now_name;
    }

    let mut code = String::new();

    let formula = &subformulae[formula_index];
    match formula {
        PtLTL::And(x, y) => {
            let x_sub_index = get_subformula_index(subformulae, x, formula_index);
            let y_sub_index = get_subformula_index(subformulae, y, formula_index);
            code.push_str(&format!("{to_assign_name}[{x_sub_index}] && {to_assign_name}[{y_sub_index}]"));
        }
        PtLTL::Or(x, y) => {
            let x_sub_index = get_subformula_index(subformulae, x, formula_index);
            let y_sub_index = get_subformula_index(subformulae, y, formula_index);
            code.push_str(&format!("{to_assign_name}[{x_sub_index}] || {to_assign_name}[{y_sub_index}]"));
        }
        PtLTL::Implies(x, y) => {
            let x_sub_index = get_subformula_index(subformulae, x, formula_index);
            let y_sub_index = get_subformula_index(subformulae, y, formula_index);
            code.push_str(&format!("!{to_assign_name}[{x_sub_index}] || {to_assign_name}[{y_sub_index}]"));
        }
        PtLTL::Equiv(x, y) => {
            let x_sub_index = get_subformula_index(subformulae, x, formula_index);
            let y_sub_index = get_subformula_index(subformulae, y, formula_index);
            code.push_str(&format!("{to_assign_name}[{x_sub_index}] == {to_assign_name}[{y_sub_index}]"));
        }
        PtLTL::Not(x) => {
            let sub_index = get_subformula_index(subformulae, x, formula_index);
            code.push_str(&format!("!{to_assign_name}[{sub_index}]"));
        },
        PtLTL::Prev(x) => {
            //just inner formula in previous state
            //same for init and loop assignment
            //because we assume state to be constant (if there is no previous: init case)
            let sub_index = get_subformula_index(subformulae, x, formula_index);
            code.push_str(&format!("{pre_name}[{sub_index}]"));
        }
        PtLTL::Since(x, y) => {
            let x_sub_index = get_subformula_index(subformulae, x, formula_index);
            let y_sub_index = get_subformula_index(subformulae, y, formula_index);
            if is_init {
                //because init and strong Since requires y to be true sometime
                code.push_str(&format!("{pre_name}[{y_sub_index}]"));
            } else {
                //either y holds or if not, in order to make since true, x must hold now and whole since must have been true previously.
                //this means that whenever x is not true (and y also not true), the Since gets "broken" and stops being true.
                code.push_str(&format!("{now_name}[{y_sub_index}] || ({now_name}[{x_sub_index}] && {pre_name}[{formula_index}])"));
            }
        }
        PtLTL::Once(x) => {
            let sub_index = get_subformula_index(subformulae, x, formula_index);
            if is_init {
                code.push_str(&format!("{pre_name}[{sub_index}]"));
            } else {
                code.push_str(&format!("{now_name}[{sub_index}] || {pre_name}[{formula_index}]"));
            }
        }
        PtLTL::Glob(x) => {
            let sub_index = get_subformula_index(subformulae, x, formula_index);
            if is_init {
                code.push_str(&format!("{pre_name}[{sub_index}]"));
            } else {
                code.push_str(&format!("{now_name}[{sub_index}] && {pre_name}[{formula_index}]"));
            }
        }
        PtLTL::Prop(x) => code.push_str(&format!("{}.get_trace()[{index_name}]", x.to_string().as_str())),
    }

    code
}

fn get_subformula_index(subformulae: &Vec<PtLTL>, formula: &PtLTL, j: usize) -> usize {
    for i in (j+1)..=(subformulae.len()-1) {
        if subformulae[i] == *formula {
            return i;
        }
    }
    panic!("subformula '{formula}' not found in allowed range of indices");
}