#[no_mangle]
pub extern "C" fn dot_product(arr1: *const i32, arr2: *const i32, len: usize) -> i32 {
    if arr1.is_null() || arr2.is_null() {
        return -1;
    }
    let slice1 = unsafe { std::slice::from_raw_parts(arr1, len) };
    let slice2 = unsafe { std::slice::from_raw_parts(arr2, len) };
    let result = slice1.iter().zip(slice2.iter()).map(|(&x, &y)| x * y).sum();
    result
}

#[no_mangle]
pub extern "C" fn elementwise_product(arr1: *const i32, arr2: *const i32, len: usize) -> *mut i32 {
    if arr1.is_null() || arr2.is_null() {
        return std::ptr::null_mut();
    }
    let slice1 = unsafe { std::slice::from_raw_parts(arr1, len) };
    let slice2 = unsafe { std::slice::from_raw_parts(arr2, len) };
    let product: Vec<i32> = slice1.iter().zip(slice2.iter()).map(|(&x, &y)| x * y).collect();
    let result = product.as_ptr().clone();
    std::mem::forget(product);
    result as *mut i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_dot_product() {
        let arr1 = [1, 2, 3];
        let arr2 = [4, 5, 6];
        let result = super::dot_product(arr1.as_ptr(), arr2.as_ptr(), arr1.len());
        assert_eq!(result, 32);
    }

    #[test]
    fn test_elementwise_product() {
        let arr1 = [1, 2, 3];
        let arr2 = [4, 5, 6];
        let len = arr1.len();
        let result_ptr = super::elementwise_product(arr1.as_ptr(), arr2.as_ptr(), len);
        let result_vec: Vec<i32> = unsafe { Vec::from_raw_parts(result_ptr, len, len) };
        assert_eq!(result_vec, vec![4, 10, 18]);
        std::mem::forget(result_vec);
    }
}
