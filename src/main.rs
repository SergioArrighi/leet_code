use std::i32;


pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    let mut total_diff = i32::MAX;
    let mut result = 0;
    nums.sort();
    println!("{:?}", nums);

    for i in 0..nums.len() - 1 {
        let mut left = i + 1;
        let mut right = nums.len() - 1;
        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            let diff = (sum - target).abs();
            println!("i {}, l {}, r {}, s {}, d {}", i, left, right, sum, diff);
            if diff <= total_diff {
                total_diff = diff;
                result = sum;
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    result
}

fn main() {
    let nums = vec![2, 5, 6, 7];
    let result = three_sum_closest(nums, 16);
    println!("{:?}", result);
}
