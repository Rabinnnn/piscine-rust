// Define the WorkEnvironment struct, representing a stack of workers.
#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link, // The top of the stack (linked list) of workers.
}

// Define a type alias for the optional boxed Worker node.
pub type Link = Option<Box<Worker>>;

// Define the Worker struct, each containing role, name, and a link to the next worker.
#[derive(Debug)]
pub struct Worker {
    pub role: String,      // Role of the worker (e.g., "Manager", "Engineer").
    pub name: String,      // Name of the worker.
    pub next: Link,        // Link to the next worker in the chain.
}

impl WorkEnvironment {
    // Create a new empty WorkEnvironment with no workers.
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None }
    }

    // Add a new worker to the top of the stack.
    pub fn add_worker(&mut self, role: String, name: String) {
        let new_node = Box::new(Worker {
            role: role,             // Assign the given role.
            name: name,             // Assign the given name.
            next: self.grade.take() // Set the current top as the next node.
        });
        self.grade = Some(new_node); // Update the top to be the new worker.
    }

    // Remove the top worker from the stack and return their name.
    pub fn remove_worker(&mut self) -> Option<String> {
        self.grade.take().map(|node| {
            self.grade = node.next; // Set the next worker as the new top.
            node.name               // Return the removed worker's name.
        })
    }

    // Return the name and role of the top worker without removing them.
    pub fn last_worker(&self) -> Option<(String, String)> {
        self.grade
            .as_ref()                              // Borrow the top worker.
            .map(|node| (node.name.clone(), node.role.clone())) // Clone and return their data.
    }
}