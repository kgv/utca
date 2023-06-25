use molecule::{
    atom::Atom::{C, H, O},
    Counter,
};
use std::num::NonZeroUsize;

pub(crate) macro ether {
    ($counter:expr) => {
        to_ether($counter)
    },
    ($c:expr, $n:expr) => {
        from_ether($c, $n).unwrap()
    }
}

pub fn from_ether(c: usize, bounds: usize) -> Option<Counter> {
    let c = c + 1;
    Some(Counter::new(maplit::btreemap! {
        C => NonZeroUsize::new(c)?,
        H => NonZeroUsize::new(2 * c - 2 * bounds)?,
        O => NonZeroUsize::new(2).unwrap(),
    }))
}

pub fn to_ether(counter: &Counter) -> Option<(usize, usize)> {
    let c = counter.get(&C)?;
    let h = counter.get(&H)?;
    let o = counter.get(&O)?;
    if o.get() != 2 {
        return None;
    }
    Some((c.get() - 1, c.get() - h.get() / 2))
}

// #[test]
// fn test() {
//     // let t = unsafe { NonZeroUsize::new_unchecked(0) };
//     // println!("{}", t.get());
//     // println!("{}", unsafe { t.unchecked_add(1) });
//     let counts: Counter = "C2H5OH".parse().unwrap();
//     println!("{counts}");
// }
