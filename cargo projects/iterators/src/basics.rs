pub fn basic_iterator() {
    // Part 1
    let v1 = vec![4, 5, 6];
    let i1 = v1.iter();
    let c1: Vec<_> = i1.clone().collect();
    println!("Vectors: {:?}\nIterator: {:?}\nCollect: {:?}", v1, i1, c1);

    // Part2
    let mut i = 0;
    let dark_sides = vec!["fear", "anger", "sad", "anxiety"];
    while i < dark_sides.len() {
        println!("i: {}\titem: {}", i, dark_sides[i]);
        i += 1;
    }
    //Part 3
    for (ind, &item) in dark_sides.iter().enumerate() {
        println!("i: {}\titem: {}", ind, item);
    }

    //Part 4
    let data = [4, 3, 5, 2, 8, 0];
    println!("{:?}", data);

    let x: Vec<i32> = data.iter().map(|s| s + 1).filter(|s| *s > 4).collect();
    println!("{:?}", data);
    println!("{:?}", x);
    let val: Vec<i32> = { 1..100 }
        .into_iter()
        .filter(|s| *s % 2 == 0 && *s > 50)
        .collect();
    println!("{:?}", val);
}
