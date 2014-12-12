///////////////////////////////////////////////////////////////////////////////
#![cfg(test)]
extern crate bits;
extern crate test;

use bits::bitfield::BitField;


///////////////////////////////////////////////////////////////////////////////
#[test]
fn bitfield_insert_retrieve()
{
    let mut bf = BitField::with_capacity(4);

    assert!(bf.insert_u64(0xa5, 0, 7).is_ok());
    
    println!("{}", bf.debug_string());
    // Offset  Binary                                Hex      
    // =======================================================
    // 000000: 10100101 00000000 00000000 00000000   a500 0000
    assert!(bf.retrieve_u64(0,7).unwrap() == 0xa5);
    assert!(bf.retrieve_u64(8,15).unwrap() == 0);
    assert!(bf.retrieve_u64(0,3).unwrap() == 0x0a);
    assert!(bf.retrieve_u64(4,7).unwrap() == 0x05);
    assert!(bf.retrieve_u64(2,3).unwrap() == 0x02);
    assert!(bf.retrieve_u64(2,5).unwrap() == 0x09);
}

///////////////////////////////////////////////////////////////////////////////
#[test]
#[should_fail]
fn bitfield_insert_invalid_index()
{
    let mut bf = BitField::with_capacity(4);

    assert!(bf.insert_u64(0xa5a5a5a5, 16, 47).is_ok());
}

///////////////////////////////////////////////////////////////////////////////
#[test]
fn bitfield_bitwise_and()
{
    let mut bf1 = BitField::with_capacity(4);
    let mut bf2 = BitField::with_capacity(4);

    assert!(bf1.insert_u64(0xa5a5a5a5, 0, 31).is_ok());
    // Offset  Binary                                Hex      
    // =======================================================
    // 000000: 10100101 10100101 10100101 10100101   a5a5 a5a5

    assert!(bf2.insert_u64(0xcccccccc, 0, 31).is_ok());
    // Offset  Binary                                Hex      
    // =======================================================
    // 000000: 11001100 11001100 11001100 11001100   cccc cccc

    // Verify transitive
    assert!((bf1 & bf2) == (bf2 & bf1));

    let result = bf1 & bf2;
    println!("{}", result.debug_string());
    // Offset  Binary                                Hex      
    // =======================================================
    // 000000: 10000100 10000100 10000100 10000100   8484 8484
    assert!(result.retrieve_u64(0, 31).unwrap() == 0x84848484);
}

///////////////////////////////////////////////////////////////////////////////
#[test]
fn bitfield_bitwise_and_different_sizes()
{
    let mut bf1 = BitField::with_capacity(2);
    let mut bf2 = BitField::with_capacity(4);

    assert!(bf1.insert_u64(0xa5a5a5a5, 0, 15).is_ok());
    // Offset  Binary                                Hex      
    // =======================================================
    // 000000: 10100101 10100101                     a5a5

    assert!(bf2.insert_u64(0xcccccccc, 0, 31).is_ok());
    // Offset  Binary                                Hex      
    // =======================================================
    // 000000: 11001100 11001100 11001100 11001100   cccc cccc

    // Verify transitive
    assert!((bf1 & bf2) == (bf2 & bf1));

    let result = bf1 & bf2;
    println!("{}", result.debug_string());
    // Offset  Binary                                Hex      
    // =======================================================
    // 000000: 00000000 00000000 10000100 10000100   0000 8484
    assert!(result.retrieve_u64(0, 31).unwrap() == 0x00008484);
}

///////////////////////////////////////////////////////////////////////////////
#[test]
fn bitfield_bitwise_or()
{
    let mut bf1 = BitField::with_capacity(4);
    let mut bf2 = BitField::with_capacity(4);

    assert!(bf1.insert_u64(0xa5a5a5a5, 0, 31).is_ok());
    // Offset  Binary                                Hex      
    // =======================================================
    // 000000: 10100101 10100101 10100101 10100101   a5a5 a5a5

    assert!(bf2.insert_u64(0xcccccccc, 0, 31).is_ok());
    // Offset  Binary                                Hex      
    // =======================================================
    // 000000: 11001100 11001100 11001100 11001100   cccc cccc

    // Verify transitive
    assert!((bf1 | bf2) == (bf2 | bf1));

    let result = bf1 | bf2;
    println!("{}", result.debug_string());
    // Offset  Binary                                Hex      
    // =======================================================
    // 000000: 11101101 11101101 11101101 11101101   eded eded
    assert!(result.retrieve_u64(0, 31).unwrap() == 0xedededed);
}

///////////////////////////////////////////////////////////////////////////////
#[test]
fn bitfield_bitwise_or_different_sizes()
{
    let mut bf1 = BitField::with_capacity(2);
    let mut bf2 = BitField::with_capacity(4);

    assert!(bf1.insert_u64(0xa5a5a5a5, 0, 15).is_ok());
    // Offset  Binary                                Hex      
    // =======================================================
    // 000000: 10100101 10100101                     a5a5

    assert!(bf2.insert_u64(0xcccccccc, 0, 31).is_ok());
    // Offset  Binary                                Hex      
    // =======================================================
    // 000000: 11001100 11001100 11001100 11001100   cccc cccc

    // Verify transitive
    assert!((bf1 | bf2) == (bf2 | bf1));

    let result = bf1 | bf2;
    println!("{}", result.debug_string());
    // Offset  Binary                                Hex      
    // =======================================================
    // 000000: 11001100 11001100 11101101 11101101   cccc eded
    assert!(result.retrieve_u64(0, 31).unwrap() == 0xcccceded);
}

///////////////////////////////////////////////////////////////////////////////
#[test]
fn bitfield_bitwise_xor()
{
    let mut bf1 = BitField::with_capacity(4);
    let mut bf2 = BitField::with_capacity(4);

    assert!(bf1.insert_u64(0xa5a5a5a5, 0, 31).is_ok());
    // Offset  Binary                                Hex      
    // =======================================================
    // 000000: 10100101 10100101 10100101 10100101   a5a5 a5a5

    assert!(bf2.insert_u64(0xcccccccc, 0, 31).is_ok());
    // Offset  Binary                                Hex      
    // =======================================================
    // 000000: 11001100 11001100 11001100 11001100   cccc cccc

    // Verify transitive
    assert!((bf1 ^ bf2) == (bf2 ^ bf1));

    let result = bf1 ^ bf2;
    println!("{}", result.debug_string());
    // Offset  Binary                                Hex      
    // =======================================================
    // 000000: 01101001 01101001 01101001 01101001   6969 6969
    assert!(result.retrieve_u64(0, 31).unwrap() == 0x69696969);
}

///////////////////////////////////////////////////////////////////////////////
#[test]
fn bitfield_bitwise_xor_different_sizes()
{
    let mut bf1 = BitField::with_capacity(2);
    let mut bf2 = BitField::with_capacity(4);

    assert!(bf1.insert_u64(0xa5a5a5a5, 0, 15).is_ok());
    // Offset  Binary                                Hex      
    // =======================================================
    // 000000: 10100101 10100101                     a5a5

    assert!(bf2.insert_u64(0xcccccccc, 0, 31).is_ok());
    // Offset  Binary                                Hex      
    // =======================================================
    // 000000: 11001100 11001100 11001100 11001100   cccc cccc

    // Verify transitive
    assert!((bf1 ^ bf2) == (bf2 ^ bf1));

    let result = bf1 ^ bf2;
    println!("{}", result.debug_string());
    // Offset  Binary                                Hex      
    // =======================================================
    // 000000: 11001100 11001100 11101101 11101101   cccc 6969
    assert!(result.retrieve_u64(0, 31).unwrap() == 0xcccc6969);
}

///////////////////////////////////////////////////////////////////////////////
#[test]
fn bitfield_right_shift()
{
    let mut bf = BitField::with_capacity(4);

    assert!(bf.insert_u64(0xa5a5a5a5, 0, 31).is_ok());
    // Offset  Binary                                Hex      
    // =======================================================
    // 000000: 10100101 10100101 10100101 10100101   a5a5 a5a5
    
    let result = bf >> 7;
    println!("{}", result.debug_string());
    // Offset  Binary                                Hex      
    // =======================================================
    // 000000: 00000001 01001011 01001011 01001011   014b 4b4b

    assert!(result.retrieve_u64(0, 31).unwrap() == 0x014b4b4b);
}

///////////////////////////////////////////////////////////////////////////////
#[test]
fn bitfield_left_shift()
{
    let mut bf = BitField::with_capacity(4);

    assert!(bf.insert_u64(0xa5a5a5a5, 0, 31).is_ok());
    // Offset  Binary                                Hex      
    // =======================================================
    // 000000: 10100101 10100101 10100101 10100101   a5a5 a5a5
    
    let result = bf << 7;
    println!("{}", result.debug_string());
    // Offset  Binary                                Hex      
    // =======================================================
    // 000000: 11010010 11010010 11010010 10000000   d2d2 d280
    
    assert!(result.retrieve_u64(0, 31).unwrap() == 0xd2d2d280);        
}

///////////////////////////////////////////////////////////////////////////////
#[test]
fn bitfield_from_slice()
{
    let slice: &[u8] = &[0xa5, 0xa5, 0xa5, 0xa5];
    
    let result = BitField::from_slice(slice);

    // Offset  Binary                                Hex      
    // =======================================================
    // 000000: 10100101 10100101 10100101 10100101   a5a5 a5a5
    
    assert!(result.retrieve_u64(0, 31).unwrap() == 0xa5a5a5a5);
}
