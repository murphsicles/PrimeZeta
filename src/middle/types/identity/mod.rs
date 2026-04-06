//! # Identity Type System
//!
//! String-based identity types for capability-based programming.
//! Treats strings as first-class identities with compile-time verification.

use std::collections::HashMap;
use std::fmt;

pub mod string_ops;
pub mod inference;

/// Identity capability level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CapabilityLevel {
    /// No capabilities - immutable data only
    Immutable,
    /// Read capability
    Read,
    /// Write capability
    Write,
    /// Execute capability
    Execute,
    /// Full ownership capabilities
    Owned,
}

impl fmt::Display for CapabilityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CapabilityLevel::Immutable => write!(f, "immutable"),
            CapabilityLevel::Read => write!(f, "read"),
            CapabilityLevel::Write => write!(f, "write"),
            CapabilityLevel::Execute => write!(f, "execute"),
            CapabilityLevel::Owned => write!(f, "owned"),
        }
    }
}

/// Identity type - represents a string with specific capabilities
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdentityType {
    /// The identity string value (if known at compile time)
    pub value: Option<String>,
    /// Required capabilities for this identity
    pub capabilities: Vec<CapabilityLevel>,
    /// Whether this identity can be delegated
    pub delegatable: bool,
    /// Constraints on this identity (e.g., must match pattern)
    pub constraints: Vec<IdentityConstraint>,
}

impl IdentityType {
    /// Create a new identity type with unknown value
    pub fn new(capabilities: Vec<CapabilityLevel>) -> Self {
        Self {
            value: None,
            capabilities,
            delegatable: false,
            constraints: Vec::new(),
        }
    }

    /// Create a new identity type with known value
    pub fn with_value(value: String, capabilities: Vec<CapabilityLevel>) -> Self {
        Self {
            value: Some(value),
            capabilities,
            delegatable: false,
            constraints: Vec::new(),
        }
    }

    /// Check if this identity has a specific capability
    pub fn has_capability(&self, level: CapabilityLevel) -> bool {
        self.capabilities.contains(&level)
    }

    /// Get all capabilities
    pub fn capabilities(&self) -> &[CapabilityLevel] {
        &self.capabilities
    }

    /// Get the value if known
    pub fn value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    /// Upgrade capabilities (add new capabilities)
    pub fn upgrade(mut self, new_caps: Vec<CapabilityLevel>) -> Self {
        for cap in new_caps {
            if !self.capabilities.contains(&cap) {
                self.capabilities.push(cap);
            }
        }
        self
    }

    /// Downgrade capabilities (remove capabilities)
    pub fn downgrade(mut self, remove_caps: Vec<CapabilityLevel>) -> Self {
        self.capabilities.retain(|cap| !remove_caps.contains(cap));
        self
    }

    /// Check if this identity can be used in place of another
    pub fn can_substitute(&self, other: &IdentityType) -> bool {
        // An identity can substitute another if it has at least the same capabilities
        for cap in &other.capabilities {
            if !self.has_capability(*cap) {
                return false;
            }
        }
        
        // Check constraints
        for constraint in &other.constraints {
            if !self.satisfies_constraint(constraint) {
                return false;
            }
        }
        
        true
    }

    /// Check if this identity satisfies a constraint
    fn satisfies_constraint(&self, constraint: &IdentityConstraint) -> bool {
        match constraint {
            IdentityConstraint::Pattern(pattern) => {
                if let Some(value) = &self.value {
                    // Simple pattern matching - could be extended with regex
                    value.contains(pattern)
                } else {
                    // Unknown value, assume it might match
                    true
                }
            }
            IdentityConstraint::MaxLength(max) => {
                if let Some(value) = &self.value {
                    value.len() <= *max
                } else {
                    // Unknown value, assume it might satisfy
                    true
                }
            }
            IdentityConstraint::MinLength(min) => {
                if let Some(value) = &self.value {
                    value.len() >= *min
                } else {
                    // Unknown value, assume it might satisfy
                    true
                }
            }
        }
    }
}

impl fmt::Display for IdentityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(value) = &self.value {
            write!(f, "identity(\"{}\")", value)?;
        } else {
            write!(f, "identity")?;
        }
        
        if !self.capabilities.is_empty() {
            write!(f, "[")?;
            for (i, cap) in self.capabilities.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", cap)?;
            }
            write!(f, "]")?;
        }
        
        if self.delegatable {
            write!(f, "+delegatable")?;
        }
        
        Ok(())
    }
}

/// Constraints on identity values
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IdentityConstraint {
    /// Must match pattern (substring)
    Pattern(String),
    /// Maximum length
    MaxLength(usize),
    /// Minimum length
    MinLength(usize),
}

/// Identity context for tracking identity relationships
#[derive(Debug, Clone, Default)]
pub struct IdentityContext {
    /// Known identities and their types
    identities: HashMap<String, IdentityType>,
    /// Subtyping relationships between identities
    subtyping: Vec<(String, String)>, // (subtype, supertype)
}

impl IdentityContext {
    /// Create a new empty identity context
    pub fn new() -> Self {
        Self {
            identities: HashMap::new(),
            subtyping: Vec::new(),
        }
    }

    /// Add a known identity
    pub fn add_identity(&mut self, name: String, identity_type: IdentityType) {
        self.identities.insert(name, identity_type);
    }

    /// Get an identity type by name
    pub fn get_identity(&self, name: &str) -> Option<&IdentityType> {
        self.identities.get(name)
    }

    /// Add a subtyping relationship
    pub fn add_subtyping(&mut self, subtype: String, supertype: String) {
        self.subtyping.push((subtype, supertype));
    }

    /// Check if one identity is a subtype of another
    pub fn is_subtype(&self, subtype: &str, supertype: &str) -> bool {
        // Direct relationship
        if self.subtyping.contains(&(subtype.to_string(), supertype.to_string())) {
            return true;
        }
        
        // Transitive closure (simplified)
        for (sub, sup) in &self.subtyping {
            if sub == subtype && self.is_subtype(sup, supertype) {
                return true;
            }
        }
        
        false
    }

    /// Unify two identity types
    pub fn unify(&self, t1: &IdentityType, t2: &IdentityType) -> Option<IdentityType> {
        // If both have values, they must match
        if let (Some(v1), Some(v2)) = (&t1.value, &t2.value) {
            if v1 != v2 {
                return None;
            }
        }
        
        // Take the union of capabilities
        let mut capabilities = t1.capabilities.clone();
        for cap in &t2.capabilities {
            if !capabilities.contains(cap) {
                capabilities.push(*cap);
            }
        }
        
        // Take the intersection of delegatable
        let delegatable = t1.delegatable && t2.delegatable;
        
        // Combine constraints
        let mut constraints = t1.constraints.clone();
        constraints.extend(t2.constraints.clone());
        
        // Use the known value if either has one
        let value = t1.value.clone().or_else(|| t2.value.clone());
        
        Some(IdentityType {
            value,
            capabilities,
            delegatable,
            constraints,
        })
    }
}

/// Identity operation - represents operations on identity types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IdentityOp {
    /// Create a new identity
    Create,
    /// Verify an identity
    Verify,
    /// Delegate an identity
    Delegate,
    /// Revoke an identity
    Revoke,
    /// Combine identities
    Combine,
    /// Split an identity
    Split,
}

impl fmt::Display for IdentityOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdentityOp::Create => write!(f, "create"),
            IdentityOp::Verify => write!(f, "verify"),
            IdentityOp::Delegate => write!(f, "delegate"),
            IdentityOp::Revoke => write!(f, "revoke"),
            IdentityOp::Combine => write!(f, "combine"),
            IdentityOp::Split => write!(f, "split"),
        }
    }
}