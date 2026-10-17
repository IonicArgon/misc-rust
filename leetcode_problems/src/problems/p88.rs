pub fn p88_merge(nums1: &mut [i32], nums2: &[i32]) {
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

#[test]
fn test_p88() {
    let mut nums1 = [1, 2, 3, 0, 0, 0];
    let nums2 = [2, 5, 6];
    const ANS: [i32; 6] = [1, 2, 2, 3, 5, 6];

    p88_merge(&mut nums1, &nums2);

    assert_eq!(nums1, ANS);
}
