#![cfg(not(test))]
extern crate bits;

use bits::bitfield::BitField;
///////////////////////////////////////////////////////////////////////////////
fn main()
{
    let mut bitfield = BitField::with_capacity(10);

    for i in range(0, 8)
    {
        for j in range(0, 65536)
        {
            //println!("{}", bitfield.debug_string());
            assert!(bitfield.insert_u64(j, i, i + 15).is_ok());
            match bitfield.retrieve_u64(i, i + 15)
            {
                Ok(value) if value == j => (),
                Ok(value) =>
                    println!("FAULT\ni = {}\nj = {}\nvalue = {}", i, j, value),
                Err(e) => println!("Error: {}", e)
            }
        }
    }
}
