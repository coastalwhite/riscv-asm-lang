use riscv_encoding::rv32i::*;

use crate::{AsmArgument, AsmInstruction, AsmRegister};

impl AsmInstruction for Lui {
    #[inline]
    fn verb(&self) -> &'static str {
        "lui"
    }
    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::Register(AsmRegister::Gp(self.rd())),
            AsmArgument::Immediate(i64::from(self.imm()) >> 12),
        ]
    }
}

impl AsmInstruction for AuiPc {
    #[inline]
    fn verb(&self) -> &'static str {
        "auipc"
    }
    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::Register(AsmRegister::Gp(self.rd())),
            AsmArgument::Immediate(i64::from(self.imm()) >> 12),
        ]
    }
}

impl AsmInstruction for Jal {
    #[inline]
    fn verb(&self) -> &'static str {
        match self.rd() {
            0 => "j",
            _ => "jal",
        }
    }
    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        match self.rd() {
            0 | 1 => vec![AsmArgument::Immediate(i64::from(self.imm()))],
            _ => vec![
                AsmArgument::Register(AsmRegister::Gp(self.rd())),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
        }
    }
}

impl AsmInstruction for JalR {
    #[inline]
    fn verb(&self) -> &'static str {
        match (self.imm(), self.rd(), self.rs1()) {
            (0, 1, 1) => "ret",
            (0, 0, _) => "jr",
            _ => "jalr",
        }
    }
    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        match (self.imm(), self.rd(), self.rs1()) {
            (0, 1, 1) => vec![],
            (0, 0 | 1, _) => vec![AsmArgument::Register(AsmRegister::Gp(self.rs1()))],
            _ => vec![
                AsmArgument::Register(AsmRegister::Gp(self.rd())),
                AsmArgument::Register(AsmRegister::Gp(self.rs1())),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
        }
    }
}

impl AsmInstruction for Beq {
    #[inline]
    fn verb(&self) -> &'static str {
        match (self.rs1(), self.rs2()) {
            (0, _) | (_, 0) => "beqz",
            _ => "beq",
        }
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        match (self.rs1(), self.rs2()) {
            (0, _) => vec![
                AsmArgument::Register(AsmRegister::Gp(self.rs2())),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
            (_, 0) => vec![
                AsmArgument::Register(AsmRegister::Gp(self.rs1())),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
            _ => vec![
                AsmArgument::Register(AsmRegister::Gp(self.rs1())),
                AsmArgument::Register(AsmRegister::Gp(self.rs2())),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
        }
    }
}

impl AsmInstruction for Bne {
    #[inline]
    fn verb(&self) -> &'static str {
        match (self.rs1(), self.rs2()) {
            (0, _) | (_, 0) => "bnez",
            _ => "bne",
        }
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        match (self.rs1(), self.rs2()) {
            (0, _) => vec![
                AsmArgument::Register(AsmRegister::Gp(self.rs2())),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
            (_, 0) => vec![
                AsmArgument::Register(AsmRegister::Gp(self.rs1())),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
            _ => vec![
                AsmArgument::Register(AsmRegister::Gp(self.rs1())),
                AsmArgument::Register(AsmRegister::Gp(self.rs2())),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
        }
    }
}

impl AsmInstruction for Blt {
    #[inline]
    fn verb(&self) -> &'static str {
        match (self.rs1(), self.rs2()) {
            (0, _) => "bgtz",
            (_, 0) => "bltz",
            _ => "blt",
        }
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        match (self.rs1(), self.rs2()) {
            (0, _) => vec![
                AsmArgument::Register(AsmRegister::Gp(self.rs2())),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
            (_, 0) => vec![
                AsmArgument::Register(AsmRegister::Gp(self.rs1())),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
            _ => vec![
                AsmArgument::Register(AsmRegister::Gp(self.rs1())),
                AsmArgument::Register(AsmRegister::Gp(self.rs2())),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
        }
    }
}

impl AsmInstruction for Bge {
    #[inline]
    fn verb(&self) -> &'static str {
        match (self.rs1(), self.rs2()) {
            (0, _) => "blez",
            (_, 0) => "bgez",
            _ => "bge",
        }
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        match (self.rs1(), self.rs2()) {
            (0, _) => vec![
                AsmArgument::Register(AsmRegister::Gp(self.rs2())),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
            (_, 0) => vec![
                AsmArgument::Register(AsmRegister::Gp(self.rs1())),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
            _ => vec![
                AsmArgument::Register(AsmRegister::Gp(self.rs1())),
                AsmArgument::Register(AsmRegister::Gp(self.rs2())),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
        }
    }
}

impl AsmInstruction for Bltu {
    #[inline]
    fn verb(&self) -> &'static str {
        "bltu"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::Register(AsmRegister::Gp(self.rs1())),
            AsmArgument::Register(AsmRegister::Gp(self.rs2())),
            AsmArgument::Immediate(i64::from(self.imm())),
        ]
    }
}

impl AsmInstruction for Bgeu {
    #[inline]
    fn verb(&self) -> &'static str {
        "bgeu"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::Register(AsmRegister::Gp(self.rs1())),
            AsmArgument::Register(AsmRegister::Gp(self.rs2())),
            AsmArgument::Immediate(i64::from(self.imm())),
        ]
    }
}

impl AsmInstruction for Lb {
    #[inline]
    fn verb(&self) -> &'static str {
        "lb"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::Register(AsmRegister::Gp(self.rd())),
            AsmArgument::OffsetImmediate(i64::from(self.imm()), AsmRegister::Gp(self.rs1())),
        ]
    }
}

impl AsmInstruction for Lh {
    #[inline]
    fn verb(&self) -> &'static str {
        "lh"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::Register(AsmRegister::Gp(self.rd())),
            AsmArgument::OffsetImmediate(i64::from(self.imm()), AsmRegister::Gp(self.rs1())),
        ]
    }
}

impl AsmInstruction for Lw {
    #[inline]
    fn verb(&self) -> &'static str {
        "lw"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::Register(AsmRegister::Gp(self.rd())),
            AsmArgument::OffsetImmediate(i64::from(self.imm()), AsmRegister::Gp(self.rs1())),
        ]
    }
}

impl AsmInstruction for Lbu {
    #[inline]
    fn verb(&self) -> &'static str {
        "lbu"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::Register(AsmRegister::Gp(self.rd())),
            AsmArgument::OffsetImmediate(i64::from(self.imm()), AsmRegister::Gp(self.rs1())),
        ]
    }
}

impl AsmInstruction for Lhu {
    #[inline]
    fn verb(&self) -> &'static str {
        "lhu"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::Register(AsmRegister::Gp(self.rd())),
            AsmArgument::OffsetImmediate(i64::from(self.imm()), AsmRegister::Gp(self.rs1())),
        ]
    }
}

impl AsmInstruction for Sb {
    #[inline]
    fn verb(&self) -> &'static str {
        "sb"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::Register(AsmRegister::Gp(self.rs2())),
            AsmArgument::OffsetImmediate(i64::from(self.imm()), AsmRegister::Gp(self.rs1())),
        ]
    }
}

impl AsmInstruction for Sh {
    #[inline]
    fn verb(&self) -> &'static str {
        "sh"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::Register(AsmRegister::Gp(self.rs2())),
            AsmArgument::OffsetImmediate(i64::from(self.imm()), AsmRegister::Gp(self.rs1())),
        ]
    }
}

impl AsmInstruction for Sw {
    #[inline]
    fn verb(&self) -> &'static str {
        "sw"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::Register(AsmRegister::Gp(self.rs2())),
            AsmArgument::OffsetImmediate(i64::from(self.imm()), AsmRegister::Gp(self.rs1())),
        ]
    }
}

impl AsmInstruction for Addi {
    #[inline]
    fn verb(&self) -> &'static str {
        match (self.rd(), self.rs1(), self.imm()) {
            (0, 0, 0) => "nop",
            (_, 0, _) => "li",
            (_, _, 0) => "mv",
            _ => "addi",
        }
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        match (self.rd(), self.rs1(), self.imm()) {
            (0, 0, 0) => vec![],
            (_, 0, _) => vec![
                AsmArgument::Register(AsmRegister::Gp(self.rd())),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
            (_, _, 0) => vec![
                AsmArgument::Register(AsmRegister::Gp(self.rd())),
                AsmArgument::Register(AsmRegister::Gp(self.rs1())),
            ],
            _ => vec![
                AsmArgument::Register(AsmRegister::Gp(self.rd())),
                AsmArgument::Register(AsmRegister::Gp(self.rs1())),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
        }
    }
}

impl AsmInstruction for Slti {
    #[inline]
    fn verb(&self) -> &'static str {
        "slti"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::Register(AsmRegister::Gp(self.rd())),
            AsmArgument::Register(AsmRegister::Gp(self.rs1())),
            AsmArgument::Immediate(i64::from(self.imm())),
        ]
    }
}

impl AsmInstruction for Sltiu {
    #[inline]
    fn verb(&self) -> &'static str {
        match self.imm() {
            1 => "seqz",
            _ => "slti",
        }
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        match self.imm() {
            1 => vec![
                AsmArgument::Register(AsmRegister::Gp(self.rd())),
                AsmArgument::Register(AsmRegister::Gp(self.rs1())),
            ],
            _ => vec![
                AsmArgument::Register(AsmRegister::Gp(self.rd())),
                AsmArgument::Register(AsmRegister::Gp(self.rs1())),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
        }
    }
}

impl AsmInstruction for Xori {
    #[inline]
    fn verb(&self) -> &'static str {
        match self.imm() {
            0xFFF => "not",
            _ => "xori",
        }
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        match self.imm() {
            0xFFF => vec![
                AsmArgument::Register(AsmRegister::Gp(self.rd())),
                AsmArgument::Register(AsmRegister::Gp(self.rs1())),
            ],
            _ => vec![
                AsmArgument::Register(AsmRegister::Gp(self.rd())),
                AsmArgument::Register(AsmRegister::Gp(self.rs1())),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
        }
    }
}

impl AsmInstruction for Ori {
    #[inline]
    fn verb(&self) -> &'static str {
        "ori"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::Register(AsmRegister::Gp(self.rd())),
            AsmArgument::Register(AsmRegister::Gp(self.rs1())),
            AsmArgument::Immediate(i64::from(self.imm())),
        ]
    }
}

impl AsmInstruction for Andi {
    #[inline]
    fn verb(&self) -> &'static str {
        "andi"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::Register(AsmRegister::Gp(self.rd())),
            AsmArgument::Register(AsmRegister::Gp(self.rs1())),
            AsmArgument::Immediate(i64::from(self.imm())),
        ]
    }
}

#[test]
fn lui() {
    use crate::{AsmFormatOptions, AsmInstruction};

    let instr = Lui::try_from(0xdeadc537).unwrap();
    println!("{}", instr.display(AsmFormatOptions::default()));
    assert!(false);
}
