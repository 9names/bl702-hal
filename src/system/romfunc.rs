use self::data::ROM_API_INDEX_e;
use self::data::ROM_APITABLE_ADDR;

pub mod data;

/// Look up the address of the ROM function in the function table.
/// Generate a function pointer from that
///
/// # Safety
///
/// The function pointer needs to be transmuted to the correct signature - take care to match the signature correctly
/// or memory corruption or other UB could occur
pub unsafe fn rom_fn_ptr(func: ROM_API_INDEX_e) -> *const () {
    let rom_function_table_base = ROM_APITABLE_ADDR as *mut usize;
    let func_entry = rom_function_table_base.wrapping_add(func as usize);
    let func_addr = func_entry.read_volatile();
    func_addr as *const ()
}
