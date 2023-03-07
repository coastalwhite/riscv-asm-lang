use riscv_encoding::rv32i::*;

use crate::{AsmArgument, AsmInstruction, AsmRegister};

// lui                  REG,{IMM >> 12}
// auipc                REG,{SYMBOL / ADDRESS}
// jal                  REG,{SYMBOL / ADDRESS}
// jalr                 REG,REG,IMM
// beq                  REG,REG,{SYMBOL / ADDRESS}
// bne                  REG,REG,{SYMBOL / ADDRESS}
// blt                  REG,REG,{SYMBOL / ADDRESS}
// bge                  REG,REG,{SYMBOL / ADDRESS}
// bltu                 REG,REG,{SYMBOL / ADDRESS}
// bgeu                 REG,REG,{SYMBOL / ADDRESS}
// lb                   REG,IMM(REG)
// lh                   REG,IMM(REG)
// lw                   REG,IMM(REG)
// lbu                  REG,IMM(REG)
// lhu                  REG,IMM(REG)
// sb                   REG,IMM(REG)
// sh                   REG,IMM(REG)
// sw                   REG,IMM(REG)
// addi                 REG,REG,IMM
// xori                 REG,REG,IMM
// ori                  REG,REG,IMM
// andi                 REG,REG,IMM
// slli                 REG,REG,IMM
// srli                 REG,REG,IMM
// srai                 REG,REG,IMM
// add                  REG,REG,REG
// sub                  REG,REG,REG
// xor                  REG,REG,REG
// or                   REG,REG,REG
// and                  REG,REG,REG
// slt                  REG,REG,REG
// sltu                 REG,REG,REG
// sll                  REG,REG,REG
// srl                  REG,REG,REG
// sra                  REG,REG,REG
// fence
// fence.tso
// pause
// ebreak
// ecall

impl AsmInstruction for Rv32IInstruction {
    fn verb(&self) -> &'static str {
        match self {
            Self::Lui(i) => i.verb(),
            Self::AuiPc(i) => i.verb(),
            Self::Jal(i) => i.verb(),
            Self::JalR(i) => i.verb(),
            Self::Beq(i) => i.verb(),
            Self::Bne(i) => i.verb(),
            Self::Blt(i) => i.verb(),
            Self::Bge(i) => i.verb(),
            Self::Bltu(i) => i.verb(),
            Self::Bgeu(i) => i.verb(),
            Self::Lb(i) => i.verb(),
            Self::Lh(i) => i.verb(),
            Self::Lw(i) => i.verb(),
            Self::Lbu(i) => i.verb(),
            Self::Lhu(i) => i.verb(),
            Self::Sb(i) => i.verb(),
            Self::Sh(i) => i.verb(),
            Self::Sw(i) => i.verb(),
            Self::Addi(i) => i.verb(),
            Self::Slti(i) => i.verb(),
            Self::Sltiu(i) => i.verb(),
            Self::Xori(i) => i.verb(),
            Self::Ori(i) => i.verb(),
            Self::Andi(i) => i.verb(),
            Self::Slli(i) => i.verb(),
            Self::Srli(i) => i.verb(),
            Self::Srai(i) => i.verb(),
            Self::Add(i) => i.verb(),
            Self::Sub(i) => i.verb(),
            Self::Sll(i) => i.verb(),
            Self::Slt(i) => i.verb(),
            Self::Sltu(i) => i.verb(),
            Self::Xor(i) => i.verb(),
            Self::Srl(i) => i.verb(),
            Self::Sra(i) => i.verb(),
            Self::Or(i) => i.verb(),
            Self::And(i) => i.verb(),
            Self::Fence(i) => i.verb(),
            Self::FenceTso(i) => i.verb(),
            Self::Pause(i) => i.verb(),
            Self::ECall(i) => i.verb(),
            Self::EBreak(i) => i.verb(),
        }
    }

    fn arguments(&self) -> Vec<AsmArgument> {
        match self {
            Self::Lui(i) => i.arguments(),
            Self::AuiPc(i) => i.arguments(),
            Self::Jal(i) => i.arguments(),
            Self::JalR(i) => i.arguments(),
            Self::Beq(i) => i.arguments(),
            Self::Bne(i) => i.arguments(),
            Self::Blt(i) => i.arguments(),
            Self::Bge(i) => i.arguments(),
            Self::Bltu(i) => i.arguments(),
            Self::Bgeu(i) => i.arguments(),
            Self::Lb(i) => i.arguments(),
            Self::Lh(i) => i.arguments(),
            Self::Lw(i) => i.arguments(),
            Self::Lbu(i) => i.arguments(),
            Self::Lhu(i) => i.arguments(),
            Self::Sb(i) => i.arguments(),
            Self::Sh(i) => i.arguments(),
            Self::Sw(i) => i.arguments(),
            Self::Addi(i) => i.arguments(),
            Self::Slti(i) => i.arguments(),
            Self::Sltiu(i) => i.arguments(),
            Self::Xori(i) => i.arguments(),
            Self::Ori(i) => i.arguments(),
            Self::Andi(i) => i.arguments(),
            Self::Slli(i) => i.arguments(),
            Self::Srli(i) => i.arguments(),
            Self::Srai(i) => i.arguments(),
            Self::Add(i) => i.arguments(),
            Self::Sub(i) => i.arguments(),
            Self::Sll(i) => i.arguments(),
            Self::Slt(i) => i.arguments(),
            Self::Sltu(i) => i.arguments(),
            Self::Xor(i) => i.arguments(),
            Self::Srl(i) => i.arguments(),
            Self::Sra(i) => i.arguments(),
            Self::Or(i) => i.arguments(),
            Self::And(i) => i.arguments(),
            Self::Fence(i) => i.arguments(),
            Self::FenceTso(i) => i.arguments(),
            Self::Pause(i) => i.arguments(),
            Self::ECall(i) => i.arguments(),
            Self::EBreak(i) => i.arguments(),
        }
    }
}

impl AsmInstruction for Lui {
    #[inline]
    fn verb(&self) -> &'static str {
        "lui"
    }
    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::reg(self.rd()).unwrap(),
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
            AsmArgument::reg(self.rd()).unwrap(),
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
                AsmArgument::reg(self.rd()).unwrap(),
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
            (0, 0 | 1, _) => vec![AsmArgument::reg(self.rs1()).unwrap()],
            _ => vec![
                AsmArgument::reg(self.rd()).unwrap(),
                AsmArgument::reg(self.rs1()).unwrap(),
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
                AsmArgument::reg(self.rs2()).unwrap(),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
            (_, 0) => vec![
                AsmArgument::reg(self.rs1()).unwrap(),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
            _ => vec![
                AsmArgument::reg(self.rs1()).unwrap(),
                AsmArgument::reg(self.rs2()).unwrap(),
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
                AsmArgument::reg(self.rs2()).unwrap(),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
            (_, 0) => vec![
                AsmArgument::reg(self.rs1()).unwrap(),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
            _ => vec![
                AsmArgument::reg(self.rs1()).unwrap(),
                AsmArgument::reg(self.rs2()).unwrap(),
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
                AsmArgument::reg(self.rs2()).unwrap(),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
            (_, 0) => vec![
                AsmArgument::reg(self.rs1()).unwrap(),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
            _ => vec![
                AsmArgument::reg(self.rs1()).unwrap(),
                AsmArgument::reg(self.rs2()).unwrap(),
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
                AsmArgument::reg(self.rs2()).unwrap(),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
            (_, 0) => vec![
                AsmArgument::reg(self.rs1()).unwrap(),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
            _ => vec![
                AsmArgument::reg(self.rs1()).unwrap(),
                AsmArgument::reg(self.rs2()).unwrap(),
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
            AsmArgument::reg(self.rs1()).unwrap(),
            AsmArgument::reg(self.rs2()).unwrap(),
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
            AsmArgument::reg(self.rs1()).unwrap(),
            AsmArgument::reg(self.rs2()).unwrap(),
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
            AsmArgument::reg(self.rd()).unwrap(),
            AsmArgument::OffsetImmediate(
                i64::from(self.imm()),
                AsmRegister::try_from(self.rs1()).unwrap(),
            ),
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
            AsmArgument::reg(self.rd()).unwrap(),
            AsmArgument::OffsetImmediate(
                i64::from(self.imm()),
                AsmRegister::try_from(self.rs1()).unwrap(),
            ),
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
            AsmArgument::reg(self.rd()).unwrap(),
            AsmArgument::OffsetImmediate(
                i64::from(self.imm()),
                AsmRegister::try_from(self.rs1()).unwrap(),
            ),
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
            AsmArgument::reg(self.rd()).unwrap(),
            AsmArgument::OffsetImmediate(
                i64::from(self.imm()),
                AsmRegister::try_from(self.rs1()).unwrap(),
            ),
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
            AsmArgument::reg(self.rd()).unwrap(),
            AsmArgument::OffsetImmediate(
                i64::from(self.imm()),
                AsmRegister::try_from(self.rs1()).unwrap(),
            ),
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
            AsmArgument::reg(self.rs2()).unwrap(),
            AsmArgument::OffsetImmediate(
                i64::from(self.imm()),
                AsmRegister::try_from(self.rs1()).unwrap(),
            ),
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
            AsmArgument::reg(self.rs2()).unwrap(),
            AsmArgument::OffsetImmediate(
                i64::from(self.imm()),
                AsmRegister::try_from(self.rs1()).unwrap(),
            ),
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
            AsmArgument::reg(self.rs2()).unwrap(),
            AsmArgument::OffsetImmediate(
                i64::from(self.imm()),
                AsmRegister::try_from(self.rs1()).unwrap(),
            ),
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
                AsmArgument::reg(self.rd()).unwrap(),
                AsmArgument::Immediate(i64::from(self.imm())),
            ],
            (_, _, 0) => vec![
                AsmArgument::reg(self.rd()).unwrap(),
                AsmArgument::reg(self.rs1()).unwrap(),
            ],
            _ => vec![
                AsmArgument::reg(self.rd()).unwrap(),
                AsmArgument::reg(self.rs1()).unwrap(),
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
            AsmArgument::reg(self.rd()).unwrap(),
            AsmArgument::reg(self.rs1()).unwrap(),
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
                AsmArgument::reg(self.rd()).unwrap(),
                AsmArgument::reg(self.rs1()).unwrap(),
            ],
            _ => vec![
                AsmArgument::reg(self.rd()).unwrap(),
                AsmArgument::reg(self.rs1()).unwrap(),
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
                AsmArgument::reg(self.rd()).unwrap(),
                AsmArgument::reg(self.rs1()).unwrap(),
            ],
            _ => vec![
                AsmArgument::reg(self.rd()).unwrap(),
                AsmArgument::reg(self.rs1()).unwrap(),
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
            AsmArgument::reg(self.rd()).unwrap(),
            AsmArgument::reg(self.rs1()).unwrap(),
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
            AsmArgument::reg(self.rd()).unwrap(),
            AsmArgument::reg(self.rs1()).unwrap(),
            AsmArgument::Immediate(i64::from(self.imm())),
        ]
    }
}

impl AsmInstruction for Slli {
    #[inline]
    fn verb(&self) -> &'static str {
        "slli"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::reg(self.rd()).unwrap(),
            AsmArgument::reg(self.rs1()).unwrap(),
            AsmArgument::Immediate(i64::from(self.shamt())),
        ]
    }
}

impl AsmInstruction for Srli {
    #[inline]
    fn verb(&self) -> &'static str {
        "srli"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::reg(self.rd()).unwrap(),
            AsmArgument::reg(self.rs1()).unwrap(),
            AsmArgument::Immediate(i64::from(self.shamt())),
        ]
    }
}

impl AsmInstruction for Srai {
    #[inline]
    fn verb(&self) -> &'static str {
        "srai"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::reg(self.rd()).unwrap(),
            AsmArgument::reg(self.rs1()).unwrap(),
            AsmArgument::Immediate(i64::from(self.shamt())),
        ]
    }
}

impl AsmInstruction for Add {
    #[inline]
    fn verb(&self) -> &'static str {
        "add"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::reg(self.rd()).unwrap(),
            AsmArgument::reg(self.rs1()).unwrap(),
            AsmArgument::reg(self.rs2()).unwrap(),
        ]
    }
}

impl AsmInstruction for Sub {
    #[inline]
    fn verb(&self) -> &'static str {
        "sub"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::reg(self.rd()).unwrap(),
            AsmArgument::reg(self.rs1()).unwrap(),
            AsmArgument::reg(self.rs2()).unwrap(),
        ]
    }
}

impl AsmInstruction for Xor {
    #[inline]
    fn verb(&self) -> &'static str {
        "xor"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::reg(self.rd()).unwrap(),
            AsmArgument::reg(self.rs1()).unwrap(),
            AsmArgument::reg(self.rs2()).unwrap(),
        ]
    }
}

impl AsmInstruction for Or {
    #[inline]
    fn verb(&self) -> &'static str {
        "or"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::reg(self.rd()).unwrap(),
            AsmArgument::reg(self.rs1()).unwrap(),
            AsmArgument::reg(self.rs2()).unwrap(),
        ]
    }
}

impl AsmInstruction for And {
    #[inline]
    fn verb(&self) -> &'static str {
        "and"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::reg(self.rd()).unwrap(),
            AsmArgument::reg(self.rs1()).unwrap(),
            AsmArgument::reg(self.rs2()).unwrap(),
        ]
    }
}

impl AsmInstruction for Slt {
    #[inline]
    fn verb(&self) -> &'static str {
        "slt"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::reg(self.rd()).unwrap(),
            AsmArgument::reg(self.rs1()).unwrap(),
            AsmArgument::reg(self.rs2()).unwrap(),
        ]
    }
}

impl AsmInstruction for Sltu {
    #[inline]
    fn verb(&self) -> &'static str {
        "sltu"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::reg(self.rd()).unwrap(),
            AsmArgument::reg(self.rs1()).unwrap(),
            AsmArgument::reg(self.rs2()).unwrap(),
        ]
    }
}

impl AsmInstruction for Sll {
    #[inline]
    fn verb(&self) -> &'static str {
        "sll"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::reg(self.rd()).unwrap(),
            AsmArgument::reg(self.rs1()).unwrap(),
            AsmArgument::reg(self.rs2()).unwrap(),
        ]
    }
}

impl AsmInstruction for Srl {
    #[inline]
    fn verb(&self) -> &'static str {
        "srl"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::reg(self.rd()).unwrap(),
            AsmArgument::reg(self.rs1()).unwrap(),
            AsmArgument::reg(self.rs2()).unwrap(),
        ]
    }
}

impl AsmInstruction for Sra {
    #[inline]
    fn verb(&self) -> &'static str {
        "add"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        vec![
            AsmArgument::reg(self.rd()).unwrap(),
            AsmArgument::reg(self.rs1()).unwrap(),
            AsmArgument::reg(self.rs2()).unwrap(),
        ]
    }
}

impl AsmInstruction for Fence {
    #[inline]
    fn verb(&self) -> &'static str {
        "fence"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        // TODO
        vec![]
    }
}

impl AsmInstruction for FenceTso {
    #[inline]
    fn verb(&self) -> &'static str {
        "fence.tso"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        // TODO
        vec![]
    }
}

impl AsmInstruction for Pause {
    #[inline]
    fn verb(&self) -> &'static str {
        "pause"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        // TODO
        vec![]
    }
}

impl AsmInstruction for EBreak {
    #[inline]
    fn verb(&self) -> &'static str {
        "ebreak"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        // TODO
        vec![]
    }
}

impl AsmInstruction for ECall {
    #[inline]
    fn verb(&self) -> &'static str {
        "ecall"
    }

    #[inline]
    fn arguments(&self) -> Vec<AsmArgument> {
        // TODO
        vec![]
    }
}

#[test]
fn lui() {
    use crate::{AsmFormatOptions, AsmInstruction};

    let instr = Lui::try_from(0xdeadc537).unwrap();
    println!("{}", instr.display(AsmFormatOptions::default()));
    assert!(false);
}
