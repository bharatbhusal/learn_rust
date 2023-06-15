fn main() {
    // let v1 = vec![4, 5, 6];
    // let i1 = v1.iter();
    // let c1 = i1.collect();
    // println!("Vectors: {:?}\nIterator: {:?}\nCollect: {:?}", v1, i1, c1);

    let mut dark_sides = ["fear", "anger", "anxiety", "hate", "suffering"];
    let mut data = [4, 3, 5, 2, 8, 0];

    // let mut i = 0;
    // while i < dark_sides.len() {
    //     println!("i: {}\titem: {}", i, dark_sides[i]);
    //     i += 1;
    // }

    // for (ind, &item) in dark_sides.iter().enumerate() {
    //     println!("i: {}\titem: {}", ind, item);
    // }
    for item in data.iter_mut() {
        *item = *item * 2;
    }
    println!("{:?}", data);
}

trait Iterator {
    type Item;
    fn next(&self) -> Option<Self::Item>;
}
