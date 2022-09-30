/// Global Timer section
pub mod glb;
/// Hibernate section
pub mod hbn;
/// Power-down section
pub mod pds;
/// Interface for accessing functions from ROM
pub mod romfunc;

/// Error type definition - used by C SDK functions (and ROM functions by extension)
#[repr(C)]
pub enum BL_Err_Type {
    SUCCESS = 0,
    ERROR = 1,
    TIMEOUT = 2,
    INVALID = 3, /* invalid arguments */
    NORESC = 4,  /* no resource or resource temperary unavailable */
}
