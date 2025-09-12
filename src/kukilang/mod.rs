use alloc::vec::Vec;

use crate::{info, println};

pub struct KukiLangInterpreter {
    code: &'static str,
}

impl KukiLangInterpreter {
    pub fn new(code: &'static str) -> Self {
        //&mut self,
        info!("Initializing KukiLang!");
        // self.code = code;
        Self { code }
    }
    pub fn check_kl_presence(self) -> bool {
        let code = self.code;
        let code_lines: Vec<&str> = code.lines().collect();
        if code_lines[0] == "#![kukilang]" {
            true
        } else {
            false
        }
    }
}
