// drive1.rs
//
// Execute `rustlings hint drive1` or use the `hint` watch subcommand for a
// hint.

// I AM  DONE

fn modify_by_address(address: usize) {
    let ptr=address as *mut u32;
    unsafe{
        *ptr =0xAABBCCDD;
    }
    
    // `address` is a memory address, there is an u32 at that address. try modify
    // the u32's value to 0xAABBCCDD
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t:u32 = 0x12345678;
        modify_by_address(&mut t as *mut u32 as usize);
        assert!(t == 0xAABBCCDD);
    }
}
