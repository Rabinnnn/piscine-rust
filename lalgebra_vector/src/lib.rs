// Import the `Scalar` trait from the `lalgebra_scalar` crate
use lalgebra_scalar::Scalar;

// Define a generic `Vector` struct that holds a vector of elements of type `T`
// where `T` must implement the `Scalar` trait and its associated `Item` type must be `T` itself
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar<Item = T>>(pub Vec<T>);

// Import the `Add` trait from the standard library to allow vector addition
use std::ops::Add;

// Implement the `Add` trait for `Vector<T>` where `T` supports `Scalar` and addition
impl<T: Scalar<Item = T> + Add<Output = T>> Add<Self> for Vector<T> {
    // The output of the addition is an `Option<Self>`, to handle mismatched vector lengths
    type Output = Option<Self>;

    // Define the `add` method for vector addition
    fn add(self, other: Self) -> Self::Output {
        // If vectors are of unequal length, return `None`
        if self.0.len() != other.0.len() {
            return None;
        }

        // Perform element-wise addition and collect the result into a new `Vector<T>`
        let result: Vector<T> = Vector(
            self.0
                .iter()                     // Iterate over elements of the first vector
                .zip(other.0.iter())       // Pair each element with the corresponding element in the second vector
                .map(|(x, y)| x.clone() + y.clone()) // Clone and add each pair of elements
                .collect(),                // Collect results into a vector
        );
        
        // Return the resulting vector wrapped in `Some`
        Some(result)
    }
}

// Implement additional methods for `Vector<T>` where `T` supports `Scalar` and multiplication
// and the result of multiplication can be summed up using `Sum`
impl<T: Scalar<Item = T> + std::iter::Sum<<T as std::ops::Mul>::Output>> Vector<T> {
    // Constructor method to create a new, empty vector
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    // Method to compute the dot product between two vectors
    pub fn dot(&self, other: &Self) -> Option<T> {
        // Return `None` if vector lengths do not match
        if self.0.len() != other.0.len() {
            return None;
        }

        // Compute the dot product by multiplying each pair of elements and summing the results
        let result = self
            .0
            .iter()                          // Iterate over elements of the first vector
            .zip(other.0.iter())             // Pair each element with corresponding element in the second vector
            .map(|(x, y)| x.clone() * y.clone()) // Clone and multiply each pair
            .sum();                          // Sum the results

        // Return the dot product wrapped in `Some`
        Some(result)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_product() {
        let vector_1: Vector<i64> = Vector(vec![1, 3, -5]);
        let vector_2: Vector<i64> = Vector(vec![4, -2, -1]);
        let expected: i64 = 3;
        assert_eq!(vector_1.dot(&vector_2), Some(expected));

        let vector_1: Vector<i64> = Vector(vec![1, 3, -5]);
        let vector_2: Vector<i64> = Vector(vec![4, -2]);
        assert_eq!(vector_1.dot(&vector_2), None);
    }
}