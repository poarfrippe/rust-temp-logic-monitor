use std::cell::RefCell;
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign, Deref};
use std::rc::Rc;

///takes any number of RcTracedBool, creates a snapshotter with all of them and connects every RcTracedBool with the snapshotter
#[macro_export]
macro_rules! setup_snapshotter {
    ($($t:ident),+) => {
        let snapshotter = RcSnapshotter::new(vec![
            $(
                RcTracedBool::clone(&$t),
            )+
        ]);
        $(
            $t.add_snapshotter(RcSnapshotter::clone(&snapshotter));
        )+
    };
}

pub struct RcTracedBool (Rc<TracedBool>);

impl Deref for RcTracedBool {
    type Target = TracedBool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Clone for RcTracedBool {
    fn clone(&self) -> Self {
        RcTracedBool(Rc::clone(&self.0))
    }
}

impl RcTracedBool {
    pub fn new (val: bool) -> Self {
        RcTracedBool(Rc::new(TracedBool::new(val)))
    }
}


//Use of RefCell because otherwise monitor cannot hold on to variables while value changes.
pub struct TracedBool{
    val: RefCell<bool>,
    trace: RefCell<Vec<bool>>,
    snap: RefCell<Option<RcSnapshotter>>
}

impl TracedBool {
    fn new (val: bool) -> Self {
        TracedBool{
            val: RefCell::new(val),
            trace: RefCell::new(vec![val]),
            snap: RefCell::new(None)
        }
    }
    pub fn not (&self) {
        self.val.borrow_mut().bitxor_assign(true);
        if let Some(s) = self.snap.borrow().clone() {
            s.snapshot();
        }
    }
    pub fn assign_true(&self) {
        self.val.borrow_mut().bitor_assign(true);
        if let Some(s) = self.snap.borrow().clone() {
            s.snapshot();
        }
    }
    pub fn assign_false(&self) {
        self.val.borrow_mut().bitand_assign(false);
        if let Some(s) = self.snap.borrow().clone() {
            s.snapshot();
        }
    }
    pub fn get_val(&self) -> bool {
        *self.val.borrow()
    }
    pub fn get_trace(&self) -> Vec<bool> {
        self.trace.borrow().clone()
    }
    pub fn trace_current(&self) {
        self.trace.borrow_mut().push(self.get_val());
    }

    pub fn add_snapshotter(&self, s: RcSnapshotter) {
        self.snap.borrow_mut().replace(s);
    }
}

pub struct Snapshotter {
    to_snapshot: Vec<RcTracedBool>
}

impl Snapshotter {
    fn new(to_snapshot: Vec<RcTracedBool>) -> Self {
        Snapshotter{to_snapshot}
    }

    pub fn snapshot (&self) {
        for i in self.to_snapshot.iter() {
            i.trace_current();
        }
    }
    pub fn clear_trace (&self) {
        for i in self.to_snapshot.iter() {
            i.trace.replace(vec![]);
        }
    }
}

pub struct RcSnapshotter(Rc<Snapshotter>);

impl Deref for RcSnapshotter {
    type Target = Snapshotter;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Clone for RcSnapshotter {
    fn clone(&self) -> Self {
        RcSnapshotter(Rc::clone(&self.0))
    }
}

impl RcSnapshotter {
    pub fn new(to_snapshot: Vec<RcTracedBool>) -> Self {
        RcSnapshotter(Rc::new(Snapshotter::new(to_snapshot)))
    }
}