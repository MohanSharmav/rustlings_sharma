// vecs1.rs
// Your task is to create a `Vec` which holds the exact same elements
// as in the array `a`.
// Make me compile and pass the test!
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.


fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    // let mut v =Vec::new();
// for i in 0..a.len(){
//     v.push(a[i] as i32);
// }
//      let v = v.iter().map(|i | v.push(a[i] as i32)).collect();
    let v = a.iter().map(|&x| x).collect::<Vec<_>>();

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}


