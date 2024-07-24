use alloc::{vec, vec::Vec};

enum Instruction {
    Mov { dest: Register, src: Operand },
    Xor { dest: Register, src: Operand },
    Int { interrupt: u8 },
}

enum Register {
    Eax,
    Ebx,
}

enum Operand {
    Register(Register),
    Immediate(u32),
}

pub struct Assembler;

impl Assembler {
    pub fn assemble(source: &str) -> Vec<u8> {
        let mut binary = Vec::new();
        let lines = source.lines();

        for line in lines {
            if let Some(instruction) = Self::parse_line(line.trim()) {
                binary.extend(Self::encode_instruction(instruction));
            }
        }
        binary
    }
    fn parse_line(line: &str) -> Option<Instruction> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.as_slice() {
            ["mov", dest, src] => {
                let dest = Self::parse_register(dest)?;
                let src = Self::parse_operand(src)?;
                Some(Instruction::Mov { dest, src })
            }
            ["xor", dest, src] => {
                let dest = Self::parse_register(dest)?;
                let src = Self::parse_operand(src)?;
                Some(Instruction::Xor { dest, src })
            }
            ["int", interrupt] => {
                let interrupt = interrupt.parse().ok()?;
                Some(Instruction::Int { interrupt })
            }
            _ => None,
        }
    }
    fn parse_register(reg: &str) -> Option<Register> {
        match reg {
            "eax" => Some(Register::Eax),
            "ebx" => Some(Register::Ebx),
            _ => None,
        }
    }
    fn parse_operand(op: &str) -> Option<Operand> {
        if let Some(reg) = Self::parse_register(op) {
            return Some(Operand::Register(reg));
        }
        if let Ok(imm) = op.parse() {
            return Some(Operand::Immediate(imm));
        }
        None
    }
    fn encode_instruction(instruction: Instruction) -> Vec<u8> {
        match instruction {
            Instruction::Mov { dest, src } => Self::encode_mov(dest, src),
            Instruction::Xor { dest, src } => Self::encode_xor(dest, src),
            Instruction::Int { interrupt } => vec![0xcd, interrupt],
        }
    }
    fn encode_mov(dest: Register, src: Operand) -> Vec<u8> {
        match (dest, src) {
            (Register::Eax, Operand::Immediate(imm)) => {
                let mut encoding = vec![0xb8];
                encoding.extend(&imm.to_le_bytes());
                encoding
            }
            _ => unimplemented!(),
        }
    }
    fn encode_xor(dest: Register, src: Operand) -> Vec<u8> {
        match (dest, src) {
            (Register::Eax, Operand::Register(Register::Eax)) => vec![0x31, 0xc0],
            _ => unimplemented!(),
        }
    }
}
