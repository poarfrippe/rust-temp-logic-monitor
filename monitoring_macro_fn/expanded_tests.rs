#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::cell::RefCell;
use monitoring_macro_fn::{monitor, monitor2, monitor_incr};
use monitoring::{TracedBool, Snapshotter};
use std::sync::atomic::{AtomicUsize, Ordering};

extern crate test;
#[cfg(test)]
#[rustc_test_marker = "single_var"]
pub const single_var: test::TestDescAndFn =
    test::TestDescAndFn {
        //Setup





        desc: test::TestDesc {
            name: test::StaticTestName("single_var"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration_test.rs",
            start_line: 7usize,
            start_col: 4usize,
            end_line: 7usize,
            end_col: 14usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] ||
                test::assert_test_result(single_var())),
    };
fn single_var() {
    let x = TracedBool::new(true);
    let s =
        Snapshotter::new(




            // wenn tuer zu, dann muss davor licht ausgeschalten sein.



            // true weil in previous war licht noch false

            // jetzt false, weil previouse war licht an

















            //Glob(x)

            //would need unsafe block on access, if multi threaded.
            //expands to some unsafe code...?

            //load from statics



            //only in first call. after pre should be optained from static PRE

            //write back to statics

            //chech pre instead of now to avoid missing violation in traces with lenght=1. on traces >1 now is coppied into pre anyways.
















            <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([&x])));
    x.add_snapshotter(&s);
    { ::std::io::_print(format_args!("Globally\n")); };
    let formula =
        || -> Result<(), &str>
            {
                let tracelength = x.get_trace().len();
                let mut pre = ::alloc::vec::from_elem(false, 2);
                let mut now = ::alloc::vec::from_elem(false, 2);
                pre[1] = x.get_trace()[0];
                pre[0] = pre[1];
                for index in 1..tracelength {
                    now[1] = x.get_trace()[index];
                    now[0] = now[1] && pre[0];
                    pre = now.clone();
                }
                if pre[0] == false { Err("formula violated") } else { Ok(()) }
            };
    { ::std::io::_print(format_args!("\nafter init with true\n")); };
    match (&formula(), &Ok(())) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    x.not();
    { ::std::io::_print(format_args!("\nafter not\n")); };
    match (&formula(), &Err("formula violated")) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    x.assign_true();
    { ::std::io::_print(format_args!("\nafter reassign true\n")); };
    match (&formula(), &Err("formula violated")) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    x.assign_true();
    { ::std::io::_print(format_args!("\nafter reassign true\n")); };
    match (&formula(), &Err("formula violated")) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "example"]
pub const example: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("example"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration_test.rs",
            start_line: 33usize,
            start_col: 4usize,
            end_line: 33usize,
            end_col: 11usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] ||
                test::assert_test_result(example())),
    };
fn example() {
    let licht_an = TracedBool::new(true);
    let tuer_offen = TracedBool::new(true);
    let s =
        Snapshotter::new(<[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([&licht_an,
                            &tuer_offen])));
    tuer_offen.add_snapshotter(&s);
    licht_an.add_snapshotter(&s);
    let tester =
        ||
            {
                let tracelength = licht_an.get_trace().len();
                let mut pre = ::alloc::vec::from_elem(false, 6);
                let mut now = ::alloc::vec::from_elem(false, 6);
                pre[5] = licht_an.get_trace()[0];
                pre[4] = !pre[5];
                pre[3] = pre[4];
                pre[2] = tuer_offen.get_trace()[0];
                pre[1] = !pre[2];
                pre[0] = !pre[1] || pre[3];
                for index in 1..tracelength {
                    now[5] = licht_an.get_trace()[index];
                    now[4] = !now[5];
                    now[3] = pre[4];
                    now[2] = tuer_offen.get_trace()[index];
                    now[1] = !now[2];
                    now[0] = !now[1] || now[3];
                    if now[0] == false {
                            {
                                ::std::io::_print(format_args!("propertiy violated\n"));
                            };
                        }
                    pre = now.clone();
                }
            };
    { ::std::io::_print(format_args!("sagt davor:\n")); };
    tester();
    licht_an.assign_false();
    tuer_offen.assign_false();
    { ::std::io::_print(format_args!("\n")); };
    { ::std::io::_print(format_args!("sagt:\n")); };
    tester();
    licht_an.assign_true();
    { ::std::io::_print(format_args!("\n")); };
    { ::std::io::_print(format_args!("sagt jetzt:\n")); };
    tester();
    licht_an.assign_true();
    { ::std::io::_print(format_args!("\n")); };
    { ::std::io::_print(format_args!("sagt jetzt:\n")); };
    tester();
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_propositional"]
pub const test_propositional: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test_propositional"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration_test.rs",
            start_line: 68usize,
            start_col: 4usize,
            end_line: 68usize,
            end_col: 22usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] ||
                test::assert_test_result(test_propositional())),
    };
fn test_propositional() {
    let x = TracedBool::new(false);
    let y = TracedBool::new(true);
    let s =
        Snapshotter::new(<[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([&x,
                            &y])));
    x.add_snapshotter(&s);
    y.add_snapshotter(&s);
    let x_and_y =
        || -> Result<(), &str>
            {
                let tracelength = y.get_trace().len();
                let mut pre = ::alloc::vec::from_elem(false, 3);
                let mut now = ::alloc::vec::from_elem(false, 3);
                pre[2] = y.get_trace()[0];
                pre[1] = x.get_trace()[0];
                pre[0] = pre[1] && pre[2];
                for index in 1..tracelength {
                    now[2] = y.get_trace()[index];
                    now[1] = x.get_trace()[index];
                    now[0] = now[1] && now[2];
                    pre = now.clone();
                }
                if pre[0] == false { Err("formula violated") } else { Ok(()) }
            };
    let x_or_y =
        || -> Result<(), &str>
            {
                let tracelength = y.get_trace().len();
                let mut pre = ::alloc::vec::from_elem(false, 3);
                let mut now = ::alloc::vec::from_elem(false, 3);
                pre[2] = y.get_trace()[0];
                pre[1] = x.get_trace()[0];
                pre[0] = pre[1] || pre[2];
                for index in 1..tracelength {
                    now[2] = y.get_trace()[index];
                    now[1] = x.get_trace()[index];
                    now[0] = now[1] || now[2];
                    pre = now.clone();
                }
                if pre[0] == false { Err("formula violated") } else { Ok(()) }
            };
    let x_implies_y =
        || -> Result<(), &str>
            {
                let tracelength = y.get_trace().len();
                let mut pre = ::alloc::vec::from_elem(false, 3);
                let mut now = ::alloc::vec::from_elem(false, 3);
                pre[2] = y.get_trace()[0];
                pre[1] = x.get_trace()[0];
                pre[0] = !pre[1] || pre[2];
                for index in 1..tracelength {
                    now[2] = y.get_trace()[index];
                    now[1] = x.get_trace()[index];
                    now[0] = !now[1] || now[2];
                    pre = now.clone();
                }
                if pre[0] == false { Err("formula violated") } else { Ok(()) }
            };
    let not_x =
        || -> Result<(), &str>
            {
                let tracelength = x.get_trace().len();
                let mut pre = ::alloc::vec::from_elem(false, 2);
                let mut now = ::alloc::vec::from_elem(false, 2);
                pre[1] = x.get_trace()[0];
                pre[0] = !pre[1];
                for index in 1..tracelength {
                    now[1] = x.get_trace()[index];
                    now[0] = !now[1];
                    pre = now.clone();
                }
                if pre[0] == false { Err("formula violated") } else { Ok(()) }
            };
    match (&x_and_y(), &Err("formula violated")) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    match (&x_or_y(), &Ok(())) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    match (&x_implies_y(), &Ok(())) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    match (&not_x(), &Ok(())) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    x.assign_true();
    match (&x_and_y(), &Ok(())) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    match (&x_or_y(), &Ok(())) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    match (&x_implies_y(), &Ok(())) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    match (&not_x(), &Err("formula violated")) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    y.assign_false();
    match (&x_and_y(), &Err("formula violated")) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    match (&x_or_y(), &Ok(())) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    match (&x_implies_y(), &Err("formula violated")) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    match (&not_x(), &Err("formula violated")) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    x.assign_false();
    match (&x_and_y(), &Err("formula violated")) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    match (&x_or_y(), &Err("formula violated")) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    match (&x_implies_y(), &Ok(())) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    match (&not_x(), &Ok(())) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_since"]
pub const test_since: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test_since"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration_test.rs",
            start_line: 110usize,
            start_col: 4usize,
            end_line: 110usize,
            end_col: 14usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] ||
                test::assert_test_result(test_since())),
    };
fn test_since() {}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_static"]
pub const test_static: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test_static"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration_test.rs",
            start_line: 115usize,
            start_col: 4usize,
            end_line: 115usize,
            end_col: 15usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] ||
                test::assert_test_result(test_static())),
    };
fn test_static() {
    let x = TracedBool::new(true);
    let s =
        Snapshotter::new(<[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([&x])));
    x.add_snapshotter(&s);
    let formula =
        || -> Result<(), &str>
            {
                static START_AT: AtomicUsize = AtomicUsize::new(1);
                const PRE: ::std::thread::LocalKey<RefCell<Vec<bool>>> =
                    {
                        #[inline]
                        fn __init() -> RefCell<Vec<bool>> {
                            RefCell::new(::alloc::vec::from_elem(false, 2))
                        }
                        unsafe {
                            use ::std::mem::needs_drop;
                            use ::std::thread::LocalKey;
                            use ::std::thread::local_impl::LazyStorage;
                            LocalKey::new(const {
                                        if needs_drop::<RefCell<Vec<bool>>>() {
                                                |init|
                                                    {
                                                        #[thread_local]
                                                        static VAL: LazyStorage<RefCell<Vec<bool>>, ()> =
                                                            LazyStorage::new();
                                                        VAL.get_or_init(init, __init)
                                                    }
                                            } else {
                                               |init|
                                                   {
                                                       #[thread_local]
                                                       static VAL: LazyStorage<RefCell<Vec<bool>>, !> =
                                                           LazyStorage::new();
                                                       VAL.get_or_init(init, __init)
                                                   }
                                           }
                                    })
                        }
                    };
                ;
                let start_at = START_AT.load(Ordering::Relaxed);
                let mut pre = PRE.with(|v| v.borrow_mut().to_vec());
                let mut now = ::alloc::vec::from_elem(false, 2);
                let tracelength = x.get_trace().len();
                if start_at == 1 {
                        pre[1] = x.get_trace()[0];
                        pre[0] = pre[1];
                    }
                for index in start_at..tracelength {
                    now[1] = x.get_trace()[index];
                    now[0] = now[1] && pre[0];
                    pre = now.clone();
                }
                START_AT.store(tracelength, Ordering::Relaxed);
                PRE.with(|v| v.replace(pre.clone()));
                if pre[0] == false { Err("formula violated") } else { Ok(()) }
            };
    s.snapshot();
    s.snapshot();
    { ::std::io::_print(format_args!("\nafter init with true\n")); };
    match (&formula(), &Ok(())) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    s.snapshot();
    s.snapshot();
    {
        ::std::io::_print(format_args!("\nsome more snapshots. still true\n"));
    };
    match (&formula(), &Ok(())) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    x.not();
    { ::std::io::_print(format_args!("\nafter not\n")); };
    match (&formula(), &Err("formula violated")) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    x.assign_true();
    { ::std::io::_print(format_args!("\nafter reassign true\n")); };
    match (&formula(), &Err("formula violated")) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    x.assign_true();
    { ::std::io::_print(format_args!("\nafter reassign true\n")); };
    match (&formula(), &Err("formula violated")) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_incremental"]
pub const test_incremental: test::TestDescAndFn =
    test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test_incremental"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/integration_test.rs",
            start_line: 187usize,
            start_col: 4usize,
            end_line: 187usize,
            end_col: 20usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] ||
                test::assert_test_result(test_incremental())),
    };
fn test_incremental() {
    let x = TracedBool::new(true);
    let s =
        Snapshotter::new(<[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([&x])));
    x.add_snapshotter(&s);
    let monitor_incr =
        || -> Result<(), &str>
            {
                static START_AT: AtomicUsize = AtomicUsize::new(1);
                const PRE: ::std::thread::LocalKey<RefCell<Vec<bool>>> =
                    {
                        #[inline]
                        fn __init() -> RefCell<Vec<bool>> {
                            RefCell::new(::alloc::vec::from_elem(false, 2))
                        }
                        unsafe {
                            use ::std::mem::needs_drop;
                            use ::std::thread::LocalKey;
                            use ::std::thread::local_impl::LazyStorage;
                            LocalKey::new(const {
                                        if needs_drop::<RefCell<Vec<bool>>>() {
                                                |init|
                                                    {
                                                        #[thread_local]
                                                        static VAL: LazyStorage<RefCell<Vec<bool>>, ()> =
                                                            LazyStorage::new();
                                                        VAL.get_or_init(init, __init)
                                                    }
                                            } else {
                                               |init|
                                                   {
                                                       #[thread_local]
                                                       static VAL: LazyStorage<RefCell<Vec<bool>>, !> =
                                                           LazyStorage::new();
                                                       VAL.get_or_init(init, __init)
                                                   }
                                           }
                                    })
                        }
                    };
                ;
                let start_at = START_AT.load(Ordering::Relaxed);
                let mut pre = PRE.with(|v| v.borrow_mut().to_vec());
                let tracelength = x.get_trace().len();
                let mut now = ::alloc::vec::from_elem(false, 2);
                if start_at == 1 {
                        pre[1] = x.get_trace()[0];
                        pre[0] = pre[1];
                    }
                for index in start_at..tracelength {
                    now[1] = x.get_trace()[index];
                    now[0] = now[1] && pre[0];
                    pre = now.clone();
                }
                START_AT.store(tracelength, Ordering::Relaxed);
                PRE.with(|v| v.replace(pre.clone()));
                if pre[0] == false { Err("formula violated") } else { Ok(()) }
            };
    s.snapshot();
    s.snapshot();
    { ::std::io::_print(format_args!("\nafter init with true\n")); };
    match (&monitor_incr(), &Ok(())) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    s.snapshot();
    s.snapshot();
    {
        ::std::io::_print(format_args!("\nsome more snapshots. still true\n"));
    };
    match (&monitor_incr(), &Ok(())) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    x.not();
    { ::std::io::_print(format_args!("\nafter not\n")); };
    match (&monitor_incr(), &Err("formula violated")) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    x.assign_true();
    { ::std::io::_print(format_args!("\nafter reassign true\n")); };
    match (&monitor_incr(), &Err("formula violated")) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
    x.assign_true();
    { ::std::io::_print(format_args!("\nafter reassign true\n")); };
    match (&monitor_incr(), &Err("formula violated")) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
}
#[rustc_main]
#[coverage(off)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&example, &single_var, &test_incremental,
                    &test_propositional, &test_since, &test_static])
}
