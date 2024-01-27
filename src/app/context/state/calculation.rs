use crate::app::context::settings::calculation::From;
use egui::epaint::util::FloatOrd;
use itertools::izip;
use serde::{Deserialize, Serialize};
use std::{
    hash::{Hash, Hasher},
    iter::zip,
};

/// Calculated data
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Calculated {
    pub(in crate::app) tags123: Calculable<Normalizable<Vec<f64>>>,
    pub(in crate::app) dags1223: Calculable<Normalizable<Vec<f64>>>,
    pub(in crate::app) mags2: Calculable<Normalizable<Vec<f64>>>,
    pub(in crate::app) dags13: Dags13<Normalizable<Vec<f64>>>,
}

impl Calculated {
    pub(in crate::app) fn zip(
        &self,
    ) -> impl Iterator<
        Item = (
            Calculable<Normalizable<f64>>,
            Calculable<Normalizable<f64>>,
            Calculable<Normalizable<f64>>,
            Dags13<Normalizable<f64>>,
        ),
    > + '_ {
        izip!(
            self.tags123.zip(),
            self.dags1223.zip(),
            self.mags2.zip(),
            self.dags13.zip(),
        )
    }
}

impl Hash for Calculated {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tags123.hash(state);
        self.dags1223.hash(state);
        self.mags2.hash(state);
        self.dags13.hash(state);
    }
}

/// Dags13
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Dags13<T> {
    /// From dag1223
    pub(in crate::app) dag1223: T,
    /// From mag2
    pub(in crate::app) mag2: T,
}

impl<T> Dags13<T> {
    pub(in crate::app) fn value(&self, from: From) -> &T {
        match from {
            From::Dag1223 => &self.dag1223,
            From::Mag2 => &self.mag2,
        }
    }
}

impl Dags13<Normalizable<Vec<f64>>> {
    pub(in crate::app) fn zip(&self) -> impl Iterator<Item = Dags13<Normalizable<f64>>> + '_ {
        zip(self.dag1223.zip(), self.mag2.zip()).map(|(dag1223, mag2)| Dags13 { dag1223, mag2 })
    }
}

/// Calculable
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(in crate::app) struct Calculable<T> {
    pub(in crate::app) experimental: T,
    pub(in crate::app) theoretical: T,
}

impl Calculable<Normalizable<Vec<f64>>> {
    pub(in crate::app) fn is_experimental(&self) -> bool {
        !self
            .experimental
            .normalized
            .iter()
            .any(|item| item.is_nan())
    }

    pub(in crate::app) fn value(&self) -> &Normalizable<Vec<f64>> {
        if self.is_experimental() {
            &self.experimental
        } else {
            &self.theoretical
        }
    }

    pub(in crate::app) fn zip(&self) -> impl Iterator<Item = Calculable<Normalizable<f64>>> + '_ {
        zip(self.experimental.zip(), self.theoretical.zip()).map(|(experimental, theoretical)| {
            Calculable {
                experimental,
                theoretical,
            }
        })
    }
}

impl Calculable<Normalizable<f64>> {
    pub(in crate::app) fn is_experimental(&self) -> bool {
        !self.experimental.normalized.is_nan()
    }

    pub(in crate::app) fn value(&self) -> &Normalizable<f64> {
        if self.is_experimental() {
            &self.experimental
        } else {
            &self.theoretical
        }
    }
}

/// Normalizable
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(in crate::app) struct Normalizable<T> {
    pub(in crate::app) unnormalized: T,
    pub(in crate::app) normalized: T,
}

impl Normalizable<Vec<f64>> {
    pub(in crate::app) fn unnormalized(&mut self, unnormalized: impl IntoIterator<Item = f64>) {
        self.unnormalized = unnormalized.into_iter().collect();
        let sum: f64 = self.unnormalized.iter().sum();
        self.normalized = self
            .unnormalized
            .iter()
            .map(|unnormalized| unnormalized / sum)
            .collect();
    }
}

impl Normalizable<Vec<f64>> {
    pub(in crate::app) fn zip(&self) -> impl Iterator<Item = Normalizable<f64>> + '_ {
        zip(&self.unnormalized, &self.normalized).map(|(&unnormalized, &normalized)| Normalizable {
            unnormalized,
            normalized,
        })
    }
}

impl Hash for Normalizable<Vec<f64>> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for unnormalized in &self.unnormalized {
            unnormalized.ord().hash(state);
        }
        for normalized in &self.normalized {
            normalized.ord().hash(state);
        }
    }
}
