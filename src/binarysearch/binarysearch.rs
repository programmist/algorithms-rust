pub fn search(nums: &[i32], key: i32) -> i32 {
    let mut low = 0;
    let mut high = nums.len() - 1;

    while low <= high {
        let mid: usize = low + (high - low) / 2;
        if key < nums[mid] {
            high = mid - 1;
        } else if key > nums[mid] {
            low = mid + 1;
        } else {
            return mid.try_into().unwrap();
        }
    }
    return -1;
}
