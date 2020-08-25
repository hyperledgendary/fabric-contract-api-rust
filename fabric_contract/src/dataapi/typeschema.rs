use std::fmt;
#[derive(Debug, Copy, Clone)]
pub enum ContractType {
    Integer,
    Long,
    Float,
    Double,
    CTString,
    Byte,
    Boolean,
    Date,
    DateTime,
}

impl fmt::Display for ContractType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

impl std::str::FromStr for ContractType {
    type Err = ();

    fn from_str(s: &str) -> Result<ContractType, ()> {
        match s {
            "Integer" | "i32" | "u32" => Ok(ContractType::Integer),
            "Long" | "i64" | "u64" => Ok(ContractType::Long),
            "Float" | "f32" => Ok(ContractType::Float),
            "Double" | "d32" => Ok(ContractType::Double),
            "String" => Ok(ContractType::CTString),
            "Byte" | "i8" | "u8" => Ok(ContractType::Byte),
            "Boolean" | "bool" => Ok(ContractType::Boolean),
            "Date" => Ok(ContractType::Date),
            "DateTime" => Ok(ContractType::DateTime),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Format {
    Other,
}

#[derive(Debug, Copy, Clone)]
pub struct TypeSchema {
    pub contract_type: ContractType,
    pub format: Option<Format>,
}

impl Default for TypeSchema {
    fn default() -> Self {
        Self {
            contract_type: ContractType::CTString,
            format: Option::None,
        }
    }
}

impl std::fmt::Display for TypeSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.contract_type)
    }
}

impl TypeSchema {}
