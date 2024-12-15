
fn recursive(n: usize) -> usize {
    if n <= 1 {
        return n;
    }
    return n + 2 + recursive(n - 1);
}
fn tail_recursive(limit: usize, cur: usize, sum: usize) -> usize {
    if cur >= limit {
        return sum + limit;
    }

    return tail_recursive(limit, cur + 1, sum + cur);
}
fn main() {
    let res = recursive(1000000000000000);
    // let res = tail_recursive(10000000000, 1, 0);
    println!("res {}", res)
}
