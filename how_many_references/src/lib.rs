// Make Rc (Reference Counted pointer) from std::rc available for use.
pub use std::rc::Rc;

// Define a struct Node that holds a list of Rc<String> elements.
#[derive(Debug)]
pub struct Node {
    pub ref_list: Vec<Rc<String>>, // Vector of reference-counted Strings.
}

impl Node {
    // Create a new Node with a given list of Rc<String>.
    pub fn new(ref_list: Vec<Rc<String>>) -> Node {
        Node { ref_list: ref_list } // Initialize the struct with the provided list.
    }

    // Add a new Rc<String> element to the ref_list.
    pub fn add_element(&mut self, v: Rc<String>) {
        self.ref_list.push(v); // Push the element into the vector.
    }

    // Remove all references from ref_list that point to the same allocation as v.
    pub fn rm_all_ref(&mut self, v: Rc<String>) {
        self.ref_list.retain(|x| is_same_allocate(x, &v)); // Retain only elements that pass the filter.
    }
}

// Helper function to determine if x and v do not point to the same allocation.
fn is_same_allocate(x: &Rc<String>, v: &Rc<String>) -> bool {
    if is_eq(x, v) {
        return v != x; // Return true if x and v point to same allocation but are different references.
    }
    return true; // Keep element if they are not pointing to the same allocation.
}

// Public function to return the number of strong references to a value.
pub fn how_many_references(value: &Rc<String>) -> usize {
    Rc::strong_count(value) // Return the strong reference count.
}

// Helper function to check if two Rc<String> point to the same allocation.
fn is_eq(value: &Rc<String>, cmp: &Rc<String>) -> bool {
    Rc::ptr_eq(value, cmp) // Return true if both Rc pointers point to the same allocation.
}

// TODO review the tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_how_many_references() {
        let a = Rc::new(String::from("a"));
        let b = Rc::new(String::from("b"));
        let c = Rc::new(String::from("c"));
        let d = Rc::new(String::from("d"));
        let mut new_node = Node::new(vec![]);
        new_node.add_element(b.clone());
        new_node.add_element(a.clone());
        new_node.add_element(c.clone());
        new_node.add_element(a.clone());

        assert_eq!(how_many_references(&d), 1);
        assert_eq!(how_many_references(&a), 3);
        assert_eq!(how_many_references(&b), 2);
        assert_eq!(how_many_references(&c), 2);
    }

    #[test]
    fn test_rm_all_ref() {
        let a = Rc::new(String::from("a"));
        let b = Rc::new(String::from("b"));
        let c = Rc::new(String::from("c"));
        let d = Rc::new(String::from("d"));

        let a1 = Rc::new(String::from("a"));
        let b1 = Rc::new(String::from("b"));
        let c1 = Rc::new(String::from("c"));
        let d1 = Rc::new(String::from("d"));
        let mut new_node = Node::new(vec![
            d.clone(),
            d.clone(),
            b.clone(),
            a.clone(),
            c.clone(),
            a.clone(),
            d.clone(),
        ]);

        new_node.rm_all_ref(a1.clone());
        assert_eq!(how_many_references(&a), 3);
        new_node.rm_all_ref(a.clone());
        assert_eq!(how_many_references(&a), 1);

        new_node.rm_all_ref(b1.clone());
        assert_eq!(how_many_references(&b), 2);
        new_node.rm_all_ref(b.clone());
        assert_eq!(how_many_references(&b), 1);

        new_node.rm_all_ref(c1.clone());
        assert_eq!(how_many_references(&c), 2);
        new_node.rm_all_ref(c.clone());
        assert_eq!(how_many_references(&c), 1);

        new_node.rm_all_ref(d1.clone());
        assert_eq!(how_many_references(&d), 4);
        new_node.rm_all_ref(d.clone());
        assert_eq!(how_many_references(&d), 1);
    }
}