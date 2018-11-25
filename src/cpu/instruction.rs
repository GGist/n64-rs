#[derive(Debug, Copy, Clone)]
enum OpCode {
    // Pseudo
    NOP,
    LI,
    BNEZ,
    MOVE,
    B,
    BEQZ,
    BNEZL,
    BAL,

    NEG,
    NEGU,
    BEQZL,
    SS,
    LS,

    // CPU
    ADDIU,
    BNE,
    LUI,
    LW,
    SW,
    ORI,
    BEQ,
    JAL,
    ADDI,
    ANDI,
    SLTIU,
    CACHE,
    BNEL,
    SLTI,
    BEQL,
    BLEZL,
    SB,
    LBU,
    XORI,
    J,
    LB,
    LWC1,


// Special
    SLL,
    OR,
    AND,
    ADD,
    SLT,
    SLTU,
    JR,
    MFLO,
    ADDU,
    SUBU,
    SRLV,
    SLLV,
    XOR,
    SRL,
    MULTU,
    MOVCI,
    DSRA32,
    MFHI,

    // COP0
    MTC0,
    MFC0,

    // REGIMM
    BGEZL,
    BGEZAL,
    BLTZ

}

impl OpCode {
    pub fn from_word(word: u32) -> Result<OpCode, String> {
        let code = (word >> 26) as u8;

        match code {
            0b001001 => Ok(OpCode::ADDIU),
            0b000101 => Ok(OpCode::BNE),
            0b001111 => Ok(OpCode::LUI),
            0b100011 => Ok(OpCode::LW),
            0b101011 => Ok(OpCode::SW),
            0b001101 => Ok(OpCode::ORI),
            0b000100 => Ok(OpCode::BEQ),
            0b000011 => Ok(OpCode::JAL),
            0b001000 => Ok(OpCode::ADDI),
            0b001100 => Ok(OpCode::ANDI),
            0b001011 => Ok(OpCode::SLTIU),
            0b101111 => Ok(OpCode::CACHE),
            0b010101 => Ok(OpCode::BNEL),
            0b001010 => Ok(OpCode::SLTI),
            0b010100 => Ok(OpCode::BEQL),
            0b010110 => Ok(OpCode::BLEZL),
            0b101000 => Ok(OpCode::SB),
            0b100100 => Ok(OpCode::LBU),
            0b001110 => Ok(OpCode::XORI),
            0b000010 => Ok(OpCode::J),
            0b100000 => Ok(OpCode::LB),
            0b110001 => Ok(OpCode::LWC1),
            0b000001 => OpCode::from_regimm_rt(word),
            0b000000 => OpCode::from_special_inst(word),
            // Last two bits are the co processor #
            0b010000 => OpCode::from_cop0_fmt(word),
            _        => Err(format!("Unknown CPU Code {:#b} From Word {}", code, word))
        }
    }

    fn from_cop0_fmt(word: u32) -> Result<OpCode, String> {
        let code = ((word << 6) >> 27) as u8;

        match code {
            0b00100 => Ok(OpCode::MTC0),
            0b00000 => Ok(OpCode::MFC0),
            // TODO: Figure out other MMU codes
            _       => Ok(OpCode::MFC0)
            //_       => Err(format!("Unknown COP0 FMT Code {:#b} From Word {}", code, word))
        }
    }

    fn from_regimm_rt(word: u32) -> Result<OpCode, String> {
        let code = ((word << 11) >> 27) as u8;

        match code {
            0b00011 => Ok(OpCode::BGEZL),
            0b10001 => Ok(OpCode::BGEZAL),
            0b00000 => Ok(OpCode::BLTZ),
            _       => Err(format!("Unknown REGIMM RT Code {:#b} From Word {}", code, word))
        }
    }

    fn from_special_inst(word: u32) -> Result<OpCode, String> {
        let code = ((word << 26) >> 26) as u8;

        match code {
            0b000000 => Ok(OpCode::SLL),
            0b100101 => Ok(OpCode::OR),
            0b100100 => Ok(OpCode::AND),
            0b100000 => Ok(OpCode::ADD),
            0b101010 => Ok(OpCode::SLT),
            0b101011 => Ok(OpCode::SLTU),
            0b001000 => Ok(OpCode::JR),
            0b010010 => Ok(OpCode::MFLO),
            0b100001 => Ok(OpCode::ADDU),
            0b100011 => Ok(OpCode::SUBU),
            0b000110 => Ok(OpCode::SRLV),
            0b000100 => Ok(OpCode::SLLV),
            0b100110 => Ok(OpCode::XOR),
            0b000010 => Ok(OpCode::SRL),
            0b011001 => Ok(OpCode::MULTU),
            0b000001 => Ok(OpCode::MOVCI),
            0b111111 => Ok(OpCode::DSRA32),
            0b010000 => Ok(OpCode::MFHI),
            _        => Err(format!("Unknown Special Code {:#b} From Word {}", code, word))
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    code: OpCode,
    line: u32
}

impl Instruction {
    pub fn from_word(word: u32) -> Result<Instruction, String> {
        Ok(Instruction {
            code: OpCode::from_word(word)?,
            line: word
        })
    }
}