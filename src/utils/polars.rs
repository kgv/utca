use polars::prelude::*;

pub fn r#struct(name: &str) -> StructNameSpace {
    col(name).r#struct()
}

/// Extension methods for [`DataFrame`]
pub trait DataFrameExt {
    fn destruct(&self, name: &str) -> DataFrame;

    fn f64(&self, name: &str) -> &Float64Chunked;

    fn list(&self, name: &str) -> &ListChunked;

    fn str(&self, name: &str) -> &StringChunked;

    fn u8(&self, name: &str) -> &UInt8Chunked;
}

impl DataFrameExt for DataFrame {
    fn f64(&self, name: &str) -> &Float64Chunked {
        self[name].f64().unwrap()
    }

    fn list(&self, name: &str) -> &ListChunked {
        self[name].list().unwrap()
    }

    fn str(&self, name: &str) -> &StringChunked {
        self[name].str().unwrap()
    }

    fn u8(&self, name: &str) -> &UInt8Chunked {
        self[name].u8().unwrap()
    }

    fn destruct(&self, name: &str) -> DataFrame {
        self.select([&name]).unwrap().unnest([name]).unwrap()
    }
}

/// Extension methods for [`Expr`]
pub trait ExprExt {
    fn normalize(self) -> Expr;

    fn r#struct(self) -> StructNameSpace;

    fn suffix(self, suffix: &str) -> Expr;
}

impl ExprExt for Expr {
    fn normalize(self) -> Expr {
        self.apply(
            |series| {
                let chunked_array = series.f64()?;
                Ok(Some(
                    chunked_array
                        .into_iter()
                        .map(|option| Some(option? / chunked_array.sum()?))
                        .collect(),
                ))
            },
            GetOutput::same_type(),
        )
    }

    fn r#struct(self) -> StructNameSpace {
        self.struct_()
    }

    fn suffix(self, suffix: &str) -> Expr {
        self.name().suffix(suffix)
    }
}

/// Extension methods for [`Series`]
pub trait SeriesExt {
    fn r#struct(&self) -> PolarsResult<&ChunkedArray<StructType>>;
}

impl SeriesExt for Series {
    fn r#struct(&self) -> PolarsResult<&ChunkedArray<StructType>> {
        self.struct_()
    }
}
