use std::{cmp, fmt, uint, u64, u32, u16, u8};

///////////////////////////////////////////////////////////////////////////////
#[deriving(Show)]
pub enum BitFieldError
{
    InternalError,
    InvalidIndex,
    NegativeRange,
    ExceededDataRange,
}

///////////////////////////////////////////////////////////////////////////////
#[deriving(Clone,PartialEq)]
pub struct BitField
{
    bytes: Vec<u8>
}

///////////////////////////////////////////////////////////////////////////////
impl BitField
{
    ///////////////////////////////////////////////////////////////////////////
    pub fn new() -> BitField
    {
        BitField{bytes: Vec::new()}
    }

    ///////////////////////////////////////////////////////////////////////////
    pub fn with_capacity(capacity: uint) -> BitField
    {
        BitField{bytes: Vec::from_elem(capacity, 0)}
    }

    ///////////////////////////////////////////////////////////////////////////
    pub fn from_slice(values: &[u8]) -> BitField
    {
        let mut bitfield = BitField::new();
        bitfield.bytes.push_all(values);
        bitfield
    }

    ///////////////////////////////////////////////////////////////////////////
    pub fn clear(&mut self)
    {
        self.bytes.clear()
    }

    ///////////////////////////////////////////////////////////////////////////
    pub fn len(&self) -> uint
    {
        self.bytes.len()
    }

    ///////////////////////////////////////////////////////////////////////////
    pub fn is_empty(&self) -> bool
    {
        self.bytes.is_empty()
    }

    ///////////////////////////////////////////////////////////////////////////
    pub fn grow(&mut self, size: uint)
    {
        self.bytes.grow(size, 0u8);
    }

    ///////////////////////////////////////////////////////////////////////////
    fn get_mask(bits: uint) -> u8
    {
        ((1u32 << bits) - 1u32) as u8
    }

    /*
    pub fn insert<T:AsSlice<u8>>(&mut self,
                                 value: T,
                                 start_bit:
                                 uint,
                                 stop_bit: uint) ->
        Result<(), &'static str>
    {
        // find out which byte range will be affected by the insert
        let start_byte = start_bit / 8;
        let stop_byte  = stop_bit / 8;

        if start_bit > stop_bit
        {
            return Err("Bit range can not be negative");
        }
        
        // find the byte length for the data
        let byte_length = (stop_byte - start_byte) + 1;
        
        if self.len() <= stop_byte
        {
            return Err("Bits are out of range for object")
        }

        let data: &[u8] = value.as_slice();

        if byte_length > data.len()
        {
            return Err("Bit range can not exceed data length");
        }

        //for b in 
        Ok(())
 
    }
    */

    ///////////////////////////////////////////////////////////////////////////
    /// Inserts the value into the object at the specified bit locations
    /// 
    /// <p>The passed in <b>value</b> will be masked against the
    /// range specified by <b>start_bit</b> and <b>stop_bit</b>.  For example, 
    /// if <b>value</b> = 6 (binary 0110), and <b>start_bit</b> = 5, and 
    /// <b>stop_bit</b> = 6, the <b>value</b> will be masked to 2 (binary 10).<p>
    pub fn insert_u64(&mut self, value: u64, start_bit: uint, stop_bit: uint) ->
                                                       Result<(), BitFieldError>
    {
        // find out which byte range will be affected by the insert
        let start_byte = start_bit / 8;
        let stop_byte  = stop_bit / 8;

        if start_bit > stop_bit
        {
            return Err(BitFieldError::NegativeRange);
        }
        
        // find the byte length for the data
        let byte_length = (stop_byte - start_byte) + 1;
        
        if self.len() <= stop_byte
        {
            return Err(BitFieldError::InvalidIndex)
        }

        if (stop_bit - start_bit + 1) > u64::BITS
        {
            return Err(BitFieldError::ExceededDataRange);
        }

        // loop through the bytes to change the value of
        for i in range(0, byte_length)
        {
            // find the start bit for the current byte
            let mut current_start_bit = 8 * (start_byte + i);
            if current_start_bit < start_bit
            {
                current_start_bit = start_bit;
            }
            // find the stop bit for the current byte
            let end_of_byte_bit = 8*(start_byte + i + 1) - 1;
            let current_stop_bit =
                if end_of_byte_bit > stop_bit
                {
                    stop_bit
                }
                else
                {
                    end_of_byte_bit
                };
            
            // Find the current byte
            let current_byte = start_byte + i;
            
            // Number bits in this byte that are going to be changed
            let bits_in_byte = current_stop_bit - current_start_bit + 1;
            
            // Find the number bits to shift
            let shift_bits = stop_bit - current_stop_bit;
            
            // Find mask and value for this byte
            let mut byte_mask = BitField::get_mask(bits_in_byte);
            let mut byte_value = (value >> shift_bits) as u8 & byte_mask;
            
            // If the current stop bit is not the end of the byte, then the
            // value needs to be shifted by the difference
            if i == (byte_length - 1)
            {
                let shift = end_of_byte_bit - current_stop_bit;
                byte_mask = byte_mask << shift;
                byte_value = byte_value << shift;
            }
            
            // stores the value by clearing the selected portion of the byte
            // with negation of the mask then inserts the value in the byte.
            match self.bytes.get_mut(current_byte)
            {
                Some(b) => *b = *b & !byte_mask | byte_value,
                None => panic!("Current byte is out of range")
            };
        }

        Ok(())
    }

    ///////////////////////////////////////////////////////////////////////////
    pub fn insert_uint(&mut self, value: uint, start_bit: uint, stop_bit: uint) ->
                                                       Result<(), BitFieldError>
    {
        if (stop_bit - start_bit + 1) > uint::BITS
        {
            return Err(BitFieldError::ExceededDataRange);
        }

        self.insert_u64(value as u64, start_bit, stop_bit)
    }

    ///////////////////////////////////////////////////////////////////////////
    pub fn insert_u32(&mut self, value: u32, start_bit: uint, stop_bit: uint) ->
                                                       Result<(), BitFieldError>
    {
        if (stop_bit - start_bit + 1) > u32::BITS
        {
            return Err(BitFieldError::ExceededDataRange);
        }

        self.insert_u64(value as u64, start_bit, stop_bit)
    }

    ///////////////////////////////////////////////////////////////////////////
    pub fn insert_u16(&mut self, value: u16, start_bit: uint, stop_bit: uint) ->
                                                       Result<(), BitFieldError>
    {
        if (stop_bit - start_bit + 1) > u16::BITS
        {
            return Err(BitFieldError::ExceededDataRange);
        }

        self.insert_u64(value as u64, start_bit, stop_bit)
    }

    ///////////////////////////////////////////////////////////////////////////
    pub fn insert_u8(&mut self, value: u16, start_bit: uint, stop_bit: uint) ->
                                                       Result<(), BitFieldError>
    {
        if (stop_bit - start_bit + 1) > u8::BITS
        {
            return Err(BitFieldError::ExceededDataRange);
        }

        self.insert_u64(value as u64, start_bit, stop_bit)
    }

    ///////////////////////////////////////////////////////////////////////////
    /// Gets the value of the specified range of bits.
    pub fn retrieve_u64(&self, start_bit: uint, stop_bit: uint) ->
                                                     Result<u64, BitFieldError>
    {
        // find out which byte range will be affected by the insert
        let start_byte = start_bit / 8;
        let stop_byte  = stop_bit / 8;

        if start_bit > stop_bit
        {
            return Err(BitFieldError::NegativeRange);
        }
        
        // find the byte length for the data
        let byte_length = (stop_byte - start_byte) + 1;
        
        if self.len() <= stop_byte
        {
            return Err(BitFieldError::InvalidIndex)
        }

        if (stop_bit - start_bit + 1) > u64::BITS
        {
            return Err(BitFieldError::ExceededDataRange);
        }

        let mut value = 0u64;

        // loop through the bytes to change the value of
        for i in range(0, byte_length)
        {
            // find the start bit for the current byte
            let mut current_start_bit = 8 * (start_byte + i);
            if current_start_bit < start_bit
            {
                current_start_bit = start_bit;
            }
            // find the stop bit for the current byte
            let end_of_byte_bit = 8 * (start_byte + i + 1) - 1;
            let current_stop_bit =
                if end_of_byte_bit > stop_bit
                {
                    stop_bit
                }
                else
                {
                    end_of_byte_bit
                };
            
            // Find the current byte
            let current_byte = start_byte + i;
            
            // Number bits in this byte that are going to be changed
            let bits_in_byte = current_stop_bit - current_start_bit + 1;
            
            // Find mask and value for this byte
            let mut byte_mask = BitField::get_mask(bits_in_byte);
            
            // If the current stop bit is not the end of the byte, then the
            // value needs to be shifted by the difference
            if i == (byte_length - 1)
            {
                let shift = end_of_byte_bit - current_stop_bit;

                byte_mask = byte_mask << shift;
                let tmp = (self.bytes[current_byte] & byte_mask) >> shift;
                value = (value << bits_in_byte) | tmp as u64;
            }
            else
            {
                value = (value << bits_in_byte) |
                               (self.bytes[current_byte] & byte_mask) as u64;
            }
        }
        
        Ok(value)
    }

    ///////////////////////////////////////////////////////////////////////////
    pub fn retrieve_uint(&self, start_bit: uint, stop_bit: uint) ->
                                                     Result<uint, BitFieldError>
    {
        if (stop_bit - start_bit + 1) > uint::BITS
        {
            return Err(BitFieldError::ExceededDataRange);
        }
        
        match self.retrieve_u64(start_bit, stop_bit)
        {
            Ok(v) => Ok(v as uint),
            Err(e) => Err(e)
        }

    }

    ///////////////////////////////////////////////////////////////////////////
    pub fn retrieve_u32(&self, start_bit: uint, stop_bit: uint) ->
                                                     Result<u32, BitFieldError>
    {
        if (stop_bit - start_bit + 1) > u32::BITS
        {
            return Err(BitFieldError::ExceededDataRange);
        }
        
        match self.retrieve_u64(start_bit, stop_bit)
        {
            Ok(v) => Ok(v as u32),
            Err(e) => Err(e)
        }

    }

    ///////////////////////////////////////////////////////////////////////////
    pub fn retrieve_u16(&self, start_bit: uint, stop_bit: uint) ->
                                                     Result<u16, BitFieldError>
    {
        if (stop_bit - start_bit + 1) > u16::BITS
        {
            return Err(BitFieldError::ExceededDataRange);
        }
        
        match self.retrieve_u64(start_bit, stop_bit)
        {
            Ok(v) => Ok(v as u16),
            Err(e) => Err(e)
        }

    }

    ///////////////////////////////////////////////////////////////////////////
    pub fn retrieve_u8(&self, start_bit: uint, stop_bit: uint) ->
                                                     Result<u8, BitFieldError>
    {
        if (stop_bit - start_bit + 1) > u8::BITS
        {
            return Err(BitFieldError::ExceededDataRange);
        }
        
        match self.retrieve_u64(start_bit, stop_bit)
        {
            Ok(v) => Ok(v as u8),
            Err(e) => Err(e)
        }

    }

    ///////////////////////////////////////////////////////////////////////////
    pub fn debug_string(&self) -> String
    {
        let mut s = String::new();

        s.push_str("Offset  Binary                                Hex      \n");
        s.push_str("=======================================================\n");

        let mut i = 0;

        while i < (self.len() / 4)
        {
            s.push_str(format!("{:06}: ", i * 4).as_slice());
            
            s.push_str(
                format!("{:08b} {:08b} {:08b} {:08b}   ",
                        self.bytes[i * 4],
                        self.bytes[i * 4 + 1],
                        self.bytes[i * 4 + 2],
                        self.bytes[i * 4 + 3]).as_slice());

            s.push_str(
                format!("{:02x}{:02x} {:02x}{:02x}\n",
                        self.bytes[i * 4],
                        self.bytes[i * 4 + 1],
                        self.bytes[i * 4 + 2],
                        self.bytes[i * 4 + 3]).as_slice());
         
            i = i + 1;
        }

        if i * 4 != self.len()
        {
            let mut second_bits = "".to_string();
            let mut second_byte = "".to_string();
            let mut third_bits = "".to_string();
            let mut third_byte = "".to_string();

            if (self.len() % 4) > 1
            {
                second_bits = format!("{:08b}", self.bytes[i * 4 + 1]);
                second_byte = format!("{:02x}", self.bytes[i * 4 + 1]);
            }
            
            if (self.len() % 4) > 2
            {
                third_bits = format!("{:08b}", self.bytes[i * 4 + 2]);
                third_byte = format!("{:02x}", self.bytes[i * 4 + 2]);
            }
        
            s.push_str(format!("{:06}: ", i * 4).as_slice());
            
            s.push_str(format!("{:08b} {:08} {:08} {:08}   ",
                               self.bytes[i * 4],
                               second_bits,
                               third_bits,
                               "").as_slice());

            s.push_str(format!("{:02x}{:02} {:02}{:02}\n",
                               self.bytes[i * 4],
                               second_byte,
                               third_byte,
                               "").as_slice());
        }

        s
    }
    
    ///////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////

}
   
///////////////////////////////////////////////////////////////////////////////
impl BitAnd<BitField, BitField> for BitField
{
    ///////////////////////////////////////////////////////////////////////////
    /// Bitwise AND two BitField Objects together.
    ///
    /// LIMITATION:
    /// If the two BitField Objects differ in size the bitwise AND will start
    /// with the last byte and continue until one of the objects first byte is
    /// reached.
    /// (e.g)
    /// BitField1 containing 2 bytes set to FF and FF (0xFFFF) bitwise AND with
    /// BitField2 containting 1 byte set to 00 (0x55) will result in
    ///
    /// BitField1 & BitField2 returns
    ///       BitField containg 2 bytes set to 00 and 55 (0x0055)
    /// BitField2 & BitField1 returns
    ///       BitField containg 2 bytes set to 00 and 55 (0x0055)
    ///
    fn bitand(&self, _rhs: &BitField) -> BitField
    {
        let len = cmp::max(self.len(), _rhs.len());
        let mut result = BitField::with_capacity(len);
        let (mut i, mut j, mut k) = (self.len() - 1,
                                     _rhs.len() - 1,
                                     len - 1);
        loop
        {
            if i == -1 || j == -1
            {
                break;
            }

            match result.bytes.get_mut(k)
            {
                Some(b) => *b = self.bytes[i] & _rhs.bytes[j],
                None => panic!("I don't know we got here...")
            };

            i = i - 1;
            j = j - 1;
            k = k - 1;
        }
        
        result
    }
}

///////////////////////////////////////////////////////////////////////////////
impl BitOr<BitField, BitField> for BitField
{
    ///////////////////////////////////////////////////////////////////////////
    fn bitor(&self, _rhs: &BitField) -> BitField
    {
        let mut result =
            if self.len() < _rhs.len()
            {
                _rhs.clone()
            }
            else
            {
                self.clone()
            };

        let (mut i, mut j, mut k) = (self.len() - 1,
                                     _rhs.len() - 1,
                                     result.len() - 1);
        loop
        {
            if i == -1 || j == -1
            {
                break;
            }

            match result.bytes.get_mut(k)
            {
                Some(b) => *b = self.bytes[i] | _rhs.bytes[j],
                None => panic!("I don't know we got here...")
            };

            i = i - 1;
            j = j - 1;
            k = k - 1;
        }
        
        result
    }
}

///////////////////////////////////////////////////////////////////////////////
impl BitXor<BitField, BitField> for BitField
{
    ///////////////////////////////////////////////////////////////////////////
    fn bitxor(&self, _rhs: &BitField) -> BitField
    {
        let mut result =
            if self.len() < _rhs.len()
            {
                _rhs.clone()
            }
            else
            {
                self.clone()
            };

        let (mut i, mut j, mut k) = (self.len() - 1,
                                     _rhs.len() - 1,
                                     result.len() - 1);
        loop
        {
            if i == -1 || j == -1
            {
                break;
            }

            match result.bytes.get_mut(k)
            {
                Some(b) => *b = self.bytes[i] ^ _rhs.bytes[j],
                None => panic!("I don't know we got here...")
            };

            i = i - 1;
            j = j - 1;
            k = k - 1;
        }
        
        result
    }
}

///////////////////////////////////////////////////////////////////////////////
impl Index<uint, u8> for BitField
{
    ///////////////////////////////////////////////////////////////////////////
    fn index<'a>(&'a self, index: &uint) -> &'a u8
    {
        &self.bytes[*index]
    }
}

///////////////////////////////////////////////////////////////////////////////
impl IndexMut<uint, u8> for BitField
{
    ///////////////////////////////////////////////////////////////////////////
    fn index_mut<'a>(&'a mut self, index: &uint) -> &'a mut u8
    {
        &mut self.bytes.as_mut_slice()[*index]
    }
}

///////////////////////////////////////////////////////////////////////////////
impl Not<BitField> for BitField
{
    ///////////////////////////////////////////////////////////////////////////
    fn not(&self) -> BitField
    {
        
        let mut result = BitField::with_capacity(self.len());

        for i in range(0, self.len())
        {
            match result.bytes.get_mut(i)
            {
                Some(b) => *b = !self.bytes[i],
                None => panic!("I don't know we got here...")
            };
        }

        result
    }
}

///////////////////////////////////////////////////////////////////////////////
impl Shr<uint, BitField> for BitField
{
    ///////////////////////////////////////////////////////////////////////////
    fn shr(&self, _rhs: &uint) -> BitField
    {
        let mut result = BitField::with_capacity(self.len());

        let (mut i, mut j) = (0, *_rhs);

        while j < (self.len() * 8)
        {
            if (j + 7) > (self.len() * 8 - 1)
            {
                let partial = (self.len() * 8 -1) -j;
                
                result.insert_u64(self.retrieve_u64(i, i + partial).unwrap(),
                              j, j + partial).unwrap();
            }
            else
            {
                result.insert_u64(self.retrieve_u64(i, i + 7).unwrap(),
                              j, j + 7).unwrap();
            }

            i = i + 8;
            j = j + 8;
        }

        result
    }
}

///////////////////////////////////////////////////////////////////////////////
impl Shl<uint, BitField> for BitField
{
    ///////////////////////////////////////////////////////////////////////////
    fn shl(&self, _rhs: &uint) -> BitField
    {
        let mut result = BitField::with_capacity(self.len());

        let (mut i, mut j) = (0, *_rhs);

        while j < (self.len() * 8)
        {
            if (j + 7) > (self.len() * 8 - 1)
            {
                let partial = (self.len() * 8 -1) -j;
                
                result.insert_u64(self.retrieve_u64(j, j + partial).unwrap(),
                              i, i + partial).unwrap();
            }
            else
            {
                result.insert_u64(self.retrieve_u64(j, j + 7).unwrap(),
                              i, i + 7).unwrap();
            }

            i = i + 8;
            j = j + 8;
        }

        result
    }
}

///////////////////////////////////////////////////////////////////////////////
impl AsSlice<u8> for BitField
{
    ///////////////////////////////////////////////////////////////////////////
    fn as_slice<'a>(&'a self) -> &'a [u8]
    {
        self.bytes.as_slice()
    }
}

///////////////////////////////////////////////////////////////////////////////
impl fmt::Show for BitField
{
    ///////////////////////////////////////////////////////////////////////////
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result
    {
        let mut i = 0;
        while i < self.len() - 1
        {
            if i != 0 { try!(write!(fmt, " ")); }
            try!(write!(fmt, "{:02x}", self.bytes[i]));
            try!(write!(fmt, "{:02x}", self.bytes[i + 1]));
            i = i + 2;
        }

        if i == self.len() - 1
        {
            if i != 0 { try!(write!(fmt, " ")); }
            try!(write!(fmt, "{:02x}", self.bytes[i]));
        }

        Ok(())
    }
}

///////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests
{
    extern crate test;

    use super::BitField;

    ///////////////////////////////////////////////////////////////////////////
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

    ///////////////////////////////////////////////////////////////////////////
    #[test]
    #[should_fail]
    fn bitfield_insert_invalid_index()
    {
        let mut bf = BitField::with_capacity(4);

        assert!(bf.insert_u64(0xa5a5a5a5, 16, 47).is_ok());
    }

    ///////////////////////////////////////////////////////////////////////////
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

    ///////////////////////////////////////////////////////////////////////////
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

    ///////////////////////////////////////////////////////////////////////////
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

    ///////////////////////////////////////////////////////////////////////////
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

    ///////////////////////////////////////////////////////////////////////////
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

    ///////////////////////////////////////////////////////////////////////////
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

    ///////////////////////////////////////////////////////////////////////////
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

    ///////////////////////////////////////////////////////////////////////////
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

    ///////////////////////////////////////////////////////////////////////////
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

    ///////////////////////////////////////////////////////////////////////////
    #[bench]
    fn bench_insert(b: &mut test::Bencher)
    {
        let mut bf = BitField::with_capacity(4);
        b.iter(|| assert!(bf.insert_u64(0xa5a5a5a5, 0, 31).is_ok()));
        assert!(bf.as_slice() == [0xa5, 0xa5, 0xa5, 0xa5]);
    }
            
    ///////////////////////////////////////////////////////////////////////////
    #[bench]
    fn bench_insert_non_byte_boundaries(b: &mut test::Bencher)
    {
        let mut bf = BitField::with_capacity(4);
        b.iter(|| assert!(bf.insert_u64(0xa5a5a5a5, 5, 27).is_ok()));
        assert!(bf.as_slice() == [0x02, 0x5a, 0x5a, 0x50]);
    }
            
    ///////////////////////////////////////////////////////////////////////////
    #[bench]
    fn bench_retrieve(b: &mut test::Bencher)
    {
        let mut bf = BitField::with_capacity(4);
        assert!(bf.insert_u64(0xa5a5a5a5, 0, 31).is_ok());
        assert!(bf.insert_u64(0xa5a5a5a5, 0, 31).is_ok());
        b.iter(|| assert!(bf.retrieve_u64(0, 31).unwrap() == 0xa5a5a5a5));
    }
            
    ///////////////////////////////////////////////////////////////////////////
    #[bench]
    fn bench_retrieve_non_byte_boundaries(b: &mut test::Bencher)
    {
        let mut bf = BitField::with_capacity(4);        
        assert!(bf.insert_u64(0xa5a5a5a5, 0, 31).is_ok());
        b.iter(|| assert!(bf.retrieve_u64(5, 27).unwrap() == 0x005a5a5a));
    }
            
    ///////////////////////////////////////////////////////////////////////////
    #[bench]
    fn bench_right_shift(b: &mut test::Bencher)
    {
        let mut bf = BitField::with_capacity(4);
        assert!(bf.insert_u64(0xa5a5a5a5, 0, 31).is_ok());
        b.iter(|| assert!((bf >> 7).as_slice() == [0x01, 0x4b, 0x4b, 0x4b]));
    }

    ///////////////////////////////////////////////////////////////////////////
    #[bench]
    fn bench_left_shift(b: &mut test::Bencher)
    {
        let mut bf = BitField::with_capacity(4);
        assert!(bf.insert_u64(0xa5a5a5a5, 0, 31).is_ok());
        b.iter(|| assert!((bf << 7).as_slice() == [0xd2, 0xd2, 0xd2, 0x80]));
    }
}
