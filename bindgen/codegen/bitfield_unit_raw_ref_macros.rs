#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}

impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}

impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    fn extract_bit(byte: u8, index: usize) -> bool {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };

        let mask = 1 << bit_index;

        byte & mask == mask
    }

    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());

        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];

        Self::extract_bit(byte, index)
    }

    #[inline]
    pub unsafe fn raw_get_bit(this: *const Self, index: usize) -> bool {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());

        let byte_index = index / 8;
        let byte = unsafe { *(core::ptr::addr_of!((*this).storage) as *const u8)
            .offset(byte_index as isize) };

        Self::extract_bit(byte, index)
    }

    #[inline]
    fn change_bit(byte: u8, index: usize, val: bool) -> u8 {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };

        let mask = 1 << bit_index;
        if val {
            byte | mask
        } else {
            byte & !mask
        }
    }

    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());

        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];

        *byte = Self::change_bit(*byte, index, val);
    }

    #[inline]
    pub unsafe fn raw_set_bit(this: *mut Self, index: usize, val: bool) {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());

        let byte_index = index / 8;
        let byte = unsafe {
            (core::ptr::addr_of_mut!((*this).storage) as *mut u8)
                .offset(byte_index as isize)
        };

        unsafe { *byte = Self::change_bit(*byte, index, val) };
    }

    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <=
                self.storage.as_ref().len()
        );

        let mut val = 0;

        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }

        val
    }

    #[inline]
    pub unsafe fn raw_get(
        this: *const Self,
        bit_offset: usize,
        bit_width: u8,
    ) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <=
                core::mem::size_of::<Storage>()
        );

        let mut val = 0;

        for i in 0..(bit_width as usize) {
            if unsafe { Self::raw_get_bit(this, i + bit_offset) } {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }

        val
    }

    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <=
                self.storage.as_ref().len()
        );

        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }

    #[inline]
    pub unsafe fn raw_set(
        this: *mut Self,
        bit_offset: usize,
        bit_width: u8,
        val: u64,
    ) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <=
                core::mem::size_of::<Storage>()
        );

        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            unsafe { Self::raw_set_bit(this, index + bit_offset, val_bit_is_set) };
        }
    }
}

/// Const-generic methods for efficient bitfield access when offset and width
/// are known at compile time.
impl<const N: usize> __BindgenBitfieldUnit<[u8; N]> {
    /// Get a field using const generics for compile-time optimization.
    /// Uses native word size operations when the field fits in usize.
    #[inline]
    pub const fn get_const<const BIT_OFFSET: usize, const BIT_WIDTH: u8>(
        &self,
    ) -> u64 {
        debug_assert!(BIT_WIDTH <= 64);
        debug_assert!(BIT_OFFSET / 8 < N);
        debug_assert!((BIT_OFFSET + (BIT_WIDTH as usize)) / 8 <= N);

        if BIT_WIDTH == 0 {
            return 0;
        }

        let start_byte = BIT_OFFSET / 8;
        let bit_shift = BIT_OFFSET % 8;
        let bytes_needed = (BIT_WIDTH as usize + bit_shift + 7) / 8;

        // Use usize for fields that fit, u64 only when necessary.
        // The compiler eliminates the unused branch since BIT_WIDTH is const.
        if BIT_WIDTH as usize + bit_shift <= usize::BITS as usize {
            let mut val = 0usize;

            if cfg!(target_endian = "big") {
                let mut i = 0;
                while i < bytes_needed {
                    val |= (self.storage[start_byte + i].reverse_bits()
                        as usize) <<
                        (i * 8);
                    i += 1;
                }
            } else {
                let mut i = 0;
                while i < bytes_needed {
                    val |= (self.storage[start_byte + i] as usize) << (i * 8);
                    i += 1;
                }
            }

            val >>= bit_shift;
            val &= (1usize << BIT_WIDTH) - 1;

            if cfg!(target_endian = "big") {
                val = val.reverse_bits() >>
                    (usize::BITS as usize - BIT_WIDTH as usize);
            }

            val as u64
        } else {
            let mut val = 0u64;

            if cfg!(target_endian = "big") {
                let mut i = 0;
                while i < bytes_needed {
                    val |= (self.storage[start_byte + i].reverse_bits() as u64) <<
                        (i * 8);
                    i += 1;
                }
            } else {
                let mut i = 0;
                while i < bytes_needed {
                    val |= (self.storage[start_byte + i] as u64) << (i * 8);
                    i += 1;
                }
            }

            val >>= bit_shift;

            if BIT_WIDTH < 64 {
                val &= (1u64 << BIT_WIDTH) - 1;
            }

            if cfg!(target_endian = "big") {
                val = val.reverse_bits() >> (64 - BIT_WIDTH as usize);
            }

            val
        }
    }

    /// Set a field using const generics for compile-time optimization.
    /// Uses native word size operations when the field fits in usize.
    #[inline]
    pub fn set_const<const BIT_OFFSET: usize, const BIT_WIDTH: u8>(
        &mut self,
        val: u64,
    ) {
        debug_assert!(BIT_WIDTH <= 64);
        debug_assert!(BIT_OFFSET / 8 < N);
        debug_assert!((BIT_OFFSET + (BIT_WIDTH as usize)) / 8 <= N);

        if BIT_WIDTH == 0 {
            return;
        }

        let start_byte = BIT_OFFSET / 8;
        let bit_shift = BIT_OFFSET % 8;
        let bytes_needed = (BIT_WIDTH as usize + bit_shift + 7) / 8;

        // Use usize for fields that fit, u64 only when necessary.
        // The compiler eliminates the unused branch since BIT_WIDTH is const.
        if BIT_WIDTH as usize + bit_shift <= usize::BITS as usize {
            let mut val = val as usize;
            val &= (1usize << BIT_WIDTH) - 1;

            if cfg!(target_endian = "big") {
                val = val.reverse_bits() >>
                    (usize::BITS as usize - BIT_WIDTH as usize);
            }

            val <<= bit_shift;

            let field_mask = ((1usize << BIT_WIDTH) - 1) << bit_shift;

            let mut i = 0;
            while i < bytes_needed {
                let byte_val = (val >> (i * 8)) as u8;
                let byte_mask = (field_mask >> (i * 8)) as u8;

                if cfg!(target_endian = "big") {
                    let byte = self.storage[start_byte + i].reverse_bits();
                    let new_byte = (byte & !byte_mask) | (byte_val & byte_mask);
                    self.storage[start_byte + i] = new_byte.reverse_bits();
                } else {
                    self.storage[start_byte + i] =
                        (self.storage[start_byte + i] & !byte_mask) |
                            (byte_val & byte_mask);
                }
                i += 1;
            }
        } else {
            let mut val = val;

            if BIT_WIDTH < 64 {
                val &= (1u64 << BIT_WIDTH) - 1;
            }

            if cfg!(target_endian = "big") {
                val = val.reverse_bits() >> (64 - BIT_WIDTH as usize);
            }

            val <<= bit_shift;

            let field_mask = if BIT_WIDTH as usize + bit_shift >= 64 {
                !0u64 << bit_shift
            } else {
                ((1u64 << BIT_WIDTH) - 1) << bit_shift
            };

            let mut i = 0;
            while i < bytes_needed {
                let byte_val = (val >> (i * 8)) as u8;
                let byte_mask = (field_mask >> (i * 8)) as u8;

                if cfg!(target_endian = "big") {
                    let byte = self.storage[start_byte + i].reverse_bits();
                    let new_byte = (byte & !byte_mask) | (byte_val & byte_mask);
                    self.storage[start_byte + i] = new_byte.reverse_bits();
                } else {
                    self.storage[start_byte + i] =
                        (self.storage[start_byte + i] & !byte_mask) |
                            (byte_val & byte_mask);
                }
                i += 1;
            }
        }
    }
}
