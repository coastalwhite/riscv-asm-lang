use std::fmt::{Display, Error, Write};

#[derive(Debug, Clone)]
pub struct AsmFormatOptions {
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

pub enum AsmRegister {
    Gp(u8),
    Pc,
}

impl AsmDisplay for AsmRegister {
    fn fmt<W: Write>(&self, f: &mut W, options: &AsmFormatOptions) -> Result<(), Error> {
        match self {
            Self::Gp(r) => write!(f, "x{r}"),
            Self::Pc => write!(f, "pc"),
        }
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
