use std::fmt::Debug;

fn main() {
    let v = vec![6, 23, 142, 2, 2, 1, 3, 7, 8, 9, 10];
    let sorted_v = bubble_sort(v.clone());
    print!("{:?},{:?}", v, sorted_v);
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

#[test]
fn test_all() {
    fn test<T: PartialOrd + Debug>(tc: Vec<T>, wants: Vec<T>) {
        let ret = bubble_sort(tc);
        assert!(wants.eq(&ret), "v:{:?},want_v:{:?}", ret, wants);
    }

    test(vec![6, 23, 142, 2, 2, 1, 3, 7, 8, 9, 10],
         vec![1, 2, 2, 3, 6, 7, 8, 9, 10, 23, 142], );

    test(vec!['k', 'a', 'b', 'c', 'd', 'e', 'f', 'g'],
         vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'k']);

    test(vec!["hello", "world", "rust", "c", "c++", "java", "python"],
         vec!["c", "c++", "hello", "java", "python", "rust", "world"]);

    test(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
         vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
}