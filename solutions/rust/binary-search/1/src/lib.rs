pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty(){
        return None;
    }
    fn binary_search(arr:&[i32],target:i32) -> Option<usize> {
        let mut l = 0;
        let mut r = arr.len();
        while l < r {
            let mid = l + (r-l)/2;
            if arr[mid] < target{
                l = mid + 1
            } else if arr[mid] > target {
                r = mid
            } else {
                return Some(mid);
            }
        }
        return None;
    }
    return binary_search(array,key);
}
