use std::borrow::Borrow;

/// Normalize
pub trait Normalize {
    // fn normalize(&mut self) -> impl Iterator<Item = f64>;

    // fn normalized<T: FromIterator<f64>>(&mut self) -> T {
    //     self.normalize().collect()
    // }

    fn normalize(&mut self) -> impl Iterator<Item = f64> {
        self.normalized().into_iter()
    }

    fn normalized(&mut self) -> Vec<f64>;
}

// impl Normalize for [f64] {
//     fn normalize(&mut self) -> Vec<f64> {
//         let sum: f64 = self.iter().sum();
//         self.iter().map(|unnormalized| unnormalized / sum).collect()
//     }
// }

// impl<I: Clone + Iterator<Item = f64>> Normalize for I {
//     fn normalize(&mut self) -> impl Iterator<Item = f64> {
//         let sum: f64 = self.clone().sum();
//         self.map(move |unnormalized| unnormalized / sum)
//     }
// }
// impl<I, T> Normalize for I
// where
//     I: Clone + Iterator<Item = T>,
//     T: Div<f64, Output = f64> + Sum<f64>,
//     f64: Sum<T>,
// {
//     fn normalize(&mut self) -> impl Iterator<Item = f64> {
//         let sum = self.fold(0.0, ||);
//         self.map(move |unnormalized| unnormalized / sum)
//     }
// }

impl<I: Iterator<Item = T>, T: Borrow<f64>> Normalize for I {
    fn normalized(&mut self) -> Vec<f64> {
        let mut normalized = Vec::new();
        let mut sum = 0.0;
        for unnormalized in self {
            let unnormalized = *unnormalized.borrow();
            normalized.push(unnormalized);
            sum += unnormalized;
        }
        for normalized in &mut normalized {
            *normalized /= sum;
        }
        normalized
    }
}

// impl<I, T> Normalize for I
// where
//     I: Iterator<Item = T>,
//     T: ToOwned<Owned = f64>,
// {
//     fn normalized(&mut self) -> Vec<f64> {
//         let mut normalized = Vec::new();
//         let mut sum = 0.0;
//         for unnormalized in self {
//             let unnormalized = unnormalized.to_owned();
//             normalized.push(unnormalized);
//             sum += unnormalized;
//         }
//         for normalized in &mut normalized {
//             *normalized /= sum;
//         }
//         normalized
//     }
// }
