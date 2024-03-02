use molecule::{
    atom::{isotopes::*, Isotope},
    Counter,
};

const H: Isotope = Isotope::H(H::One);
const C: Isotope = Isotope::C(C::Twelve);

/// Equivalent carbon number
///
/// `ECN = CN - 2DB`
pub trait Ecn {
    fn ecn(&self) -> usize;
}

impl Ecn for Counter {
    fn ecn(&self) -> usize {
        let c = self.count(C);
        let h = self.count(H);
        assert!(h >= c, "Invalid fatty acid for ECN calculation {self}");
        // TODO: необходимо переделать хранимую формулу. В Configuration
        // участвуют метиловые эфиры, а в ecn - ацилы жирных кислот.
        h - c - 1
    }
}
