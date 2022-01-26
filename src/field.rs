use arrow2::datatypes::{DataType, Field};
use chrono::{NaiveDate,NaiveDateTime};

/// Trait implemented by all types that can be used as an Arrow field.
/// 
/// Implementations are provided for types already supported by the arrow2 crate:
/// - numeric types: [`u8`], [`u16`], [`u32`], [`u64`], [`i8`], [`i16`], [`i32`], [`i64`], [`f32`], [`f64`]
/// - other types: [`bool`], [`String`]
/// - temporal types: [`chrono::NaiveDate`], [`chrono::NaiveDateTime`]
/// 
/// Custom implementations can be provided for other types.
/// 
/// The trait simply requires defining the [`ArrowField::data_type`]
/// 
/// Serialize and Deserialize functionality requires implementing the [`crate::ArrowSerialize`] 
/// and the [`crate::ArrowDeserialize`] traits respectively.
pub trait ArrowField: Sized
{
    /// The arrow data type. The default is the same as ArrowType
    fn data_type() -> DataType;

    #[inline]
    // for internal use
    fn field(name: &str) -> Field {
        Field::new(name, Self::data_type(), Self::is_nullable())
    }

    #[inline]
    // for internal use
    fn is_nullable() -> bool {
        false
    }
}

/// Enables the blanket implementations of [`Vec<T>`] as an Arrow field 
/// if `T` is an Arrow field.
/// 
/// This tag is needed for [`Vec<u8>`] specialization, and can be obviated
/// once implementation specialization is available in rust.
#[macro_export]
macro_rules! arrow_enable_vec_for_type {
    ($t:ty) => {
        impl $crate::ArrowEnableVecForType for $t {}
    };
}
/// Marker used to allow [`Vec<T>`] to be used as a [`ArrowField`]. 
#[doc(hidden)]
pub trait ArrowEnableVecForType {}

// Macro to facilitate implementation for numeric types.
macro_rules! impl_numeric_type {
    ($physical_type:ty, $logical_type:ident) => {
        impl ArrowField for $physical_type {
            #[inline]
            fn data_type() -> arrow2::datatypes::DataType {
                arrow2::datatypes::DataType::$logical_type
            }
        }
    };
}

// blanket implementation for optional fields
impl<T> ArrowField for Option<T>
where T: ArrowField,
{
    #[inline]
    fn data_type() -> arrow2::datatypes::DataType {
        <T as ArrowField>::data_type()
    }

    #[inline]
    fn is_nullable() -> bool {
        true
    }
}

impl_numeric_type!(u8, UInt8);
impl_numeric_type!(u16, UInt16);
impl_numeric_type!(u32, UInt32);
impl_numeric_type!(u64, UInt64);
impl_numeric_type!(i8, Int8);
impl_numeric_type!(i16, Int16);
impl_numeric_type!(i32, Int32);
impl_numeric_type!(i64, Int64);
impl_numeric_type!(f32, Float32);
impl_numeric_type!(f64, Float64);

impl ArrowField for String
{
    #[inline]
    fn data_type() -> arrow2::datatypes::DataType {
        arrow2::datatypes::DataType::Utf8
    }
}

impl ArrowField for bool
{
    #[inline]
    fn data_type() -> arrow2::datatypes::DataType {
        arrow2::datatypes::DataType::Boolean
    }
}

impl ArrowField for NaiveDateTime
{
    #[inline]
    fn data_type() -> arrow2::datatypes::DataType {
        arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Nanosecond, None)
    }
}

impl ArrowField for NaiveDate
{
    #[inline]
    fn data_type() -> arrow2::datatypes::DataType {
        arrow2::datatypes::DataType::Date32
    }
}

impl ArrowField for Vec<u8> {
    #[inline]
    fn data_type() -> arrow2::datatypes::DataType {
        arrow2::datatypes::DataType::Binary
    }
}

// Blanket implementation for Vec. 
impl<T> ArrowField for Vec<T>
where T: ArrowField + ArrowEnableVecForType
{
    #[inline]
    fn data_type() -> arrow2::datatypes::DataType {
        arrow2::datatypes::DataType::List(Box::new(
            <T as ArrowField>::field("item"),
        ))
    }
}

arrow_enable_vec_for_type!(String);
arrow_enable_vec_for_type!(bool);
arrow_enable_vec_for_type!(NaiveDateTime);
arrow_enable_vec_for_type!(NaiveDate);
arrow_enable_vec_for_type!(Vec<u8>);

// Blanket implementation for Vec<Option<T>> if vectors are enabled for T
impl<T> ArrowEnableVecForType for Option<T>
where T: ArrowField + ArrowEnableVecForType
{}

// Blanket implementation for Vec<Vec<T>> if vectors are enabled for T
impl<T> ArrowEnableVecForType for Vec<T>
where T: ArrowField + ArrowEnableVecForType,
{}
