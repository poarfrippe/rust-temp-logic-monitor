use monitoring_macro_fn::{monitor, monitor2};
use monitoring::{TracedBool, Snapshotter};

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