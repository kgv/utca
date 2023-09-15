//! https://stackoverflow.com/questions/59602202/how-can-i-retain-vector-elements-with-their-original-index

pub(crate) fn with_index<T, U, F: FnMut(usize, &T) -> U>(mut f: F) -> impl FnMut(&T) -> U {
    let mut index = 0;
    move |item| (f(index, item), index += 1).0
}
