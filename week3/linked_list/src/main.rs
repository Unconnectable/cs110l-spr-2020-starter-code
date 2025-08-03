use linked_list::LinkedList;
pub mod linked_list;
use crate::linked_list::ComputeNorm;
fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display

    // If you implement iterator trait:
    //for val in &list {
    //    println!("{}", val);
    //}
    let green = "\x1b[32m";
    let reset = "\x1b[0m";
    println!("\n{green}String Type LinkList{reset}:");
    let mut list: LinkedList<String> = LinkedList::new();
    println!("List get_size() is: {}", list.get_size());
    assert!(list.is_empty());
    list.push_front("a".to_string());
    list.push_front("bb".to_string());
    list.push_front("ccc".to_string());
    list.push_front("dddd".to_string());
    println!("list is: {}", list);
    let mut list_clone = list.clone();
    println!("{green}list_clone: {}{reset}", list_clone);

    list_clone.push_front("eeeee".to_string());
    list_clone.push_front("f".to_string());

    //test Eq
    list.push_front("eeeee".to_string());
    list.push_front("f".to_string());

    println!("list = list_clone: {}", list == list_clone);

    //Norm
    println!("\n{green}Calculate Nrom{reset}");
    let mut f64_list: LinkedList<f64> = LinkedList::new();
    f64_list.push_front(300.0);
    f64_list.push_front(400.0);
    println!("{}", f64_list.compute_norm());
}
