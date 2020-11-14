fn sum_integer_sets(t: &[u32]) -> Option<u32> {
    if t.len() > 0 {
        let mut total = 0;
        for item in t.iter() {
            total += item;
        }
        Some(total)
    }
    else {
        None
    }
}

fn main() {
    let arr: [u32; 5] = [1, 2, 3, 4, 5];

    println!("array sum is {:?}",sum_integer_sets(&arr));

}
