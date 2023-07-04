// Write a function that takes a vector of integers and returns a new vector containing only the unique elements from the input vector.

mod anagram;
mod intersection;
mod nested;
mod ordering;
mod unique;

use crate::anagram::*;
use crate::intersection::*;
use crate::nested::*;
use crate::ordering::*;
use crate::unique::*;

fn main() {
    //assignment
    let mut v = vec![1, 2, 3, 4, 5];

    //query-indexing
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    //query-method
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    //iteration
    for i in &v {
        println!("{}", i);
    }
    println!("The first element is {}", &v[0]);

    //changing values.
    for i in &mut v {
        *i = *i + 1;
    }
    println!("The first element is {}", &v[0]);

    //filter the unique values
    let my_lst = vec![5, 4, 3, 6, 5, 4, 1, 2, 9, 4, 8, 9];
    println!("{:?}", my_lst);
    let new_lst = unique(my_lst);
    println!("{:?}", new_lst);

    //intersection of two lists
    let lst1 = vec![4, 5, 6, 3];
    let lst2 = vec![4, 0, 6, 3];
    let intersection_res = insertion(lst1, lst2);
    println!("{:?}", intersection_res);

    //anagram or not
    let my_str1 = "rami";
    let my_str2 = "mira";
    println!("{}", anagram(my_str1, my_str2));

    //nestec vector
    let vec = vec![vec![4, 5, 3], vec![2, 1, 3], vec![9, 5, 4]];
    println!("{:?}", nested(vec));

    //ordering the elements
    let my_lst = vec![5, 4, 3, 6, 5, 4, 1, 2, 9, 4, 8, 9];
    println!("{:?}", order(my_lst));
}
