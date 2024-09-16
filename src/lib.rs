use std::cell::RefCell;
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};

//Use of RefCell because otherwise monitor cannot hold on to variables while value changes.
pub struct TracedBool<'a>{
    val: RefCell<bool>,
    trace: RefCell<Vec<bool>>,
    snap: RefCell<Option<&'a Snapshotter<'a>>>
}

impl <'a>TracedBool<'a> {
    pub fn new (val: bool) -> Self {
        TracedBool{
            val: RefCell::new(val),
            trace: RefCell::new(vec![val]),
            snap: RefCell::new(None)
        }
    }
    pub fn not (&self) {
        self.val.borrow_mut().bitxor_assign(true);
        if let Some(s) = *self.snap.borrow() {
            s.snapshot();
        }
    }
    pub fn assign_true(&self) {
        self.val.borrow_mut().bitor_assign(true);
        if let Some(s) = *self.snap.borrow() {
            s.snapshot();
        }
    }
    pub fn assign_false(&self) {
        self.val.borrow_mut().bitand_assign(false);
        if let Some(s) = *self.snap.borrow() {
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

    pub fn add_snapshotter(&self, s: &'a Snapshotter<'a>) {
        self.snap.borrow_mut().replace(s);
    }
}

#[derive(Clone)]
pub struct Snapshotter<'a> {
    to_snapshot: Vec<&'a TracedBool<'a>>
}

impl <'a>Snapshotter<'a> {
    pub fn new(to_snapshot: Vec<&'a TracedBool<'a>>) -> Self {
        Snapshotter{to_snapshot}
    }

    pub fn snapshot (&self) {
        for i in self.to_snapshot.iter() {
            i.trace_current();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}