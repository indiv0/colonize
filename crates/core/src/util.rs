/// Converts a 2d int array to a float array.
pub fn array_int_to_float(array: [i32; 2]) -> [f64; 2] {
    [array[0] as f64, array[1] as f64]
}

pub(crate) fn array_float_to_int(array: [f64; 2]) -> [i32; 2] {
    [array[0] as i32, array[1] as i32]
}
