#[derive(Debug, Copy, Clone)]
enum OpCode {
    // Error
    UNKNOWN,

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
    LH,
    SC,
    SCD,
    LWL,
    DADDI,
    LL,
    BLEZ,
    SD,

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
    SRA,
    SYSCALL,
    SYNC,
    DSLL,
    DSRL32,

    // COP0
    MTC0,
    MFC0,

    // COP1
    MFC1,

    // COP2
    MFC2,

    // COP2 VEC
    VNE,

    // REGIMM
    BGEZL,
    BGEZAL,
    BLTZ,
    BLTZAL,
    TGEI

}

impl OpCode {
    pub fn from_word(word: u32) -> Result<OpCode, String> {
        /*let mut word1 = 0;
        word1 |= (word >> 24 & 0xFF) << 0;
        word1 |= (word >> 16 & 0xFF) << 8;
        word1 |= (word >> 8 & 0xFF) << 16;
        word1 |= (word >> 0 & 0xFF) << 24;
        let word = word1;*/

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
            0b100001 => Ok(OpCode::LH),
            0b111000 => Ok(OpCode::SC),
            0b111100 => Ok(OpCode::SCD),
            0b100010 => Ok(OpCode::LWL),
            0b011000 => Ok(OpCode::DADDI),
            0b110000 => Ok(OpCode::LL),
            0b000110 => Ok(OpCode::BLEZ),
            0b111111 => Ok(OpCode::SD),
            0b000001 => OpCode::from_regimm_rt(word),
            0b000000 => OpCode::from_special_inst(word),
            // Last two bits are the co processor #
            // COP 0 = MMU, COP 1 = FPU, COP 2 = RCP
            0b010000 => OpCode::from_cop0_fmt(word),
            0b010001 => OpCode::from_cop1_fmt(word),
            0b010010 => OpCode::from_cop2_fmt(word),
            0b011111 |
            0b110010 |
            0b011100 => Ok(OpCode::UNKNOWN),
            _        => Err(format!("Unknown CPU Code {:#08b} From Word {}", code, word))
        }
    }

    fn from_cop0_fmt(word: u32) -> Result<OpCode, String> {
        let code = ((word << 6) >> 27) as u8;

        match code {
            0b00000 => Ok(OpCode::MFC0),
            0b00100 => Ok(OpCode::MTC0),
            // TODO: Figure out other MMU codes
            _       => Ok(OpCode::MFC0)
            //_       => Err(format!("Unknown COP0 FMT Code {:#07b} From Word {}", code, word))
        }
    }

    fn from_cop1_fmt(word: u32) -> Result<OpCode, String> {
        let code = ((word << 6) >> 27) as u8;

        match code {
            0b00000 => Ok(OpCode::MFC1),
            _       => Err(format!("Unknown COP1 FMT Code {:#07b} From Word {}", code, word))
        }
    }

    fn from_cop2_fmt(word: u32) -> Result<OpCode, String> {
        let code = ((word << 6) >> 27) as u8;

        match code {
            0b00000 => Ok(OpCode::MFC2),
            0b10001 => OpCode::from_cop2_vec(word),
            _       => Err(format!("Unknown COP2 FMT Code {:#07b} From Word {}", code, word))
        }
    }

    fn from_cop2_vec(word: u32) -> Result<OpCode, String> {
        let code = ((word << 26) >> 26) as u8;

        match code {
            0b100010 => Ok(OpCode::VNE),
            _        => Err(format!("Unknown COP2 VEC Code {:#08b} From Word {}", code, word))
        }
    }

    fn from_regimm_rt(word: u32) -> Result<OpCode, String> {
        let code = ((word << 11) >> 27) as u8;

        match code {
            0b00011 => Ok(OpCode::BGEZL),
            0b10001 => Ok(OpCode::BGEZAL),
            0b00000 => Ok(OpCode::BLTZ),
            0b10000 => Ok(OpCode::BLTZAL),
            0b01000 => Ok(OpCode::TGEI),
            _       => Err(format!("Unknown REGIMM RT Code {:#07b} From Word {}", code, word))
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
            0b000011 => Ok(OpCode::SRA),
            0b001100 => Ok(OpCode::SYSCALL),
            0b001111 => Ok(OpCode::SYNC),
            0b111000 => Ok(OpCode::DSLL),
            0b111110 => Ok(OpCode::DSRL32),
            _        => Err(format!("Unknown Special Code {:#08b} From Word {}", code, word))
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    code: OpCode,
    word: u32
}

impl Instruction {
    pub fn from_word(word: u32) -> Result<Instruction, String> {
        Ok(Instruction {
            code: OpCode::from_word(word)?,
            word
        })
    }
}