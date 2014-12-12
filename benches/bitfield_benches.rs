///////////////////////////////////////////////////////////////////////////////
extern crate bits;
extern crate test;

use bits::bitfield::BitField;


///////////////////////////////////////////////////////////////////////////////
#[bench]
fn bench_insert(b: &mut test::Bencher)
{
    let mut bf = BitField::with_capacity(4);
    b.iter(|| assert!(bf.insert_u64(0xa5a5a5a5, 0, 31).is_ok()));
    assert!(bf.as_slice() == [0xa5, 0xa5, 0xa5, 0xa5]);
}

///////////////////////////////////////////////////////////////////////////////
#[bench]
fn bench_insert_non_byte_boundaries(b: &mut test::Bencher)
{
    let mut bf = BitField::with_capacity(4);
    b.iter(|| assert!(bf.insert_u64(0xa5a5a5a5, 5, 27).is_ok()));
    assert!(bf.as_slice() == [0x02, 0x5a, 0x5a, 0x50]);
}

///////////////////////////////////////////////////////////////////////////////
#[bench]
fn bench_retrieve(b: &mut test::Bencher)
{
    let mut bf = BitField::with_capacity(4);
    assert!(bf.insert_u64(0xa5a5a5a5, 0, 31).is_ok());
    assert!(bf.insert_u64(0xa5a5a5a5, 0, 31).is_ok());
    b.iter(|| assert!(bf.retrieve_u64(0, 31).unwrap() == 0xa5a5a5a5));
}

///////////////////////////////////////////////////////////////////////////////
#[bench]
fn bench_retrieve_non_byte_boundaries(b: &mut test::Bencher)
{
    let mut bf = BitField::with_capacity(4);        
    assert!(bf.insert_u64(0xa5a5a5a5, 0, 31).is_ok());
    b.iter(|| assert!(bf.retrieve_u64(5, 27).unwrap() == 0x005a5a5a));
}

///////////////////////////////////////////////////////////////////////////////
#[bench]
fn bench_right_shift(b: &mut test::Bencher)
{
    let mut bf = BitField::with_capacity(4);
    assert!(bf.insert_u64(0xa5a5a5a5, 0, 31).is_ok());
    b.iter(|| assert!((bf >> 7).as_slice() == [0x01, 0x4b, 0x4b, 0x4b]));
}

///////////////////////////////////////////////////////////////////////////////
#[bench]
fn bench_left_shift(b: &mut test::Bencher)
{
    let mut bf = BitField::with_capacity(4);
    assert!(bf.insert_u64(0xa5a5a5a5, 0, 31).is_ok());
    b.iter(|| assert!((bf << 7).as_slice() == [0xd2, 0xd2, 0xd2, 0x80]));
}

