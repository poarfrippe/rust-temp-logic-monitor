use std::cell::{RefCell};
use monitoring_macro_fn::{monitor, monitor2, monitor_incr};
use monitoring::{TracedBool, Snapshotter};
use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
fn single_var() {
    //Setup
    let x = TracedBool::new(true);
    let s = Snapshotter::new(vec![&x]);
    x.add_snapshotter(&s);

    println!("Globally");
    let formula = monitor2!(Glob(x));

    println!("\nafter init with true");
    assert_eq!(formula(), Ok(()));

    x.not();
    println!("\nafter not");
    assert_eq!(formula(), Err("formula violated"));

    x.assign_true();
    println!("\nafter reassign true");
    assert_eq!(formula(), Err("formula violated"));

    x.assign_true();
    println!("\nafter reassign true");
    assert_eq!(formula(), Err("formula violated"));
}

#[test]
fn example() {
    let licht_an = TracedBool::new(true);
    let tuer_offen = TracedBool::new(true);

    let s = Snapshotter::new(vec![&licht_an, &tuer_offen]);
    tuer_offen.add_snapshotter(&s);
    licht_an.add_snapshotter(&s);


    // wenn tuer zu, dann muss davor licht ausgeschalten sein.
    let tester = monitor!(Implies(Not(tuer_offen), Prev(Not(licht_an))));

    println!("sagt davor:");
    tester();

    licht_an.assign_false();
    tuer_offen.assign_false();
    println!();
    println!("sagt:");
    tester();

    licht_an.assign_true();
    println!();
    println!("sagt jetzt:");
    // true weil in previous war licht noch false
    tester();

    licht_an.assign_true();
    println!();
    println!("sagt jetzt:");
    // jetzt false, weil previouse war licht an
    tester();
}

#[test]
fn test_propositional() {

    let x = TracedBool::new(false);
    let y = TracedBool::new(true);
    let s = Snapshotter::new(vec![&x, &y]);
    x.add_snapshotter(&s);
    y.add_snapshotter(&s);

    let x_and_y = monitor2!(And(x,y));
    let x_or_y = monitor2!(Or(x,y));
    let x_implies_y = monitor2!(Implies(x,y));
    let not_x = monitor2!(Not(x));

    assert_eq!(x_and_y(), Err("formula violated"));
    assert_eq!(x_or_y(), Ok(()));
    assert_eq!(x_implies_y(), Ok(()));
    assert_eq!(not_x(), Ok(()));

    x.assign_true();

    assert_eq!(x_and_y(), Ok(()));
    assert_eq!(x_or_y(), Ok(()));
    assert_eq!(x_implies_y(), Ok(()));
    assert_eq!(not_x(), Err("formula violated"));

    y.assign_false();

    assert_eq!(x_and_y(), Err("formula violated"));
    assert_eq!(x_or_y(), Ok(()));
    assert_eq!(x_implies_y(), Err("formula violated"));
    assert_eq!(not_x(), Err("formula violated"));

    x.assign_false();

    assert_eq!(x_and_y(), Err("formula violated"));
    assert_eq!(x_or_y(), Err("formula violated"));
    assert_eq!(x_implies_y(), Ok(()));
    assert_eq!(not_x(), Ok(()));

}

#[test]
fn test_since() {

}

#[test]
fn test_static() {

    let x = TracedBool::new(true);
    let s = Snapshotter::new(vec![&x]);
    x.add_snapshotter(&s);


    //Glob(x)
    let formula =
        || -> Result<(), &str>
        {
            static START_AT: AtomicUsize = AtomicUsize::new(1);

            //would need unsafe block on access, if multi threaded.
            //expands to some unsafe code...?
            thread_local! {
                static PRE: RefCell<Vec<bool>> = RefCell::new(vec![false; 2]);
            }

            //load from statics
            let start_at = START_AT.load(Ordering::Relaxed);
            let mut pre = PRE.with(|v| v.borrow_mut().to_vec());

            let mut now = vec![false; 2];

            let tracelength = x.get_trace().len();

            //only in first call. after pre should be optained from static PRE
            if start_at == 1 {
                pre[1] = x.get_trace()[0];
                pre[0] = pre[1];
            }
            for index in start_at..tracelength {
                now[1] = x.get_trace()[index];
                now[0] = now[1] && pre[0];
                pre = now.clone();
            }

            //write back to statics
            START_AT.store(tracelength, Ordering::Relaxed);
            PRE.with(|v| v.replace(pre.clone()));

            //chech pre instead of now to avoid missing violation in traces with lenght=1. on traces >1 now is coppied into pre anyways.
            if pre[0] == false { Err("formula violated") } else { Ok(()) }
        };


    s.snapshot();
    s.snapshot();

    println!("\nafter init with true");
    assert_eq!(formula(), Ok(()));

    s.snapshot();
    s.snapshot();
    println!("\nsome more snapshots. still true");
    assert_eq!(formula(), Ok(()));

    x.not();
    println!("\nafter not");
    assert_eq!(formula(), Err("formula violated"));

    x.assign_true();
    println!("\nafter reassign true");
    assert_eq!(formula(), Err("formula violated"));

    x.assign_true();
    println!("\nafter reassign true");
    assert_eq!(formula(), Err("formula violated"));
}

#[test]
fn test_incremental(){
    let x = TracedBool::new(true);
    let s = Snapshotter::new(vec![&x]);
    x.add_snapshotter(&s);

    let monitor_incr = monitor_incr!(Glob(x));

    for _ in 1..10000 {
        s.snapshot();
    }

    println!("\nafter init with true");
    assert_eq!(monitor_incr(), Ok(()));

    for _ in 1..10000 {
        s.snapshot();
    }
    println!("\nsome more snapshots. still true");
    assert_eq!(monitor_incr(), Ok(()));

    x.not();
    println!("\nafter not");
    assert_eq!(monitor_incr(), Err("formula violated"));

    x.assign_true();
    println!("\nafter reassign true");
    assert_eq!(monitor_incr(), Err("formula violated"));

    x.assign_true();
    println!("\nafter reassign true");
    assert_eq!(monitor_incr(), Err("formula violated"));

    println!("\nafter reassign true");
    assert_eq!(monitor_incr(), Err("formula violated"));

    println!("\nafter reassign true");
    assert_eq!(monitor_incr(), Err("formula violated"));

    println!("\nafter reassign true");
    assert_eq!(monitor_incr(), Err("formula violated"));

    println!("\nafter reassign true");
    assert_eq!(monitor_incr(), Err("formula violated"));

    println!("\nafter reassign true");
    assert_eq!(monitor_incr(), Err("formula violated"));

    println!("\nafter reassign true");
    assert_eq!(monitor_incr(), Err("formula violated"));

}

#[test]
fn test_incremental_not(){
    let x = TracedBool::new(true);
    let s = Snapshotter::new(vec![&x]);
    x.add_snapshotter(&s);

    let monitor_incr = monitor2!(Glob(x));

    for _ in 1..10000 {
        s.snapshot();
    }

    println!("\nafter init with true");
    assert_eq!(monitor_incr(), Ok(()));

    for _ in 1..10000 {
        s.snapshot();
    }
    println!("\nsome more snapshots. still true");
    assert_eq!(monitor_incr(), Ok(()));

    x.not();
    println!("\nafter not");
    assert_eq!(monitor_incr(), Err("formula violated"));

    x.assign_true();
    println!("\nafter reassign true");
    assert_eq!(monitor_incr(), Err("formula violated"));

    x.assign_true();
    println!("\nafter reassign true");
    assert_eq!(monitor_incr(), Err("formula violated"));

    println!("\nafter reassign true");
    assert_eq!(monitor_incr(), Err("formula violated"));

    println!("\nafter reassign true");
    assert_eq!(monitor_incr(), Err("formula violated"));

    println!("\nafter reassign true");
    assert_eq!(monitor_incr(), Err("formula violated"));

    println!("\nafter reassign true");
    assert_eq!(monitor_incr(), Err("formula violated"));

    println!("\nafter reassign true");
    assert_eq!(monitor_incr(), Err("formula violated"));

    println!("\nafter reassign true");
    assert_eq!(monitor_incr(), Err("formula violated"));

}