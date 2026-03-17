/// Constants used during serialization and deserialization.
/// According to Scratch devs, there shouldn't be a case where block is null, but shadow is present...
#[derive(PartialEq, Eq)]
pub enum InputShadowType {
    /// unobscured shadow
    InputSameBlockShadow = 1,
    /// no shadow
    InputBlockNoShadow = 2,
    /// obscured shadow
    InputDiffBlockShadow = 3,
}

/// Constants referring to 'primitive' blocks that are usually shadows,
/// or in the case of variables and lists, appear quite often in projects.
/// There's no reason these constants can't collide with [InputShadowType].
#[derive(PartialEq, Eq)]
pub enum PrimitiveTypes {
    /// math_number
    MathNumPrimitive = 4,
    /// math_positive_number
    PositiveNumPrimitive = 5,
    /// math_whole_number
    WholeNumPrimitive = 6,
    /// math_integer
    IntegerNumPrimitive = 7,
    /// math_angle
    AngleNumPrimitive = 8,
    /// colour_picker
    ColorPickerPrimitive = 9,
    /// text
    TextPrimitive = 10,
    /// event_broadcast_menu
    BroadcastPrimitive = 11,
    /// data_variable
    VarPrimitive = 12,
    /// data_listcontents
    ListPrimitive = 13,
}

impl TryFrom<u8> for InputShadowType {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(InputShadowType::InputSameBlockShadow),
            2 => Ok(InputShadowType::InputBlockNoShadow),
            3 => Ok(InputShadowType::InputDiffBlockShadow),
            n => Err(n),
        }
    }
}

impl TryFrom<u8> for PrimitiveTypes {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            4 => Ok(PrimitiveTypes::MathNumPrimitive),
            5 => Ok(PrimitiveTypes::PositiveNumPrimitive),
            6 => Ok(PrimitiveTypes::WholeNumPrimitive),
            7 => Ok(PrimitiveTypes::IntegerNumPrimitive),
            8 => Ok(PrimitiveTypes::AngleNumPrimitive),
            9 => Ok(PrimitiveTypes::ColorPickerPrimitive),
            10 => Ok(PrimitiveTypes::TextPrimitive),
            11 => Ok(PrimitiveTypes::BroadcastPrimitive),
            12 => Ok(PrimitiveTypes::VarPrimitive),
            13 => Ok(PrimitiveTypes::ListPrimitive),
            n => Err(n),
        }
    }
}
