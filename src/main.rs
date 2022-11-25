fn eq_pairs<T1, D1, T2, D2>(mut it1: T1, mut it2: T2) -> impl Iterator<Item = (D1, D2)> where
    T1: Iterator<Item = D1>,
    D1: PartialOrd<D2>,
    T2: Iterator<Item = D2>,
    D2: PartialOrd<D1> {
    std::iter::from_fn(move || {
        let mut x = it1.next();
        let mut y = it2.next();
        while x.is_some() && y.is_some() {
            if x.as_ref().unwrap() < y.as_ref().unwrap() {
                x = it1.next();
            }
            else if x.as_ref().unwrap() > y.as_ref().unwrap() {
                y = it2.next();
            }
            else {
                return Some((x.unwrap(), y.unwrap()));
            }
        }
        None
    })
}

#[derive(PartialEq, PartialOrd, Debug)]
enum Example {
    One,
    Two,
    Three,
    Four,
    Five,
}

use Example::*;

fn main() {
    // let vec1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    // let vec2 = vec![0, 2, 5, 10];
    let  vec1 = vec!(One, Three, Five);
    let  vec2 = vec!(Two, Four, Five);
    for (a, b) in eq_pairs(vec1.iter(), vec2.iter()) {
        println!("{a:?} and {b:?}");
    }
}
