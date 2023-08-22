fn main() {
    let v = vec![6, 23, 142, 2, 2, 1, 3, 7, 8, 9, 10];
    let sorted_v = bubble_sort(v.clone());
    println!("{:?}\n{:?}\n", v, sorted_v);

    let v = vec!['k', 'a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let sorted_v = bubble_sort(v.clone());
    println!("{:?}\n{:?}\n", v, sorted_v);

    let v = vec!["hello", "world", "rust", "c", "c++", "java", "python"];
    let sorted_v = bubble_sort(v.clone());
    println!("{:?}\n{:?}\n", v, sorted_v);

    let v = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let sorted_v = bubble_sort(v.clone());
    println!("{:?}\n{:?}\n", v, sorted_v);
}

fn bubble_sort<T: PartialOrd>(mut v: Vec<T>) -> Vec<T> {
    for i in 0..v.len() {
        for j in i..v.len() {
            if v[i] > v[j] {
                v.swap(i, j);
            }
        }
    }
    v
}