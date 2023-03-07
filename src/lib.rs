use std::fmt::{Display, Error, Write};

pub use riscv_encoding;

#[derive(Debug, Clone)]
pub struct AsmFormatOptions {
    raw_reg_names: bool,
    verb_arg_spacing: &'static str,
    arg_spacing: &'static str,
    immediate_format: AsmImmediateFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsmImmediateFormat {
    UnsignedDecimal,
    SignedDecimal,
    Hex,
    Bin,
}

impl Default for AsmFormatOptions {
    fn default() -> Self {
        Self {
            raw_reg_names: false,
            verb_arg_spacing: "\t",
            arg_spacing: "",
            immediate_format: AsmImmediateFormat::Hex,
        }
    }
}

pub enum AsmArgument<'a> {
    Label(&'a str),
    Immediate(i64),
    Register(AsmRegister),
    OffsetImmediate(i64, AsmRegister),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AsmRegister {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    R16,
    R17,
    R18,
    R19,
    R20,
    R21,
    R22,
    R23,
    R24,
    R25,
    R26,
    R27,
    R28,
    R29,
    R30,
    R31,
}

impl AsmDisplay for AsmRegister {
    fn fmt<W: Write>(&self, f: &mut W, options: &AsmFormatOptions) -> Result<(), Error> {
        if options.raw_reg_names {
            match self {
                Self::R0 => write!(f, "x0"),
                Self::R1 => write!(f, "x1"),
                Self::R2 => write!(f, "x2"),
                Self::R3 => write!(f, "x3"),
                Self::R4 => write!(f, "x4"),
                Self::R5 => write!(f, "x5"),
                Self::R6 => write!(f, "x6"),
                Self::R7 => write!(f, "x7"),
                Self::R8 => write!(f, "x8"),
                Self::R9 => write!(f, "x9"),
                Self::R10 => write!(f, "x10"),
                Self::R11 => write!(f, "x11"),
                Self::R12 => write!(f, "x12"),
                Self::R13 => write!(f, "x13"),
                Self::R14 => write!(f, "x14"),
                Self::R15 => write!(f, "x15"),
                Self::R16 => write!(f, "x16"),
                Self::R17 => write!(f, "x17"),
                Self::R18 => write!(f, "x18"),
                Self::R19 => write!(f, "x19"),
                Self::R20 => write!(f, "x20"),
                Self::R21 => write!(f, "x21"),
                Self::R22 => write!(f, "x22"),
                Self::R23 => write!(f, "x23"),
                Self::R24 => write!(f, "x24"),
                Self::R25 => write!(f, "x25"),
                Self::R26 => write!(f, "x26"),
                Self::R27 => write!(f, "x27"),
                Self::R28 => write!(f, "x28"),
                Self::R29 => write!(f, "x29"),
                Self::R30 => write!(f, "x30"),
                Self::R31 => write!(f, "x31"),
            }
        } else {
            match self {
                Self::R0 => write!(f, "zero"),
                Self::R1 => write!(f, "ra"),
                Self::R2 => write!(f, "sp"),
                Self::R3 => write!(f, "gp"),
                Self::R4 => write!(f, "tp"),
                Self::R5 => write!(f, "t0"),
                Self::R6 => write!(f, "t1"),
                Self::R7 => write!(f, "t2"),
                // or `fp`
                Self::R8 => write!(f, "s0"),
                Self::R9 => write!(f, "s1"),
                Self::R10 => write!(f, "a0"),
                Self::R11 => write!(f, "a1"),
                Self::R12 => write!(f, "a2"),
                Self::R13 => write!(f, "a3"),
                Self::R14 => write!(f, "a4"),
                Self::R15 => write!(f, "a5"),
                Self::R16 => write!(f, "a6"),
                Self::R17 => write!(f, "a7"),
                Self::R18 => write!(f, "s2"),
                Self::R19 => write!(f, "s3"),
                Self::R20 => write!(f, "s4"),
                Self::R21 => write!(f, "s5"),
                Self::R22 => write!(f, "s6"),
                Self::R23 => write!(f, "s7"),
                Self::R24 => write!(f, "s8"),
                Self::R25 => write!(f, "s9"),
                Self::R26 => write!(f, "s10"),
                Self::R27 => write!(f, "s11"),
                Self::R28 => write!(f, "t3"),
                Self::R29 => write!(f, "t4"),
                Self::R30 => write!(f, "t5"),
                Self::R31 => write!(f, "t6"),
            }
        }
    }
}

impl TryFrom<u8> for AsmRegister {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::R0),
            1 => Ok(Self::R1),
            2 => Ok(Self::R2),
            3 => Ok(Self::R3),
            4 => Ok(Self::R4),
            5 => Ok(Self::R5),
            6 => Ok(Self::R6),
            7 => Ok(Self::R7),
            8 => Ok(Self::R8),
            9 => Ok(Self::R9),
            10 => Ok(Self::R10),
            11 => Ok(Self::R11),
            12 => Ok(Self::R12),
            13 => Ok(Self::R13),
            14 => Ok(Self::R14),
            15 => Ok(Self::R15),
            16 => Ok(Self::R16),
            17 => Ok(Self::R17),
            18 => Ok(Self::R18),
            19 => Ok(Self::R19),
            20 => Ok(Self::R20),
            21 => Ok(Self::R21),
            22 => Ok(Self::R22),
            23 => Ok(Self::R23),
            24 => Ok(Self::R24),
            25 => Ok(Self::R25),
            26 => Ok(Self::R26),
            27 => Ok(Self::R27),
            28 => Ok(Self::R28),
            29 => Ok(Self::R29),
            30 => Ok(Self::R30),
            31 => Ok(Self::R31),
            _ => Err(()),
        }
    }
}

impl From<AsmRegister> for u8 {
    fn from(value: AsmRegister) -> Self {
        match value {
            AsmRegister::R0 => 0,
            AsmRegister::R1 => 1,
            AsmRegister::R2 => 2,
            AsmRegister::R3 => 3,
            AsmRegister::R4 => 4,
            AsmRegister::R5 => 5,
            AsmRegister::R6 => 6,
            AsmRegister::R7 => 7,
            AsmRegister::R8 => 8,
            AsmRegister::R9 => 9,
            AsmRegister::R10 => 10,
            AsmRegister::R11 => 11,
            AsmRegister::R12 => 12,
            AsmRegister::R13 => 13,
            AsmRegister::R14 => 14,
            AsmRegister::R15 => 15,
            AsmRegister::R16 => 16,
            AsmRegister::R17 => 17,
            AsmRegister::R18 => 18,
            AsmRegister::R19 => 19,
            AsmRegister::R20 => 20,
            AsmRegister::R21 => 21,
            AsmRegister::R22 => 22,
            AsmRegister::R23 => 23,
            AsmRegister::R24 => 24,
            AsmRegister::R25 => 25,
            AsmRegister::R26 => 26,
            AsmRegister::R27 => 27,
            AsmRegister::R28 => 28,
            AsmRegister::R29 => 29,
            AsmRegister::R30 => 30,
            AsmRegister::R31 => 31,
        }
    }
}

impl<'a> AsmArgument<'a> {
    #[inline]
    pub fn reg(num: u8) -> Option<Self> {
        Some(AsmArgument::Register(AsmRegister::try_from(num).ok()?))
    }
}

impl<'a> AsmDisplay for AsmArgument<'a> {
    fn fmt<W: Write>(&self, f: &mut W, options: &AsmFormatOptions) -> Result<(), Error> {
        match self {
            Self::Label(label) => f.write_str(label),
            Self::Immediate(imm) => match options.immediate_format {
                AsmImmediateFormat::Hex => write!(f, "0x{imm:x}"),
                AsmImmediateFormat::Bin => write!(f, "0b{imm:b}"),
                AsmImmediateFormat::UnsignedDecimal => write!(f, "{}", *imm as u64),
                AsmImmediateFormat::SignedDecimal => write!(f, "{imm}"),
            },
            Self::Register(reg) => reg.fmt(f, options),
            Self::OffsetImmediate(imm, reg) => {
                write!(f, "{imm}(")?;
                AsmDisplay::fmt(reg, f, options)?;
                write!(f, ")")
            }
        }
    }
}

trait AsmDisplay {
    fn fmt<W: Write>(&self, f: &mut W, options: &AsmFormatOptions) -> Result<(), Error>;
}

pub trait AsmInstruction: Sized {
    fn verb(&self) -> &'static str;
    fn arguments(&self) -> Vec<AsmArgument>;
    fn display(&self, options: AsmFormatOptions) -> AsmDisplayInstruction<Self> {
        AsmDisplayInstruction {
            options,
            instruction: self,
        }
    }
}

pub struct AsmDisplayInstruction<'a, I> {
    options: AsmFormatOptions,
    instruction: &'a I,
}

impl<'a, I: AsmInstruction> AsmDisplay for AsmDisplayInstruction<'a, I> {
    fn fmt<W: Write>(&self, f: &mut W, options: &AsmFormatOptions) -> Result<(), Error> {
        f.write_str(self.instruction.verb())?;
        f.write_str(options.verb_arg_spacing)?;

        for (i, arg) in self.instruction.arguments().iter().enumerate() {
            AsmDisplay::fmt(arg, f, options)?;

            if i != self.instruction.arguments().len() - 1 {
                f.write_str(",")?;
                f.write_str(options.arg_spacing)?;
            }
        }

        Ok(())
    }
}

impl<'a, I: AsmInstruction> Display for AsmDisplayInstruction<'a, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        AsmDisplay::fmt(self, f, &self.options)
    }
}

mod rv32i;
