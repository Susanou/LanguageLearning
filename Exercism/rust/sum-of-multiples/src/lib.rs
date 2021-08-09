pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut list = Vec::new();

    for i in 1..limit {
        for f in factors.iter() {
            if f == &0{
                break;
            }
            else if i % f == 0 && !list.contains(&i) {
                list.push(i);
                break;
            }
        }
    }

    list.iter().sum()
}
