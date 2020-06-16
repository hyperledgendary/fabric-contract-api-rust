

pub enum ContractType {
    Integer, Long, Float, Double, CTString, Byte, Boolean, Date, DateTime
}

pub enum Format {
  Other(String)
}

pub struct TypeSchema {
    pub contract_type: ContractType,
    pub format: Option<Format>,
}

impl Default for TypeSchema {
    fn default() -> Self {
      Self { contract_type: ContractType::CTString, format: Option::None}
    }
    
}

impl TypeSchema {
    
}

