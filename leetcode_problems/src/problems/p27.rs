pub fn p27_remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&x| x != val);
    nums.len() as i32
}

#[test]
fn test_p27() {
    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    const VAL: i32 = 2;
    const EXPECTED_NUMS: [i32; 5] = [0, 0, 1, 3, 4];

    let k = p27_remove_element(&mut nums, VAL);

    assert_eq!(k, EXPECTED_NUMS.len() as i32);
    nums.sort();

    for i in 0..EXPECTED_NUMS.len() {
        assert_eq!(nums[i], EXPECTED_NUMS[i]);
    }
}
