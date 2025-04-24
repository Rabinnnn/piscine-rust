#[derive(Debug, Clone)]
pub struct Person<'a> {
    pub name: &'a str,
    pub age: u8,
}

impl<'a> Person<'a> {
    pub fn new(name: &str) -> Person {
        Person { name, age: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fields() {
        let person = Person {
            name: "Dijkstra",
            age: 10,
        };
        assert_eq!(person.age, 10);
        assert_eq!(person.name, "Dijkstra");
    }

    #[test]
    fn create_person() {
        let person = Person::new("Leo");
        assert_eq!(person.age, 0);
        assert_eq!(person.name, "Leo");
    }
}