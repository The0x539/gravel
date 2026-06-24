pub mod bitmap;
pub mod color;
pub mod geometry;
// pub mod gdraw;
// pub mod gpath;
// pub mod text;
// pub mod fonts;
// pub mod context;

#[repr(C)]
pub struct GContext(crate::util::Opaque);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum GCompOp {
    Assign,
    AssignInverted,
    Or,
    And,
    Clear,
    Set,
}
