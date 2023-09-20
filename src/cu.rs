use maplit::btreemap;
use molecule::{
    atom::Atom::{C, H, O},
    Counter,
};
use std::num::NonZeroUsize;

pub(crate) macro ether {
    ($counter:expr) => { to_cu($counter) },
    ($c:expr, $u:expr) => { from_cu($c, $u).unwrap() }
}

/// Saturation
pub trait Saturation {
    fn unsaturation(&self) -> usize;

    fn saturation(&self) -> bool {
        self.unsaturation() == 0
    }
}

impl Saturation for Counter {
    fn unsaturation(&self) -> usize {
        if self.is_empty() {
            return 0;
        }
        // .expect("expected some `C` atoms");
        // .expect("expected some `H` atoms");
        (|| {
            let c = self.get(&C)?;
            let h = self.get(&H)?;
            Some(c.get() - h.get() / 2)
        })()
        .unwrap_or_default()
    }
}

pub fn from_cu(c: usize, u: usize) -> Option<Counter> {
    let c = c + 1;
    Some(Counter::new(btreemap! {
        C => NonZeroUsize::new(c)?,
        H => NonZeroUsize::new(2 * c - 2 * u)?,
        O => NonZeroUsize::new(2)?,
    }))
}

pub fn to_cu(counter: &Counter) -> Option<(usize, usize)> {
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
