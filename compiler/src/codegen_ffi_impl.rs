// codegen_ffi_impl.rs — FFI code generation for U v0.9
// This file contains the implementation of generate_extern_block
// To be added to c.rs

use crate::parser::{ExternBlock, ExternFunction, Type};

impl CGenerator {
    fn generate_extern_block(&mut self, block: ExternBlock) {
        self.emit(&format!("// extern \"{}\" block", block.abi));
        for func in block.functions {
            self.generate_extern_function(&func);
        }
        self.emitln();
    }
    
    fn generate_extern_function(&mut self, func: &ExternFunction) {
        // Generate C function declaration
        let return_type = if let Some(ref ty) = func.return_type {
            self.type_to_c(ty)
        } else {
            "void".to_string()
        };
        
        let params = if func.params.is_empty() {
            if func.is_variadic {
                "...".to_string()
            } else {
                "void".to_string()
            }
        } else {
            let param_list = func.params.iter()
                .map(|(name, ty)| format!("{} {}", self.type_to_c(ty), name))
                .collect::<Vec<_>>()
                .join(", ");
            
            if func.is_variadic {
                format!("{}, ...", param_list)
            } else {
                param_list
            }
        };
        
        self.emit(&format!("extern {} {}({});", return_type, func.name, params));
    }
}
