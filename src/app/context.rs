use crate::app::computers::calculator::{Calculated, Key as CalculatorKey};
use egui::Context;
use polars::prelude::*;

const CONFIGURATION: &str = "Configuration";

/// Extension methods for [`Context`]
pub(crate) trait ContextExt {
    fn configure(&self, data_frame: &DataFrame);
    fn calculate(&self) -> Option<DataFrame>;
    fn compose(&self) -> Option<DataFrame>;
}

impl ContextExt for Context {
    fn configure(&self, data_frame: &DataFrame) {
        self.data_mut(|data| data.insert_temp(CONFIGURATION.into(), data_frame.clone()));
    }

    fn calculate(&self) -> Option<DataFrame> {
        let data_frame = self.data_mut(|data| data.get_temp(CONFIGURATION.into()))?;
        Some(self.memory_mut(|memory| {
            memory.caches.cache::<Calculated>().get(CalculatorKey {
                data_frame: &data_frame,
            })
        }))
    }

    fn compose(&self) -> Option<DataFrame> {
        let data_frame = self.calculate()?;
        // self.memory_mut(|memory| memory.caches.cache::<Composed>().get((&*self).into()));
        Some(data_frame)
    }
}
