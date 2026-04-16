/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Data Type Definitions (Language Layer)
//!
//! Provides cross-module reusable common data type enum `DataType` and type mapping `DataTypeOf`.
//!
//! # Author
//!
//! Haixing Hu

use bigdecimal::BigDecimal;
use chrono::{
    DateTime,
    NaiveDate,
    NaiveDateTime,
    NaiveTime,
    Utc,
};
use num_bigint::BigInt;
use serde::{
    Deserialize,
    Serialize,
};
use serde_json;
use std::collections::HashMap;
use std::time::Duration;
use parse_display::{Display, FromStr as DeriveFromStr};
use url::Url;

/// Universal data type enumeration for cross-module type representation
///
/// Defines all basic data types and composite types supported by the system.
/// This enum provides a unified way to represent and work with different data types
/// across various modules and components.
///
/// `DataType` serves as a bridge between Rust's type system and runtime type
/// information, enabling dynamic type handling, serialization, validation,
/// and other type-aware operations.
///
/// # Features
///
/// - **Comprehensive Coverage**: Supports all basic Rust types plus common third-party types
/// - **String Representation**: Each variant has a consistent string representation
/// - **Serialization Support**: Implements `Serialize` and `Deserialize` for JSON/YAML support
/// - **Type Mapping**: Works with `DataTypeOf` trait for compile-time type mapping
///
/// # Use Cases
///
/// - **Dynamic Type Handling**: Runtime type checking and conversion
/// - **Serialization/Deserialization**: Type-aware data format conversion
/// - **Validation Systems**: Type-based input validation
/// - **Generic Programming**: Type-safe generic operations
/// - **API Documentation**: Automatic type information generation
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust,ignore
/// use qubit_common::lang::DataType;
///
/// let data_type = DataType::Int32;
/// assert_eq!(data_type.to_string(), "int32");
/// assert_eq!(data_type.as_str(), "int32");
/// ```
///
/// ## Type Checking
///
/// ```rust,ignore
/// use qubit_common::lang::DataType;
///
/// fn is_numeric(data_type: DataType) -> bool {
///     matches!(data_type,
///         DataType::Int8 | DataType::Int16 | DataType::Int32 | DataType::Int64 | DataType::Int128 |
///         DataType::UInt8 | DataType::UInt16 | DataType::UInt32 | DataType::UInt64 | DataType::UInt128 |
///         DataType::Float32 | DataType::Float64 | DataType::BigInteger | DataType::BigDecimal
///     )
/// }
///
/// assert!(is_numeric(DataType::Int32));
/// assert!(!is_numeric(DataType::String));
/// ```
///
/// ## Serialization
///
/// ```rust,ignore
/// use qubit_common::lang::DataType;
/// use serde_json;
///
/// let data_type = DataType::Float64;
/// let json = serde_json::to_string(&data_type).unwrap();
/// assert_eq!(json, "\"float64\"");
///
/// let deserialized: DataType = serde_json::from_str(&json).unwrap();
/// assert_eq!(deserialized, DataType::Float64);
/// ```
///
/// # Author
///
/// Haixing Hu
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, DeriveFromStr)]
pub enum DataType {
    /// Boolean type
    #[display("bool")]
    #[from_str(regex = "(?i)bool")]
    Bool,
    /// Character type
    #[display("char")]
    #[from_str(regex = "(?i)char")]
    Char,
    /// 8-bit signed integer
    #[display("int8")]
    #[from_str(regex = "(?i)int8")]
    Int8,
    /// 16-bit signed integer
    #[display("int16")]
    #[from_str(regex = "(?i)int16")]
    Int16,
    /// 32-bit signed integer
    #[display("int32")]
    #[from_str(regex = "(?i)int32")]
    Int32,
    /// 64-bit signed integer
    #[display("int64")]
    #[from_str(regex = "(?i)int64")]
    Int64,
    /// 128-bit signed integer
    #[display("int128")]
    #[from_str(regex = "(?i)int128")]
    Int128,
    /// 8-bit unsigned integer
    #[display("uint8")]
    #[from_str(regex = "(?i)uint8")]
    UInt8,
    /// 16-bit unsigned integer
    #[display("uint16")]
    #[from_str(regex = "(?i)uint16")]
    UInt16,
    /// 32-bit unsigned integer
    #[display("uint32")]
    #[from_str(regex = "(?i)uint32")]
    UInt32,
    /// 64-bit unsigned integer
    #[display("uint64")]
    #[from_str(regex = "(?i)uint64")]
    UInt64,
    /// 128-bit unsigned integer
    #[display("uint128")]
    #[from_str(regex = "(?i)uint128")]
    UInt128,
    /// 32-bit floating point number
    #[display("float32")]
    #[from_str(regex = "(?i)float32")]
    Float32,
    /// 64-bit floating point number
    #[display("float64")]
    #[from_str(regex = "(?i)float64")]
    Float64,
    /// String type
    #[display("string")]
    #[from_str(regex = "(?i)string")]
    String,
    /// Date type (NaiveDate)
    #[display("date")]
    #[from_str(regex = "(?i)date")]
    Date,
    /// Time type (NaiveTime)
    #[display("time")]
    #[from_str(regex = "(?i)time")]
    Time,
    /// DateTime type (NaiveDateTime)
    #[display("datetime")]
    #[from_str(regex = "(?i)datetime")]
    DateTime,
    /// UTC time point (equivalent to Java Instant) (`DateTime<Utc>`) 
    #[display("instant")]
    #[from_str(regex = "(?i)instant")]
    Instant,
    /// Big integer type (BigInt)
    #[display("biginteger")]
    #[from_str(regex = "(?i)biginteger")]
    BigInteger,
    /// Big decimal type (BigDecimal)
    #[display("bigdecimal")]
    #[from_str(regex = "(?i)bigdecimal")]
    BigDecimal,
    /// Platform-dependent signed integer (isize)
    #[display("intsize")]
    #[from_str(regex = "(?i)intsize")]
    IntSize,
    /// Platform-dependent unsigned integer (usize)
    #[display("uintsize")]
    #[from_str(regex = "(?i)uintsize")]
    UIntSize,
    /// Duration type (std::time::Duration)
    #[display("duration")]
    #[from_str(regex = "(?i)duration")]
    Duration,
    /// URL type (url::Url)
    #[display("url")]
    #[from_str(regex = "(?i)url")]
    Url,
    /// String map type (HashMap<String, String>)
    #[display("stringmap")]
    #[from_str(regex = "(?i)stringmap")]
    StringMap,
    /// JSON value type (serde_json::Value)
    #[display("json")]
    #[from_str(regex = "(?i)json")]
    Json,
}

impl DataType {
    // Keep explicit conversion helper for existing call-sites and API parity with
    // current serialized/configuration text values.
    /// Get the string representation of the data type.
    ///
    /// # Returns
    ///
    /// Returns the name string of the data type.
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            DataType::Bool => "bool",
            DataType::Char => "char",
            DataType::Int8 => "int8",
            DataType::Int16 => "int16",
            DataType::Int32 => "int32",
            DataType::Int64 => "int64",
            DataType::Int128 => "int128",
            DataType::UInt8 => "uint8",
            DataType::UInt16 => "uint16",
            DataType::UInt32 => "uint32",
            DataType::UInt64 => "uint64",
            DataType::UInt128 => "uint128",
            DataType::Float32 => "float32",
            DataType::Float64 => "float64",
            DataType::String => "string",
            DataType::Date => "date",
            DataType::Time => "time",
            DataType::DateTime => "datetime",
            DataType::Instant => "instant",
            DataType::BigInteger => "biginteger",
            DataType::BigDecimal => "bigdecimal",
            DataType::IntSize => "intsize",
            DataType::UIntSize => "uintsize",
            DataType::Duration => "duration",
            DataType::Url => "url",
            DataType::StringMap => "stringmap",
            DataType::Json => "json",
        }
    }
}

// =============================================================================
// Compile-time mapping from types to DataType
// =============================================================================

/// Marker trait for mapping concrete Rust types to `DataType`
///
/// Provides an associated constant to know the corresponding `DataType` at compile time,
/// facilitating static type-to-data-type queries in generic code based on `T`.
///
/// This trait enables compile-time type-to-data-type mapping, allowing generic code
/// to determine the appropriate `DataType` for any type that implements this trait.
/// This is particularly useful for serialization frameworks, validation systems,
/// and other scenarios where you need to know the data type at compile time.
///
/// # Usage
///
/// The trait is automatically implemented for all basic Rust types and common
/// third-party types. You can use it in generic functions to determine the
/// corresponding `DataType` for any type.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust,ignore
/// use qubit_common::lang::{DataType, DataTypeOf};
///
/// // Get the data type for a specific type
/// assert_eq!(i32::DATA_TYPE, DataType::Int32);
/// assert_eq!(String::DATA_TYPE, DataType::String);
/// assert_eq!(bool::DATA_TYPE, DataType::Bool);
/// ```
///
/// ## Generic Function Example
///
/// ```rust,ignore
/// use qubit_common::lang::{DataType, DataTypeOf};
///
/// fn get_type_name<T: DataTypeOf>() -> &'static str {
///     T::DATA_TYPE.as_str()
/// }
///
/// assert_eq!(get_type_name::<i32>(), "int32");
/// assert_eq!(get_type_name::<String>(), "string");
/// assert_eq!(get_type_name::<f64>(), "float64");
/// ```
///
/// ## Generic Value Container Example
///
/// ```rust,ignore
/// use qubit_common::lang::{DataType, DataTypeOf};
///
/// struct TypedValue<T: DataTypeOf> {
///     value: T,
///     data_type: DataType,
/// }
///
/// impl<T: DataTypeOf> TypedValue<T> {
///     fn new(value: T) -> Self {
///         Self {
///             value,
///             data_type: T::DATA_TYPE,
///         }
///     }
///
///     fn get_data_type(&self) -> DataType {
///         self.data_type
///     }
/// }
///
/// let typed_value = TypedValue::new(42i32);
/// assert_eq!(typed_value.get_data_type(), DataType::Int32);
/// ```
///
/// ## Type Validation Example
///
/// ```rust,ignore
/// use qubit_common::lang::{DataType, DataTypeOf};
///
/// fn validate_numeric_type<T: DataTypeOf>() -> bool {
///     matches!(T::DATA_TYPE,
///         DataType::Int8 | DataType::Int16 | DataType::Int32 | DataType::Int64 | DataType::Int128 |
///         DataType::UInt8 | DataType::UInt16 | DataType::UInt32 | DataType::UInt64 | DataType::UInt128 |
///         DataType::Float32 | DataType::Float64 | DataType::BigInteger | DataType::BigDecimal
///     )
/// }
///
/// assert!(validate_numeric_type::<i32>());
/// assert!(validate_numeric_type::<f64>());
/// assert!(!validate_numeric_type::<String>());
/// ```
///
/// # Supported Types
///
/// The following types have `DataTypeOf` implementations:
///
/// - **Basic Types**: `bool`, `char`, `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`, `f32`, `f64`
/// - **String Types**: `String`
/// - **Date/Time Types**: `NaiveDate`, `NaiveTime`, `NaiveDateTime`, `DateTime<Utc>`
/// - **Big Number Types**: `BigInt`, `BigDecimal`
/// - **URL**: `url::Url`
///
/// # Author
///
/// Haixing Hu
///
pub trait DataTypeOf {
    /// The `DataType` corresponding to this Rust type
    const DATA_TYPE: DataType;
}

// Basic scalar types
impl DataTypeOf for bool {
    const DATA_TYPE: DataType = DataType::Bool;
}
impl DataTypeOf for char {
    const DATA_TYPE: DataType = DataType::Char;
}
impl DataTypeOf for i8 {
    const DATA_TYPE: DataType = DataType::Int8;
}
impl DataTypeOf for i16 {
    const DATA_TYPE: DataType = DataType::Int16;
}
impl DataTypeOf for i32 {
    const DATA_TYPE: DataType = DataType::Int32;
}
impl DataTypeOf for i64 {
    const DATA_TYPE: DataType = DataType::Int64;
}
impl DataTypeOf for i128 {
    const DATA_TYPE: DataType = DataType::Int128;
}
impl DataTypeOf for u8 {
    const DATA_TYPE: DataType = DataType::UInt8;
}
impl DataTypeOf for u16 {
    const DATA_TYPE: DataType = DataType::UInt16;
}
impl DataTypeOf for u32 {
    const DATA_TYPE: DataType = DataType::UInt32;
}
impl DataTypeOf for u64 {
    const DATA_TYPE: DataType = DataType::UInt64;
}
impl DataTypeOf for u128 {
    const DATA_TYPE: DataType = DataType::UInt128;
}
impl DataTypeOf for f32 {
    const DATA_TYPE: DataType = DataType::Float32;
}
impl DataTypeOf for f64 {
    const DATA_TYPE: DataType = DataType::Float64;
}

// String types
impl DataTypeOf for String {
    const DATA_TYPE: DataType = DataType::String;
}

// Date and time types (chrono)
impl DataTypeOf for NaiveDate {
    const DATA_TYPE: DataType = DataType::Date;
}
impl DataTypeOf for NaiveTime {
    const DATA_TYPE: DataType = DataType::Time;
}
impl DataTypeOf for NaiveDateTime {
    const DATA_TYPE: DataType = DataType::DateTime;
}
impl DataTypeOf for DateTime<Utc> {
    const DATA_TYPE: DataType = DataType::Instant;
}

// Big number types
impl DataTypeOf for BigInt {
    const DATA_TYPE: DataType = DataType::BigInteger;
}
impl DataTypeOf for BigDecimal {
    const DATA_TYPE: DataType = DataType::BigDecimal;
}

// Platform-dependent integer types
impl DataTypeOf for isize {
    const DATA_TYPE: DataType = DataType::IntSize;
}
impl DataTypeOf for usize {
    const DATA_TYPE: DataType = DataType::UIntSize;
}

// Standard library types
impl DataTypeOf for Duration {
    const DATA_TYPE: DataType = DataType::Duration;
}

impl DataTypeOf for Url {
    const DATA_TYPE: DataType = DataType::Url;
}

// String map type
impl DataTypeOf for HashMap<String, String> {
    const DATA_TYPE: DataType = DataType::StringMap;
}

// JSON value type
impl DataTypeOf for serde_json::Value {
    const DATA_TYPE: DataType = DataType::Json;
}
