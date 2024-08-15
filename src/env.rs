use std::collections::HashMap;

/// Environment is a map of variable names to their values.
pub(crate) type Env = HashMap<String, i64>;

/// Adds a name to the environment, assigning it a new slot number.
///
/// ## Parameters:
/// - `name`: The name to add to the environment.
/// - `env`: A mutable reference to the environment, a `HashMap` of `(String, i64)` pairs.
///
/// ## Returns:
/// A tuple containing the updated environment and the assigned slot number.
pub(crate) fn add(name: String, env: &mut Env) -> i64 {
    let slot = (env.len() as i64) + 1;  // Calculate the new slot number
    env.insert(name, slot);             // Insert the name with the slot into the environment
    slot                                // Return the slot
}

#[cfg(test)]
mod tests {
    use expectest::prelude::*;
    use super::*;

    #[test]
    fn test_add() {
        let mut env = Env::new();  // Create a new environment
        let slot = add("x".to_string(), &mut env);  // Add the variable "x" to the environment
        expect!(slot).to(be_equal_to(1));  // The first variable should have slot 1
        let slot = add("y".to_string(), &mut env);  // Add the variable "y" to the environment
        expect!(slot).to(be_equal_to(2));  // The second variable should have slot 2
    }
}
