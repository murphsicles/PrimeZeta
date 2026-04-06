# Week 3 Day 2 Plan - Capability-based String Operations
## April 7, 2026

### Goal
Extend string operations with identity semantics to create capability-based string operations.

### Background
We have implemented the identity type system foundation. Now we need to:
1. Extend string operations to work with identity types
2. Add identity semantics to string manipulation
3. Create capability-based string functions

### Implementation Plan

#### 1. Extend String Type with Identity Capabilities
- Modify `std::string` type to include identity capabilities
- Add identity annotations to string operations
- Create capability-aware string functions

#### 2. Capability-based String Operations
- **Read operations**: `length()`, `contains()`, `find()` - require Read capability
- **Write operations**: `append()`, `replace()`, `clear()` - require Write capability
- **Ownership operations**: `clone()`, `move()` - require Owned capability
- **Transform operations**: `to_upper()`, `to_lower()` - require Read+Write capabilities

#### 3. Identity-aware String Functions
- `string_with_capabilities(value: str, caps: [CapabilityLevel]) -> IdentityType<String>`
- `check_capability(str: IdentityType<String>, cap: CapabilityLevel) -> bool`
- `upgrade_capability(str: IdentityType<String>, new_caps: [CapabilityLevel]) -> IdentityType<String>`
- `downgrade_capability(str: IdentityType<String>, remove_caps: [CapabilityLevel]) -> IdentityType<String>`

#### 4. Integration with Existing String Operations
- Update existing string functions in `src/std/string/mod.rs`
- Add identity type checking to string operations
- Create identity-aware versions of common string operations

#### 5. Test Suite
- Create tests for capability-based string operations
- Test capability checking and enforcement
- Test identity type inference for string operations

### Files to Modify/Create

#### 1. `src/std/string/identity.rs` (NEW)
- Capability-aware string operations
- Identity type extensions for strings
- Capability checking functions

#### 2. `src/std/string/mod.rs` (MODIFY)
- Integrate identity operations
- Add identity type annotations
- Update function signatures

#### 3. `src/middle/types/identity/string_ops.rs` (NEW)
- String-specific identity operations
- Capability-based string type inference
- Identity constraints for string operations

#### 4. `tests/string_identity_tests.rs` (NEW)
- Tests for capability-based string operations
- Identity type checking for strings
- Capability enforcement tests

### Implementation Steps

#### Step 1: Create String Identity Module
```rust
// src/std/string/identity.rs
pub mod identity {
    use crate::middle::types::identity::{CapabilityLevel, IdentityType};
    
    pub struct StringWithIdentity {
        value: String,
        identity: IdentityType,
    }
    
    impl StringWithIdentity {
        pub fn new(value: String, caps: Vec<CapabilityLevel>) -> Self {
            let identity = IdentityType::with_value(value.clone(), caps);
            Self { value, identity }
        }
        
        pub fn has_capability(&self, cap: CapabilityLevel) -> bool {
            self.identity.has_capability(cap)
        }
        
        // ... capability-based operations
    }
}
```

#### Step 2: Extend String Operations
```rust
// src/std/string/mod.rs
pub fn length(s: &IdentityType<String>) -> usize {
    // Check Read capability
    if !s.has_capability(CapabilityLevel::Read) {
        panic!("String requires Read capability for length()");
    }
    s.value().len()
}

pub fn append(s: &mut IdentityType<String>, other: &str) {
    // Check Write capability
    if !s.has_capability(CapabilityLevel::Write) {
        panic!("String requires Write capability for append()");
    }
    // ... implementation
}
```

#### Step 3: Add Identity Type Inference
```rust
// src/middle/types/identity/string_ops.rs
pub fn infer_string_operation_capabilities(
    op: &str,
    args: &[&IdentityType]
) -> Result<Vec<CapabilityLevel>, String> {
    match op {
        "length" | "contains" | "find" => Ok(vec![CapabilityLevel::Read]),
        "append" | "replace" | "clear" => Ok(vec![CapabilityLevel::Write]),
        "clone" | "move" => Ok(vec![CapabilityLevel::Owned]),
        "to_upper" | "to_lower" => Ok(vec![CapabilityLevel::Read, CapabilityLevel::Write]),
        _ => Err(format!("Unknown string operation: {}", op)),
    }
}
```

#### Step 4: Create Tests
```rust
// tests/string_identity_tests.rs
#[test]
fn test_string_with_read_capability() {
    let s = IdentityType::with_value(
        "hello".to_string(),
        vec![CapabilityLevel::Read],
    );
    
    // Should work: length requires Read
    assert_eq!(s.value().len(), 5);
    
    // Should panic: append requires Write
    // (test would expect panic)
}

#[test]
fn test_capability_upgrade() {
    let s = IdentityType::with_value(
        "hello".to_string(),
        vec![CapabilityLevel::Read],
    );
    
    let upgraded = s.upgrade(vec![CapabilityLevel::Write]);
    assert!(upgraded.has_capability(CapabilityLevel::Read));
    assert!(upgraded.has_capability(CapabilityLevel::Write));
}
```

### Expected Outcomes
1. String operations now require appropriate capabilities
2. Compile-time checking of string capability requirements
3. Identity-aware string manipulation
4. Comprehensive test coverage for capability-based string operations

### Success Criteria
- All existing string tests still pass (with identity annotations)
- New identity string tests pass
- Compiler can infer required capabilities for string operations
- Capability violations are caught at compile time (or runtime with clear errors)

### Timeline
- **Morning**: Implement string identity module and basic operations
- **Afternoon**: Integrate with existing string functions and add tests
- **Evening**: Test and debug, create documentation

### Dependencies
- Identity type system (completed)
- Existing string implementation
- Type inference system

### Risks
1. Breaking existing string functionality
2. Performance overhead from capability checking
3. Complex type inference for identity operations

### Mitigation
1. Add identity annotations as optional features initially
2. Use compile-time capability checking where possible
3. Start with simple cases and expand gradually