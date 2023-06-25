// /// Normalize
// pub trait Normalize {
//     fn normalize(&self) -> Array1<f64>;
// }

// impl<T: Data<Elem = f64>> Normalize for ArrayBase<T, Ix1> {
//     fn normalize(&self) -> Array1<f64> {
//         let sum = self.sum();
//         self.map(|unnormalized| {
//             let normalized = unnormalized / sum;
//             normalized
//         })
//     }
// }

/// Normalize
pub trait Normalize {
    fn normalize(&mut self) -> Vec<f64>;
}

// impl Normalize for [f64] {
//     fn normalize(&mut self) -> Vec<f64> {
//         let sum: f64 = self.iter().sum();
//         self.iter().map(|unnormalized| unnormalized / sum).collect()
//     }
// }

impl<T: Clone + Iterator<Item = f64>> Normalize for T {
    fn normalize(&mut self) -> Vec<f64> {
        let sum: f64 = self.clone().sum();
        self.map(|unnormalized| unnormalized / sum).collect()
    }
}
