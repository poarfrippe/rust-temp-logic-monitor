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
    assert_eq!(formula(), Err(String::from("violation at trace index: 1")));

    x.assign_true();
    println!("\nafter reassign true");
    assert_eq!(formula(), Err(String::from("violation at trace index: 1")));

    x.assign_true();
    println!("\nafter reassign true");
    assert_eq!(formula(), Err(String::from("violation at trace index: 1")));
}