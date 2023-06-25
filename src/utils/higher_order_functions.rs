//! https://stackoverflow.com/questions/59602202/how-can-i-retain-vector-elements-with-their-original-index

pub fn temp<T, F: FnMut(usize) -> bool>(mut f: F) -> impl FnMut(&T) -> bool {
    let mut index = 0;
    move |item| {
        if f(index) {
            index += 1;
            return true;
        }
        false
    }
}

pub fn first<T, U, V, F: FnMut(T) -> V>(mut f: F) -> impl FnMut((T, U)) -> V {
    move |(t, _)| f(t)
}

pub fn second<T, U, V, F: FnMut(U) -> V>(mut f: F) -> impl FnMut((T, U)) -> V {
    move |(_, u)| f(u)
}

pub fn index<T, U, F: FnMut(usize) -> U>(mut f: F) -> impl FnMut(&T) -> U {
    let mut index = 0;
    move |_| (f(index), index += 1).0
}

pub fn with_index<T, U, F: FnMut(usize, &T) -> U>(mut f: F) -> impl FnMut(&T) -> U {
    let mut index = 0;
    move |item| (f(index, item), index += 1).0
}
