// MIR System Smoke Tests
// Basic smoke tests for the Middle Intermediate Representation system

use zetac::middle::mir::mir::{Mir, MirExpr, MirStmt};

#[test]
fn test_mir_smoke_creation() {
    // Test basic MIR creation
    let mut mir = Mir::default();
    
    // Add some expressions
    let expr1_id = 1;
    let expr2_id = 2;
    mir.exprs.insert(expr1_id, MirExpr::Lit(42));
    mir.exprs.insert(expr2_id, MirExpr::Lit(10));
    
    // Add a statement
    mir.stmts.push(MirStmt::Assign {
        lhs: expr1_id,
        rhs: expr2_id,
    });
    
    // Add a return statement
    mir.stmts.push(MirStmt::Return {
        val: expr1_id,
    });
    
    // Verify MIR structure
    assert_eq!(mir.exprs.len(), 2, "Should have 2 expressions");
    assert_eq!(mir.stmts.len(), 2, "Should have 2 statements");
    
    // Verify expression values
    if let Some(MirExpr::Lit(value)) = mir.exprs.get(&expr1_id) {
        assert_eq!(*value, 42, "Expression 1 should have value 42");
    } else {
        panic!("Expression 1 should be a literal");
    }
    
    if let Some(MirExpr::Lit(value)) = mir.exprs.get(&expr2_id) {
        assert_eq!(*value, 10, "Expression 2 should have value 10");
    } else {
        panic!("Expression 2 should be a literal");
    }
}

#[test]
fn test_mir_smoke_expression_types() {
    // Test different MIR expression types
    let mut mir = Mir::default();
    
    // Test literal expression
    let lit_id = mir.add_expr(MirExpr::Lit(100));
    assert!(matches!(mir.exprs.get(&lit_id), Some(MirExpr::Lit(100))));
    
    // Test variable expression
    let var_id = mir.add_expr(MirExpr::Var(1));
    assert!(matches!(mir.exprs.get(&var_id), Some(MirExpr::Var(1))));
    
    // Test binary operation
    let binop_id = mir.add_expr(MirExpr::BinaryOp {
        op: "+".to_string(),
        lhs: lit_id,
        rhs: var_id,
    });
    
    if let Some(MirExpr::BinaryOp { op, lhs, rhs }) = mir.exprs.get(&binop_id) {
        assert_eq!(op, "+", "Binary operation should be '+'");
        assert_eq!(*lhs, lit_id, "Left operand should be literal");
        assert_eq!(*rhs, var_id, "Right operand should be variable");
    } else {
        panic!("Should be a binary operation");
    }
}

#[test]
fn test_mir_smoke_statement_types() {
    // Test different MIR statement types
    let mut mir = Mir::default();
    
    // Create some expressions first
    let expr1_id = mir.add_expr(MirExpr::Lit(1));
    let expr2_id = mir.add_expr(MirExpr::Lit(2));
    let expr3_id = mir.add_expr(MirExpr::Lit(3));
    
    // Test assignment statement
    mir.stmts.push(MirStmt::Assign {
        lhs: expr1_id,
        rhs: expr2_id,
    });
    
    // Test return statement
    mir.stmts.push(MirStmt::Return {
        val: expr3_id,
    });
    
    // Test void call statement (function call without return value)
    mir.stmts.push(MirStmt::VoidCall {
        func: "print".to_string(),
        args: vec![expr1_id],
    });
    
    // Verify statements
    assert_eq!(mir.stmts.len(), 3, "Should have 3 statements");
    
    // Check assignment statement
    if let MirStmt::Assign { lhs, rhs } = &mir.stmts[0] {
        assert_eq!(*lhs, expr1_id, "Assignment lhs should be expr1");
        assert_eq!(*rhs, expr2_id, "Assignment rhs should be expr2");
    } else {
        panic!("First statement should be assignment");
    }
    
    // Check return statement
    if let MirStmt::Return { val } = &mir.stmts[1] {
        assert_eq!(*val, expr3_id, "Return value should be expr3");
    } else {
        panic!("Second statement should be return");
    }
    
    // Check void call statement
    if let MirStmt::VoidCall { func, args } = &mir.stmts[2] {
        assert_eq!(func, "print", "Function name should be 'print'");
        assert_eq!(args.len(), 1, "Should have 1 argument");
        assert_eq!(args[0], expr1_id, "Argument should be expr1");
    } else {
        panic!("Third statement should be void call");
    }
}

#[test]
fn test_mir_smoke_optimization_interface() {
    // Test that optimization functions can be called
    use zetac::middle::optimization::{dead_code_elimination, constant_folding};
    
    let mut mir = Mir::default();
    
    // Create a simple MIR
    let expr1_id = mir.add_expr(MirExpr::Lit(42));
    let expr2_id = mir.add_expr(MirExpr::Lit(10));
    mir.stmts.push(MirStmt::Assign {
        lhs: expr1_id,
        rhs: expr2_id,
    });
    mir.stmts.push(MirStmt::Return {
        val: expr1_id,
    });
    
    // Test that optimization functions can be called without panicking
    dead_code_elimination(&mut mir);
    constant_folding(&mut mir);
    
    // If we get here, optimizations ran without panic
    println!("Optimizations completed on MIR with {} expressions and {} statements", 
             mir.exprs.len(), mir.stmts.len());
}

#[test]
fn test_mir_smoke_complex_structure() {
    // Test creating a more complex MIR structure
    let mut mir = Mir::default();
    
    // Create expressions for a simple computation: (a + b) * c
    let a_id = mir.add_expr(MirExpr::Lit(5));
    let b_id = mir.add_expr(MirExpr::Lit(3));
    let add_id = mir.add_expr(MirExpr::BinaryOp {
        op: "+".to_string(),
        lhs: a_id,
        rhs: b_id,
    });
    
    let c_id = mir.add_expr(MirExpr::Lit(2));
    let mul_id = mir.add_expr(MirExpr::BinaryOp {
        op: "*".to_string(),
        lhs: add_id,
        rhs: c_id,
    });
    
    // Store result and return it
    let result_id = mir.add_expr(MirExpr::Var(100)); // Some variable ID
    mir.stmts.push(MirStmt::Assign {
        lhs: result_id,
        rhs: mul_id,
    });
    mir.stmts.push(MirStmt::Return {
        val: result_id,
    });
    
    // Verify structure
    assert_eq!(mir.exprs.len(), 5, "Should have 5 expressions");
    assert_eq!(mir.stmts.len(), 2, "Should have 2 statements");
    
    // Verify the computation structure
    if let Some(MirExpr::BinaryOp { op, lhs, rhs }) = mir.exprs.get(&mul_id) {
        assert_eq!(op, "*", "Should be multiplication");
        
        // Check that lhs is the addition
        if let Some(MirExpr::BinaryOp { op: add_op, lhs: a, rhs: b }) = mir.exprs.get(lhs) {
            assert_eq!(add_op, "+", "Nested operation should be addition");
            assert_eq!(*a, a_id, "Addition lhs should be a");
            assert_eq!(*b, b_id, "Addition rhs should be b");
        } else {
            panic!("Multiplication lhs should be addition");
        }
        
        assert_eq!(*rhs, c_id, "Multiplication rhs should be c");
    } else {
        panic!("Should have multiplication at root");
    }
}