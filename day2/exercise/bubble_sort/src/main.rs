use rand::Rng;

fn main() {
    println!("随机生成10个1-100之间的数字");

    let mut nums = [0u32; 10];
    for i in 0..10 {
        nums[i] = rand::thread_rng().gen_range(1..=100);
    }

    println!("排序之前的数组为: {:?}", nums);

    let len = nums.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }
    }

    println!("排序之后的数组为: {:?}", nums);
}
