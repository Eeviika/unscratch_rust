/// Constants used during serialization and deserialization.
/// According to Scratch devs, there shouldn't be a case where block is null, but shadow is present...
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
pub enum PrimitiveTypes {
    /// math_number
    MathNumPrimitive = 4, // there's no reason these constants can't collide
    /// math_positive_number
    PositiveNumPrimitive = 5, // with the above, but removing duplication for clarity
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
