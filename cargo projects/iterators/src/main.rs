use iterators::basics::basic_iterator;
use iterators::shoe::*;
fn main() {
    // basic_iterator();
    let s1 = Shoe {
        size: 5,
        style: "Boots".to_string(),
    };
    let s2 = Shoe {
        size: 8,
        style: "ladies".to_string(),
    };
    let s3 = Shoe {
        size: 8,
        style: "treak".to_string(),
    };
    let s4 = Shoe {
        size: 9,
        style: "daily".to_string(),
    };
    let s5 = Shoe {
        size: 8,
        style: "Boots".to_string(),
    };

    let all_shoes = vec![s1, s2, s3, s4, s5];
    let new_list = shoe_8(all_shoes, 8);
    println!("Shoe 8: {}", new_list.len());
}
