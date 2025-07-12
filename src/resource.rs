use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Resource {
    pub metadata: Metadata,
    pub regions: HashMap<String, Region>,
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub major_version: u32,
    pub minor_version: u32,
    pub revision: u32,
    pub build_number: u32,
}

#[derive(Debug, Clone)]
pub struct Region {
    pub name: String,
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub name: Option<String>,
    pub parent: Option<String>,
    pub attributes: HashMap<String, NodeAttribute>,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct NodeAttribute {
    pub attribute_type: AttributeType,
    pub value: AttributeValue,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AttributeType {
    None = 0,
    Byte = 1,
    Short = 2,
    UShort = 3,
    Int = 4,
    UInt = 5,
    Float = 6,
    Double = 7,
    IVec2 = 8,
    IVec3 = 9,
    IVec4 = 10,
    Vec2 = 11,
    Vec3 = 12,
    Vec4 = 13,
    Mat2 = 14,
    Mat3 = 15,
    Mat3x4 = 16,
    Mat4x3 = 17,
    Mat4 = 18,
    Bool = 19,
    String = 20,
    Path = 21,
    FixedString = 22,
    LSString = 23,
    ULongLong = 24,
    ScratchBuffer = 25,
    LongLong = 26,
    Int8 = 27,
    TranslatedString = 28,
    WString = 29,
    LSWString = 30,
    UUID = 31,
    Int64 = 32,
    TranslatedFSString = 33,
}

impl AttributeType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::None),
            1 => Some(Self::Byte),
            2 => Some(Self::Short),
            3 => Some(Self::UShort),
            4 => Some(Self::Int),
            5 => Some(Self::UInt),
            6 => Some(Self::Float),
            7 => Some(Self::Double),
            8 => Some(Self::IVec2),
            9 => Some(Self::IVec3),
            10 => Some(Self::IVec4),
            11 => Some(Self::Vec2),
            12 => Some(Self::Vec3),
            13 => Some(Self::Vec4),
            14 => Some(Self::Mat2),
            15 => Some(Self::Mat3),
            16 => Some(Self::Mat3x4),
            17 => Some(Self::Mat4x3),
            18 => Some(Self::Mat4),
            19 => Some(Self::Bool),
            20 => Some(Self::String),
            21 => Some(Self::Path),
            22 => Some(Self::FixedString),
            23 => Some(Self::LSString),
            24 => Some(Self::ULongLong),
            25 => Some(Self::ScratchBuffer),
            26 => Some(Self::LongLong),
            27 => Some(Self::Int8),
            28 => Some(Self::TranslatedString),
            29 => Some(Self::WString),
            30 => Some(Self::LSWString),
            31 => Some(Self::UUID),
            32 => Some(Self::Int64),
            33 => Some(Self::TranslatedFSString),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Byte => "uint8",
            Self::Short => "int16",
            Self::UShort => "uint16",
            Self::Int => "int32",
            Self::UInt => "uint32",
            Self::Float => "float",
            Self::Double => "double",
            Self::IVec2 => "ivec2",
            Self::IVec3 => "ivec3",
            Self::IVec4 => "ivec4",
            Self::Vec2 => "fvec2",
            Self::Vec3 => "fvec3",
            Self::Vec4 => "fvec4",
            Self::Mat2 => "mat2",
            Self::Mat3 => "mat3",
            Self::Mat3x4 => "mat3x4",
            Self::Mat4x3 => "mat4x3",
            Self::Mat4 => "mat4",
            Self::Bool => "bool",
            Self::String => "LSString",
            Self::Path => "path",
            Self::FixedString => "FixedString",
            Self::LSString => "LSString",
            Self::ULongLong => "uint64",
            Self::ScratchBuffer => "ScratchBuffer",
            Self::LongLong => "int64",
            Self::Int8 => "int8",
            Self::TranslatedString => "TranslatedString",
            Self::WString => "WString",
            Self::LSWString => "LSWString",
            Self::UUID => "guid",
            Self::Int64 => "int64",
            Self::TranslatedFSString => "TranslatedFSString",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "None" => Some(Self::None),
            "uint8" => Some(Self::Byte),
            "int16" => Some(Self::Short),
            "uint16" => Some(Self::UShort),
            "int32" => Some(Self::Int),
            "uint32" => Some(Self::UInt),
            "float" => Some(Self::Float),
            "double" => Some(Self::Double),
            "ivec2" => Some(Self::IVec2),
            "ivec3" => Some(Self::IVec3),
            "ivec4" => Some(Self::IVec4),
            "fvec2" => Some(Self::Vec2),
            "fvec3" => Some(Self::Vec3),
            "fvec4" => Some(Self::Vec4),
            "mat2" => Some(Self::Mat2),
            "mat3" => Some(Self::Mat3),
            "mat3x4" => Some(Self::Mat3x4),
            "mat4x3" => Some(Self::Mat4x3),
            "mat4" => Some(Self::Mat4),
            "bool" => Some(Self::Bool),
            "LSString" => Some(Self::String),
            "path" => Some(Self::Path),
            "FixedString" => Some(Self::FixedString),
            "uint64" => Some(Self::ULongLong),
            "ScratchBuffer" => Some(Self::ScratchBuffer),
            "int64" => Some(Self::LongLong),
            "int8" => Some(Self::Int8),
            "TranslatedString" => Some(Self::TranslatedString),
            "WString" => Some(Self::WString),
            "LSWString" => Some(Self::LSWString),
            "guid" => Some(Self::UUID),
            "TranslatedFSString" => Some(Self::TranslatedFSString),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AttributeValue {
    None,
    Byte(u8),
    Short(i16),
    UShort(u16),
    Int(i32),
    UInt(u32),
    Float(f32),
    Double(f64),
    IVec2([i32; 2]),
    IVec3([i32; 3]),
    IVec4([i32; 4]),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Mat2([f32; 4]),
    Mat3([f32; 9]),
    Mat3x4([f32; 12]),
    Mat4x3([f32; 12]),
    Mat4([f32; 16]),
    Bool(bool),
    String(String),
    Path(String),
    FixedString(String),
    LSString(String),
    ULongLong(u64),
    ScratchBuffer(Vec<u8>),
    LongLong(i64),
    Int8(i8),
    TranslatedString { value: String, handle: String },
    WString(String),
    LSWString(String),
    UUID(Uuid),
    Int64(i64),
    TranslatedFSString { value: String, handle: String },
}

impl AttributeValue {
    pub fn to_string(&self) -> String {
        match self {
            Self::None => String::new(),
            Self::Byte(v) => v.to_string(),
            Self::Short(v) => v.to_string(),
            Self::UShort(v) => v.to_string(),
            Self::Int(v) => v.to_string(),
            Self::UInt(v) => v.to_string(),
            Self::Float(v) => v.to_string(),
            Self::Double(v) => v.to_string(),
            Self::IVec2(v) => format!("{} {}", v[0], v[1]),
            Self::IVec3(v) => format!("{} {} {}", v[0], v[1], v[2]),
            Self::IVec4(v) => format!("{} {} {} {}", v[0], v[1], v[2], v[3]),
            Self::Vec2(v) => format!("{} {}", v[0], v[1]),
            Self::Vec3(v) => format!("{} {} {}", v[0], v[1], v[2]),
            Self::Vec4(v) => format!("{} {} {} {}", v[0], v[1], v[2], v[3]),
            Self::Mat2(v) => v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "),
            Self::Mat3(v) => v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "),
            Self::Mat3x4(v) => v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "),
            Self::Mat4x3(v) => v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "),
            Self::Mat4(v) => v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "),
            Self::Bool(v) => if *v { "True".to_string() } else { "False".to_string() },
            Self::String(v) | Self::Path(v) | Self::FixedString(v) | Self::LSString(v) | Self::WString(v) | Self::LSWString(v) => v.clone(),
            Self::ULongLong(v) => v.to_string(),
            Self::ScratchBuffer(v) => base64::encode(v),
            Self::LongLong(v) => v.to_string(),
            Self::Int8(v) => v.to_string(),
            Self::TranslatedString { value, handle } => format!("{};{}", value, handle),
            Self::UUID(v) => v.to_string(),
            Self::Int64(v) => v.to_string(),
            Self::TranslatedFSString { value, handle } => format!("{};{}", value, handle),
        }
    }

    pub fn from_string(attr_type: &AttributeType, s: &str) -> anyhow::Result<Self> {
        Ok(match attr_type {
            AttributeType::None => Self::None,
            AttributeType::Byte => Self::Byte(s.parse()?),
            AttributeType::Short => Self::Short(s.parse()?),
            AttributeType::UShort => Self::UShort(s.parse()?),
            AttributeType::Int => Self::Int(s.parse()?),
            AttributeType::UInt => Self::UInt(s.parse()?),
            AttributeType::Float => Self::Float(s.parse()?),
            AttributeType::Double => Self::Double(s.parse()?),
            AttributeType::IVec2 => {
                let parts: Vec<i32> = s.split_whitespace().map(|x| x.parse()).collect::<Result<Vec<_>, _>>()?;
                if parts.len() != 2 { anyhow::bail!("IVec2 requires 2 values"); }
                Self::IVec2([parts[0], parts[1]])
            },
            AttributeType::IVec3 => {
                let parts: Vec<i32> = s.split_whitespace().map(|x| x.parse()).collect::<Result<Vec<_>, _>>()?;
                if parts.len() != 3 { anyhow::bail!("IVec3 requires 3 values"); }
                Self::IVec3([parts[0], parts[1], parts[2]])
            },
            AttributeType::IVec4 => {
                let parts: Vec<i32> = s.split_whitespace().map(|x| x.parse()).collect::<Result<Vec<_>, _>>()?;
                if parts.len() != 4 { anyhow::bail!("IVec4 requires 4 values"); }
                Self::IVec4([parts[0], parts[1], parts[2], parts[3]])
            },
            AttributeType::Vec2 => {
                let parts: Vec<f32> = s.split_whitespace().map(|x| x.parse()).collect::<Result<Vec<_>, _>>()?;
                if parts.len() != 2 { anyhow::bail!("Vec2 requires 2 values"); }
                Self::Vec2([parts[0], parts[1]])
            },
            AttributeType::Vec3 => {
                let parts: Vec<f32> = s.split_whitespace().map(|x| x.parse()).collect::<Result<Vec<_>, _>>()?;
                if parts.len() != 3 { anyhow::bail!("Vec3 requires 3 values"); }
                Self::Vec3([parts[0], parts[1], parts[2]])
            },
            AttributeType::Vec4 => {
                let parts: Vec<f32> = s.split_whitespace().map(|x| x.parse()).collect::<Result<Vec<_>, _>>()?;
                if parts.len() != 4 { anyhow::bail!("Vec4 requires 4 values"); }
                Self::Vec4([parts[0], parts[1], parts[2], parts[3]])
            },
            AttributeType::Mat2 => {
                let parts: Vec<f32> = s.split_whitespace().map(|x| x.parse()).collect::<Result<Vec<_>, _>>()?;
                if parts.len() != 4 { anyhow::bail!("Mat2 requires 4 values"); }
                Self::Mat2([parts[0], parts[1], parts[2], parts[3]])
            },
            AttributeType::Mat3 => {
                let parts: Vec<f32> = s.split_whitespace().map(|x| x.parse()).collect::<Result<Vec<_>, _>>()?;
                if parts.len() != 9 { anyhow::bail!("Mat3 requires 9 values"); }
                let mut arr = [0.0; 9];
                arr.copy_from_slice(&parts);
                Self::Mat3(arr)
            },
            AttributeType::Mat3x4 | AttributeType::Mat4x3 => {
                let parts: Vec<f32> = s.split_whitespace().map(|x| x.parse()).collect::<Result<Vec<_>, _>>()?;
                if parts.len() != 12 { anyhow::bail!("Mat3x4/Mat4x3 requires 12 values"); }
                let mut arr = [0.0; 12];
                arr.copy_from_slice(&parts);
                if matches!(attr_type, AttributeType::Mat3x4) {
                    Self::Mat3x4(arr)
                } else {
                    Self::Mat4x3(arr)
                }
            },
            AttributeType::Mat4 => {
                let parts: Vec<f32> = s.split_whitespace().map(|x| x.parse()).collect::<Result<Vec<_>, _>>()?;
                if parts.len() != 16 { anyhow::bail!("Mat4 requires 16 values"); }
                let mut arr = [0.0; 16];
                arr.copy_from_slice(&parts);
                Self::Mat4(arr)
            },
            AttributeType::Bool => Self::Bool(s == "True" || s == "true" || s == "1"),
            AttributeType::String | AttributeType::LSString => Self::String(s.to_string()),
            AttributeType::Path => Self::Path(s.to_string()),
            AttributeType::FixedString => Self::FixedString(s.to_string()),
            AttributeType::ULongLong => Self::ULongLong(s.parse()?),
            AttributeType::ScratchBuffer => Self::ScratchBuffer(base64::decode(s)?),
            AttributeType::LongLong => Self::LongLong(s.parse()?),
            AttributeType::Int8 => Self::Int8(s.parse()?),
            AttributeType::TranslatedString => {
                let parts: Vec<&str> = s.splitn(2, ';').collect();
                if parts.len() == 2 {
                    Self::TranslatedString { value: parts[0].to_string(), handle: parts[1].to_string() }
                } else {
                    Self::TranslatedString { value: s.to_string(), handle: String::new() }
                }
            },
            AttributeType::WString => Self::WString(s.to_string()),
            AttributeType::LSWString => Self::LSWString(s.to_string()),
            AttributeType::UUID => Self::UUID(Uuid::parse_str(s)?),
            AttributeType::Int64 => Self::Int64(s.parse()?),
            AttributeType::TranslatedFSString => {
                let parts: Vec<&str> = s.splitn(2, ';').collect();
                if parts.len() == 2 {
                    Self::TranslatedFSString { value: parts[0].to_string(), handle: parts[1].to_string() }
                } else {
                    Self::TranslatedFSString { value: s.to_string(), handle: String::new() }
                }
            },
        })
    }
}