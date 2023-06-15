use generic_types::duplicate::remove_duplicate;
use generic_types::max_element::max_ele;
use generic_types::stack::MyStack;
use generic_types::swap::swap;

fn main() {
    // swap
    let mut a = "5.0";
    let mut b = "10.3";
    println!("A: {}\tB: {}", a, b);
    swap(&mut a, &mut b);
    println!("A: {}\tB: {}", a, b);

    // Maximum number
    println!("{:?}", max_ele(&[4, 3, 5, 10, 9, 3]));

    // Stack
    let mut new_stack: MyStack<String> = MyStack::new(5);
    new_stack.push("hello".to_string());
    new_stack.push("how".to_string());
    new_stack.push("hi".to_string());
    println!(
        "Top index push: {:#?}",
        new_stack.push("bharat".to_string())
    );
    println!("{:#?}", new_stack);
    println!("Top index pop: {:#?}", new_stack.pop());
    println!("Top index pop: {:#?}", new_stack.pop());
    println!("{:#?}", new_stack);
    println!("Peek: {:#?}", new_stack.peek());

    // Duplicate
    let list = vec![3, 3, 4, 3, 2, 5, 4, 9, 2, 2, 3, 4, 5, 6];
    let new_list = remove_duplicate(&list);
    println!("{:#?}", new_list);
}
