pub fn merge_88(nums1: &mut [i32], nums2: &[i32]) {
    let mut i: usize = nums1.len() - nums2.len();
    let mut j: usize = nums2.len();

    while j > 0 {
        if i > 0 && nums1[i - 1] > nums2[j - 1] {
            nums1[i + j - 1] = nums1[i - 1];
            i -= 1;
        } else {
            nums1[i + j - 1] = nums2[j - 1];
            j -= 1;
        }
    }
}

pub fn merge_88_val(f: fn(nums1: &mut [i32], nums2: &[i32])) {
    let mut merge_nums1 = [1, 2, 3, 0, 0, 0];
    let merge_nums2 = [2, 5, 6];
    const MERGE_ANS: [i32; 6] = [1, 2, 2, 3, 5, 6];
    f(&mut merge_nums1, &merge_nums2);
    assert_eq!(merge_nums1, MERGE_ANS);
}
