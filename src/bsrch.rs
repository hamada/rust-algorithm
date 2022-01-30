const NOT_FOUND: i8 = -1;
const N: u8 = 20;
const RAND_MAX_F: f64 = libc::RAND_MAX as f64;

fn bsrch(x: u8, a: &[u8; 20], mut left: u8, mut right: u8) -> i8 {
    while left < right {
        let mid = (left + right) / 2;
        if a[mid as usize] < x {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    if a[left as usize] == x { return left as i8; }

    return NOT_FOUND;
}

fn main() {
    let mut buffer = Vec::with_capacity(16);
    buffer.resize(16, 0u8);
    srand::read(&mut buffer);
    let mut r = 1.0;
    let mut a: [u8; 20] = [0; 20];
    let nums = (0..N).collect::<Vec<u8>>();

    for i in nums.iter().rev() {
        unsafe {
            let base: f64 = (libc::rand() as f64) / (RAND_MAX_F + 1.0);
            let exponent = 1.0 / ((i + 1) as f64);
            r = r * base.powf(exponent); // 1未満0以上の一様乱数を降順で作る
            a[*i as usize] = (100.0 * r) as u8; // 0以上100未満の整数に直す
        }
    }

    println!("i : ");
    for i in &nums { println!("   {}", i); }
    println!("a[i]: ");
    for i in &nums { println!("     {}", a[*i as usize]); }
    println!("何を探しますか? ");
    let x: u8 = 94; // TODO: 標準入力を参照する処理を実装する

    let i = bsrch(x, &a, 0, N - 1);
    if i != NOT_FOUND {
        println!("i = {}", i);
    } else {
        println!("見つかりません");
    }
}
