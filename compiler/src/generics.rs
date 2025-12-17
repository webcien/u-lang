// generics.rs — Generics implementation for U Language
// MIT License — Copyright (c) 2025 Webcien and U contributors

use crate::parser::*;
use std::collections::HashMap;

/// Represents a monomorphized instance of a generic function or type
#[derive(Debug, Clone)]
pub struct MonomorphizedInstance {
    pub original_name: String,
    pub mangled_name: String,
    pub type_params: Vec<String>,
    pub concrete_types: Vec<String>,
}

/// Generics engine for monomorphization
pub struct GenericsEngine {
    /// Map from (original_name, concrete_types) to mangled_name
    instances: HashMap<(String, Vec<String>), String>,
    /// Counter for generating unique names
    counter: usize,
}

impl GenericsEngine {
    pub fn new() -> Self {
        GenericsEngine {
            instances: HashMap::new(),
            counter: 0,
        }
    }

    /// Monomorphize a generic function
    pub fn monomorphize_function(
        &mut self,
        func_name: &str,
        type_params: &[String],
        concrete_types: &[String],
    ) -> String {
        let key = (func_name.to_string(), concrete_types.to_vec());
        
        if let Some(mangled) = self.instances.get(&key) {
            return mangled.clone();
        }

        // Generate mangled name: func_name_Type1_Type2_...
        let mangled = if concrete_types.is_empty() {
            func_name.to_string()
        } else {
            format!("{}_{}", func_name, concrete_types.join("_"))
        };

        self.instances.insert(key, mangled.clone());
        mangled
    }

    /// Monomorphize a generic type
    pub fn monomorphize_type(
        &mut self,
        type_name: &str,
        type_params: &[String],
        concrete_types: &[String],
    ) -> String {
        let key = (type_name.to_string(), concrete_types.to_vec());
        
        if let Some(mangled) = self.instances.get(&key) {
            return mangled.clone();
        }

        // Generate mangled name: TypeName_Type1_Type2_...
        let mangled = if concrete_types.is_empty() {
            type_name.to_string()
        } else {
            format!("{}_{}", type_name, concrete_types.join("_"))
        };

        self.instances.insert(key, mangled.clone());
        mangled
    }

    /// Get all monomorphized instances
    pub fn get_instances(&self) -> Vec<MonomorphizedInstance> {
        self.instances
            .iter()
            .map(|((name, types), mangled)| MonomorphizedInstance {
                original_name: name.clone(),
                mangled_name: mangled.clone(),
                type_params: vec![],
                concrete_types: types.clone(),
            })
            .collect()
    }
}

/// Trait bounds checker
pub struct TraitBoundsChecker {
    /// Map from type to implemented traits
    type_traits: HashMap<String, Vec<String>>,
}

impl TraitBoundsChecker {
    pub fn new() -> Self {
        let mut checker = TraitBoundsChecker {
            type_traits: HashMap::new(),
        };

        // Register built-in trait implementations
        checker.register_trait("i32", "Clone");
        checker.register_trait("i64", "Clone");
        checker.register_trait("f32", "Clone");
        checker.register_trait("f64", "Clone");
        checker.register_trait("bool", "Clone");
        checker.register_trait("str", "Clone");

        checker
    }

    pub fn register_trait(&mut self, type_name: &str, trait_name: &str) {
        self.type_traits
            .entry(type_name.to_string())
            .or_insert_with(Vec::new)
            .push(trait_name.to_string());
    }

    pub fn check_trait_bound(&self, type_name: &str, trait_name: &str) -> bool {
        if let Some(traits) = self.type_traits.get(type_name) {
            traits.contains(&trait_name.to_string())
        } else {
            false
        }
    }

    pub fn check_all_bounds(
        &self,
        concrete_types: &[String],
        trait_bounds: &[(String, String)],
    ) -> Result<(), String> {
        for (type_param, trait_name) in trait_bounds {
            // Find the concrete type for this type parameter
            // For simplicity, we assume the first concrete type corresponds to the first type parameter
            if let Some(concrete_type) = concrete_types.first() {
                if !self.check_trait_bound(concrete_type, trait_name) {
                    return Err(format!(
                        "Type '{}' does not implement trait '{}'",
                        concrete_type, trait_name
                    ));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monomorphize_function() {
        let mut engine = GenericsEngine::new();
        
        let mangled = engine.monomorphize_function("Vec_push", &["T".to_string()], &["i32".to_string()]);
        assert_eq!(mangled, "Vec_push_i32");

        let mangled2 = engine.monomorphize_function("Vec_push", &["T".to_string()], &["String".to_string()]);
        assert_eq!(mangled2, "Vec_push_String");
    }

    #[test]
    fn test_trait_bounds() {
        let checker = TraitBoundsChecker::new();
        
        assert!(checker.check_trait_bound("i32", "Clone"));
        assert!(!checker.check_trait_bound("i32", "Display"));
    }
}
