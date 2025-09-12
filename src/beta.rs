use crate::{kukilang::KukiLangInterpreter, println};

pub fn test_kukilang_code() {
    let code = "
        a to to je zbytok
        ";
    let interpreter = KukiLangInterpreter::new(code);
    let presnece = interpreter.check_kl_presence();
    println!("PRESENCE: {presnece}");
}
