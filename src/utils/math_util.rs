pub(crate) fn index_to_x(idx: i32, width: i32) -> i32 {
    idx % width
}

pub(crate) fn index_to_y(idx: i32, width: i32) -> i32 {
    idx / width
}
