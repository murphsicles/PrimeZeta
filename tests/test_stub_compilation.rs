// Test compilation of stub types
mod zeta {
    pub mod frontend {
        pub mod ast {
            #[derive(Debug, Clone)]
            pub enum AstNode {
                Placeholder,
            }
        }
        pub mod parser {
            pub mod top_level {
                use super::super::ast::AstNode;
                pub fn parse_zeta(_input: &str) -> Result<(&str, Vec<AstNode>), &'static str> {
                    Ok(("", vec![]))
                }
            }
        }
    }

    pub mod middle {
        pub mod resolver {
            pub mod resolver {
                #[derive(Debug, Default)]
                pub struct Resolver {
                    _placeholder: (),
                }
            }
        }

        pub mod mir {
            pub mod mir {
                #[derive(Debug, Clone)]
                pub struct Mir {
                    _placeholder: (),
                }
            }
        }

        pub mod specialization {
            pub fn is_cache_safe(_ty: &str) -> bool {
                false
            }
            pub fn lookup_specialization(_key: &str) -> Option<String> {
                None
            }
            pub fn record_specialization(_key: &str, _value: &str) -> bool {
                false
            }
        }
    }

    pub mod backend {
        pub mod codegen {
            pub mod codegen {
                #[derive(Debug)]
                pub struct LLVMCodegen {
                    _placeholder: (),
                }
            }
        }
    }

    pub mod runtime {
        pub mod actor {
            pub mod channel {
                pub struct Channel<T> {
                    _phantom: std::marker::PhantomData<T>,
                }
            }

            pub mod scheduler {
                #[derive(Debug)]
                pub struct Scheduler {
                    _placeholder: (),
                }
            }
        }
    }
}

fn main() {
    use zeta::backend::codegen::codegen::LLVMCodegen;
    use zeta::frontend::ast::AstNode;
    use zeta::frontend::parser::top_level::parse_zeta;
    use zeta::middle::mir::mir::Mir;
    use zeta::middle::resolver::resolver::Resolver;
    use zeta::middle::specialization::{
        is_cache_safe, lookup_specialization, record_specialization,
    };
    use zeta::runtime::actor::channel::Channel;
    use zeta::runtime::actor::scheduler::Scheduler;

    println!("All stub types compile successfully!");

    // Test that we can create instances
    let _ast = AstNode::Placeholder;
    let _parse_result = parse_zeta("test");
    let _resolver = Resolver::default();
    let _mir = Mir { _placeholder: () };
    let _cache_safe = is_cache_safe("i32");
    let _lookup = lookup_specialization("key");
    let _record = record_specialization("key", "value");
    let _codegen = LLVMCodegen { _placeholder: () };
    let _channel: Channel<i32> = Channel {
        _phantom: std::marker::PhantomData,
    };
    let _scheduler = Scheduler { _placeholder: () };

    println!("All instances created successfully!");
}
