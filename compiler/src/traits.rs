// traits.rs — U v0.8 Trait System
// MIT License — Copyright (c) 2025 Webcien and U contributors
//
// Trait and interface system for U
// Traits define shared behavior that types can implement
// Supports:
// - Trait definition with method signatures
// - Trait implementation for types
// - Trait bounds in function signatures
// - Default implementations (v0.9+)

use std::collections::HashMap;
use std::fmt;

/// Type representation for trait system
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TraitType {
    I32,
    Str,
    Bool,
    Custom(String),
}

impl fmt::Display for TraitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TraitType::I32 => write!(f, "i32"),
            TraitType::Str => write!(f, "str"),
            TraitType::Bool => write!(f, "bool"),
            TraitType::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// Method signature in a trait
#[derive(Debug, Clone)]
pub struct MethodSignature {
    pub name: String,
    pub params: Vec<(String, TraitType)>,
    pub return_type: Option<TraitType>,
}

impl MethodSignature {
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: Vec::new(),
            return_type: None,
        }
    }

    pub fn with_param(mut self, name: String, ty: TraitType) -> Self {
        self.params.push((name, ty));
        self
    }

    pub fn with_return(mut self, ty: TraitType) -> Self {
        self.return_type = Some(ty);
        self
    }
}

/// Trait definition
#[derive(Debug, Clone)]
pub struct Trait {
    pub name: String,
    pub methods: Vec<MethodSignature>,
    pub documentation: Option<String>,
}

impl Trait {
    pub fn new(name: String) -> Self {
        Self {
            name,
            methods: Vec::new(),
            documentation: None,
        }
    }

    pub fn with_method(mut self, method: MethodSignature) -> Self {
        self.methods.push(method);
        self
    }

    pub fn with_doc(mut self, doc: String) -> Self {
        self.documentation = Some(doc);
        self
    }

    pub fn add_method(&mut self, method: MethodSignature) {
        self.methods.push(method);
    }

    pub fn find_method(&self, name: &str) -> Option<&MethodSignature> {
        self.methods.iter().find(|m| m.name == name)
    }
}

/// Trait implementation for a type
#[derive(Debug, Clone)]
pub struct TraitImpl {
    pub trait_name: String,
    pub type_name: String,
    pub methods: HashMap<String, String>, // method_name -> implementation
}

impl TraitImpl {
    pub fn new(trait_name: String, type_name: String) -> Self {
        Self {
            trait_name,
            type_name,
            methods: HashMap::new(),
        }
    }

    pub fn add_method(&mut self, name: String, implementation: String) {
        self.methods.insert(name, implementation);
    }

    pub fn get_method(&self, name: &str) -> Option<&str> {
        self.methods.get(name).map(|s| s.as_str())
    }
}

/// Trait registry for managing all traits and implementations
pub struct TraitRegistry {
    traits: HashMap<String, Trait>,
    implementations: Vec<TraitImpl>,
}

impl TraitRegistry {
    pub fn new() -> Self {
        Self {
            traits: HashMap::new(),
            implementations: Vec::new(),
        }
    }

    /// Register a new trait
    pub fn register_trait(&mut self, trait_def: Trait) -> Result<(), String> {
        if self.traits.contains_key(&trait_def.name) {
            return Err(format!("Trait '{}' already defined", trait_def.name));
        }
        self.traits.insert(trait_def.name.clone(), trait_def);
        Ok(())
    }

    /// Register a trait implementation
    pub fn register_impl(&mut self, impl_def: TraitImpl) -> Result<(), String> {
        if !self.traits.contains_key(&impl_def.trait_name) {
            return Err(format!("Trait '{}' not found", impl_def.trait_name));
        }

        let trait_def = &self.traits[&impl_def.trait_name];

        // Verify all trait methods are implemented
        for method in &trait_def.methods {
            if !impl_def.methods.contains_key(&method.name) {
                return Err(format!(
                    "Method '{}' not implemented for type '{}'",
                    method.name, impl_def.type_name
                ));
            }
        }

        self.implementations.push(impl_def);
        Ok(())
    }

    /// Get a trait definition
    pub fn get_trait(&self, name: &str) -> Option<&Trait> {
        self.traits.get(name)
    }

    /// Check if a type implements a trait
    pub fn implements(&self, type_name: &str, trait_name: &str) -> bool {
        self.implementations
            .iter()
            .any(|impl_def| impl_def.type_name == type_name && impl_def.trait_name == trait_name)
    }

    /// Get implementation of a trait for a type
    pub fn get_impl(&self, type_name: &str, trait_name: &str) -> Option<&TraitImpl> {
        self.implementations
            .iter()
            .find(|impl_def| impl_def.type_name == type_name && impl_def.trait_name == trait_name)
    }

    /// List all traits
    pub fn list_traits(&self) -> Vec<&str> {
        self.traits.keys().map(|s| s.as_str()).collect()
    }

    /// List all implementations
    pub fn list_implementations(&self) -> Vec<(&str, &str)> {
        self.implementations
            .iter()
            .map(|impl_def| (impl_def.type_name.as_str(), impl_def.trait_name.as_str()))
            .collect()
    }
}

/// Built-in traits for common operations
pub fn create_builtin_traits() -> TraitRegistry {
    let mut registry = TraitRegistry::new();

    // Display trait: types that can be converted to string
    let mut display_trait = Trait::new("Display".to_string());
    display_trait.add_method(MethodSignature::new("to_string".to_string()));
    let _ = registry.register_trait(display_trait);

    // Clone trait: types that can be cloned
    let mut clone_trait = Trait::new("Clone".to_string());
    clone_trait.add_method(MethodSignature::new("clone".to_string()));
    let _ = registry.register_trait(clone_trait);

    // Eq trait: types that support equality
    let mut eq_trait = Trait::new("Eq".to_string());
    eq_trait.add_method(
        MethodSignature::new("eq".to_string())
            .with_param("other".to_string(), TraitType::Custom("Self".to_string()))
            .with_return(TraitType::Bool),
    );
    let _ = registry.register_trait(eq_trait);

    // Ord trait: types that support ordering
    let mut ord_trait = Trait::new("Ord".to_string());
    ord_trait.add_method(
        MethodSignature::new("cmp".to_string())
            .with_param("other".to_string(), TraitType::Custom("Self".to_string()))
            .with_return(TraitType::Custom("Ordering".to_string())),
    );
    let _ = registry.register_trait(ord_trait);

    registry
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_creation() {
        let trait_def = Trait::new("Display".to_string());
        assert_eq!(trait_def.name, "Display");
        assert_eq!(trait_def.methods.len(), 0);
    }

    #[test]
    fn test_trait_with_method() {
        let method = MethodSignature::new("to_string".to_string());
        let trait_def = Trait::new("Display".to_string()).with_method(method);
        assert_eq!(trait_def.methods.len(), 1);
    }

    #[test]
    fn test_trait_registry() {
        let mut registry = TraitRegistry::new();
        let trait_def = Trait::new("Display".to_string());
        assert!(registry.register_trait(trait_def).is_ok());
        assert!(registry.get_trait("Display").is_some());
    }

    #[test]
    fn test_trait_impl() {
        let mut registry = TraitRegistry::new();
        let trait_def = Trait::new("Display".to_string())
            .with_method(MethodSignature::new("to_string".to_string()));
        registry.register_trait(trait_def).unwrap();

        let mut impl_def = TraitImpl::new("Display".to_string(), "i32".to_string());
        impl_def.add_method("to_string".to_string(), "impl".to_string());
        assert!(registry.register_impl(impl_def).is_ok());
        assert!(registry.implements("i32", "Display"));
    }

    #[test]
    fn test_builtin_traits() {
        let registry = create_builtin_traits();
        assert!(registry.get_trait("Display").is_some());
        assert!(registry.get_trait("Clone").is_some());
        assert!(registry.get_trait("Eq").is_some());
        assert!(registry.get_trait("Ord").is_some());
    }
}
