pub fn display(&self) {
    let mut current: &Option<Box<Node>> = &self.head;
    let mut result = String::new();
    loop {
        match current {
            Some(node) => {
                result = format!("{} {}", result, node.value);
                current = &node.next;
            }
            None => {
                break;
            }
        }
    }
    println!("{}", result);
}
impl fmt::Display for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Box<Node>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                }
                None => {
                    break;
                }
            }
        }
        write!(f, "{}", result)
    }
}
impl Drop for LinkedList {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}
