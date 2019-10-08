/// A Windows application icon
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Icon {
    pub data: Vec<u8>,
    pub length: u32,
    pub width: u32,
    pub height: u32,
}