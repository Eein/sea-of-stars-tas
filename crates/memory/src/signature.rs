//! Support for finding patterns in a process's memory.

use core::{
    iter,
    mem::{self, MaybeUninit},
    slice,
};

use bytemuck::AnyBitPattern;

use crate::process::Process;

type Offset = u8;

/// A signature that can be used to find a pattern in a process.
///
/// It is recommended to store this in a `static` or `const` variable to ensure that
/// the signature is parsed at compile time, which enables the code to be
/// optimized a lot more.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Signature<const N: usize> {
    /// A simple signature that does not contain any wildcards.
    Simple([u8; N]),
    /// A complex signature that contains wildcards.
    Complex {
        /// The signature itself.
        needle: [u8; N],
        /// The mask that indicates which bytes are wildcards.
        mask: [u8; N],
        /// A lookup table of offsets to jump forward by when certain bytes are encountered.
        skip_offsets: [Offset; 256],
    },
}

struct Parser<'a> {
    bytes: &'a [u8],
}

impl Parser<'_> {
    const fn next(mut self) -> (Option<u8>, Self) {
        while let [b, rem @ ..] = self.bytes {
            self.bytes = rem;
            let b: u8 = *b;
            return (
                Some(match b {
                    b'0'..=b'9' => b - b'0',
                    b'a'..=b'f' => b - b'a' + 0xA,
                    b'A'..=b'F' => b - b'A' + 0xA,
                    b'?' => 0x10,
                    b' ' | b'\r' | b'\n' | b'\t' => continue,
                    _ => panic!("Invalid byte"),
                }),
                self,
            );
        }
        (None, self)
    }
}

const fn contains(mut bytes: &[u8], search_byte: u8) -> bool {
    while let [b, rem @ ..] = bytes {
        bytes = rem;
        if *b == search_byte {
            return true;
        }
    }
    false
}

impl<const N: usize> Signature<N> {
    /// Creates a new signature from a string. The string must be a hexadecimal
    /// string with `?` as wildcard. It is recommended to store this in a
    /// `static` or `const` variable to ensure that the signature is parsed at
    /// compile time, which enables the code to be optimized a lot more.
    ///
    /// # Panics
    ///
    /// This function panics if the signature is invalid. It also panics if the
    /// signature is longer than 255 bytes.
    ///
    /// # Example
    ///
    /// ```
    /// # use asr::signature::Signature;
    /// static SIG: Signature<8> = Signature::new("3A 45 FF ?? ?? B? 00 12");
    /// ```
    pub const fn new(signature: &str) -> Self {
        // We only support u8 offsets atm and thus signatures can't be 256 bytes
        // or longer.
        assert!(N > 0 && N < 256);

        let mut parser = Parser {
            bytes: signature.as_bytes(),
        };

        if contains(signature.as_bytes(), b'?') {
            let mut needle = [0; N];
            let mut mask = [0; N];
            let mut i = 0;

            loop {
                let (a, next) = parser.next();
                parser = next;
                let (b, next) = parser.next();
                parser = next;
                let (Some(a), Some(b)) = (a, b) else { break };
                let sig_byte = (a << 4) | (b & 0x0F);
                let mask_byte = ((a != 0x10) as u8 * 0xF0) | ((b != 0x10) as u8 * 0x0F);
                needle[i] = sig_byte & mask_byte;
                mask[i] = mask_byte;
                i += 1;
            }
            assert!(i == N);

            let mut skip_offsets = [0; 256];

            let mut unknown = 0;
            let end = N - 1;
            let mut i = 0;
            while i < end {
                let byte = needle[i];
                let mask = mask[i];
                if mask == 0xFF {
                    skip_offsets[byte as usize] = (end - i) as Offset;
                } else {
                    unknown = (end - i) as Offset;
                }
                i += 1;
            }

            if unknown == 0 {
                unknown = N as Offset;
            }

            i = 0;
            while i < skip_offsets.len() {
                if unknown < skip_offsets[i] || skip_offsets[i] == 0 {
                    skip_offsets[i] = unknown;
                }
                i += 1;
            }

            Self::Complex {
                needle,
                mask,
                skip_offsets,
            }
        } else {
            let mut needle = [0; N];
            let mut i = 0;

            loop {
                let (a, next) = parser.next();
                parser = next;
                let (b, next) = parser.next();
                parser = next;
                let (Some(a), Some(b)) = (a, b) else { break };
                let sig_byte = (a << 4) | b;
                needle[i] = sig_byte;
                i += 1;
            }
            assert!(i == N);

            Self::Simple(needle)
        }
    }
    /// Performs a signature scan over a provided slice.
    /// Returns an iterator over the positions where the signature matches.
    fn scan_internal<'a>(&'a self, haystack: &'a [u8]) -> impl Iterator<Item = usize> + 'a {
        let mut cursor = 0;
        let end = haystack.len().saturating_sub(N.saturating_sub(1));

        iter::from_fn(move || 'outer: loop {
            if cursor >= end {
                return None;
            }

            match self {
                Signature::Simple(needle) => {
                    match memchr::memmem::find(&haystack[cursor..], needle) {
                        Some(offset) => {
                            let current_cursor = cursor;
                            cursor += offset + 1;
                            return Some(offset + current_cursor);
                        }
                        None => return None,
                    };
                }
                Signature::Complex {
                    needle,
                    mask,
                    skip_offsets,
                } => {
                    let mut i = 0;

                    unsafe {
                        let (scan, mut needle, mut mask) = (
                            haystack.as_ptr().add(cursor),
                            needle.as_ptr(),
                            mask.as_ptr(),
                        );

                        while i + 8 <= N {
                            if scan.add(i).cast::<u64>().read_unaligned()
                                & mask.cast::<u64>().read_unaligned()
                                != needle.cast::<u64>().read_unaligned()
                            {
                                cursor +=
                                    skip_offsets[*scan.add(N.saturating_sub(1)) as usize] as usize;
                                continue 'outer;
                            } else {
                                mask = mask.add(8);
                                needle = needle.add(8);
                                i += 8;
                            }
                        }

                        while i + 4 <= N {
                            if scan.add(i).cast::<u32>().read_unaligned()
                                & mask.cast::<u32>().read_unaligned()
                                != needle.cast::<u32>().read_unaligned()
                            {
                                cursor +=
                                    skip_offsets[*scan.add(N.saturating_sub(1)) as usize] as usize;
                                continue 'outer;
                            } else {
                                mask = mask.add(4);
                                needle = needle.add(4);
                                i += 4;
                            }
                        }

                        while i + 2 <= N {
                            if scan.add(i).cast::<u16>().read_unaligned()
                                & mask.cast::<u16>().read_unaligned()
                                != needle.cast::<u16>().read_unaligned()
                            {
                                cursor +=
                                    skip_offsets[*scan.add(N.saturating_sub(1)) as usize] as usize;
                                continue 'outer;
                            } else {
                                mask = mask.add(2);
                                needle = needle.add(2);
                                i += 2;
                            }
                        }

                        while i < N {
                            if *scan.add(i) & *mask != *needle {
                                cursor +=
                                    skip_offsets[*scan.add(N.saturating_sub(1)) as usize] as usize;
                                continue 'outer;
                            } else {
                                mask = mask.add(1);
                                needle = needle.add(1);
                                i += 1;
                            }
                        }

                        let current_cursor = cursor;
                        cursor += 1;
                        return Some(current_cursor);
                    }
                }
            }
        })
        .fuse()
    }

    // fn scan(&self, haystack: &[u8]) -> Option<usize> {
    //     match self {
    //         Signature::Simple(needle) => memchr::memmem::find(haystack, needle),
    //         Signature::Complex {
    //             needle,
    //             mask,
    //             skip_offsets,
    //         } => {
    //             let mut current = 0;
    //             let end = N - 1;
    //             while let Some(scan) = strip_pod::<[u8; N]>(&mut &haystack[current..]) {
    //                 if matches(scan, needle, mask) {
    //                     return Some(current);
    //                 }
    //                 let offset = skip_offsets[scan[end] as usize];
    //                 current += offset as usize;
    //             }
    //             None
    //         }
    //     }
    // }

    // /// Scans a process for the signature. This will scan the address range of
    // /// the process given. If the signature is found, the address of the start
    // /// of the signature is returned.
    // pub fn scan_process_range(
    //     &self,
    //     process: &Process,
    //     (mut addr, len): (u64, u64),
    // ) -> Option<u64> {
    //     // TODO: Handle the case where a signature may be cut in half by a page
    //     // boundary.
    //     let overall_end = addr + len;
    //     let mut buf = vec![0; 4 << 10];

    //     while addr < overall_end {
    //         // We round up to the 4 KiB address boundary as that's a single
    //         // page, which is safe to read either fully or not at all. We do
    //         // this to do a single read rather than many small ones as the
    //         // syscall overhead is a quite high.
    //         let end = (addr & !((4 << 10) - 1)) + (4 << 10).min(overall_end);
    //         let len = end - addr;
    //         let current_read_buf = &mut buf[..len as usize];
    //         if let Ok(current_read_buf) = process.read_into_uninit_buf(addr, current_read_buf) {
    //             if let Some(pos) = self.scan(current_read_buf) {
    //                 return Some(addr + pos as u64);
    //             }
    //         };
    //         addr = end;
    //     }
    //     None
    // }
}

fn matches<const N: usize>(scan: &[u8; N], needle: &[u8; N], mask: &[u8; N]) -> bool {
    // SAFETY: Before reading individual chunks from the arrays, we check that
    // we can still read values of that size. We also read them unaligned as the
    // original arrays are entirely unaligned.
    unsafe {
        let mut i = 0;
        let (mut scan, mut needle, mut mask) = (scan.as_ptr(), needle.as_ptr(), mask.as_ptr());
        while i + 8 <= N {
            if scan.cast::<u64>().read_unaligned() & mask.cast::<u64>().read_unaligned()
                != needle.cast::<u64>().read_unaligned()
            {
                return false;
            }
            scan = scan.add(8);
            mask = mask.add(8);
            needle = needle.add(8);
            i += 8;
        }
        while i + 4 <= N {
            if scan.cast::<u32>().read_unaligned() & mask.cast::<u32>().read_unaligned()
                != needle.cast::<u32>().read_unaligned()
            {
                return false;
            }
            scan = scan.add(4);
            mask = mask.add(4);
            needle = needle.add(4);
            i += 4;
        }
        while i + 2 <= N {
            if scan.cast::<u16>().read_unaligned() & mask.cast::<u16>().read_unaligned()
                != needle.cast::<u16>().read_unaligned()
            {
                return false;
            }
            scan = scan.add(2);
            mask = mask.add(2);
            needle = needle.add(2);
            i += 2;
        }
        while i < N {
            if *scan & *mask != *needle {
                return false;
            }
            scan = scan.add(1);
            mask = mask.add(1);
            needle = needle.add(1);
            i += 1;
        }
        true
    }
}

fn strip_pod<'a, T: AnyBitPattern>(cursor: &mut &'a [u8]) -> Option<&'a T> {
    if cursor.len() < mem::size_of::<T>() {
        return None;
    }
    let (before, after) = cursor.split_at(mem::size_of::<T>());
    *cursor = after;
    Some(bytemuck::from_bytes(before))
}

/// Trait that provides scanning methods for the `Signature` type.
pub trait SignatureScanner {
    /// Scans a process's memory in the given range for the first occurrence of the signature.
    ///
    /// # Arguments
    ///
    /// * `process` - A reference to the `Process` in which the scan occurs.
    /// * `addr` - The starting address of the memory range.
    /// * `len` - The length of the memory range to scan.
    ///
    /// Returns `Some(Address)` of the first match if found, otherwise `None`.
    fn scan(&self, process: &Process, range: (u64, u64)) -> Option<u64>;

    /// Returns an iterator over all occurrences of the signature in the process's memory range.
    ///
    /// # Arguments
    ///
    /// * `process` - A reference to the `Process` in which the scan occurs.
    /// * `addr` - The starting address of the memory range.
    /// * `len` - The length of the memory range to scan.
    ///
    /// Returns an iterator that yields each matching address.
    fn scan_process_range(&self, process: &Process, range: (u64, u64))
        -> impl Iterator<Item = u64>;
}

impl<const N: usize> SignatureScanner for Signature<N> {
    fn scan(&self, process: &Process, range: (u64, u64)) -> Option<u64> {
        self.scan_process_range(process, range).next()
    }

    fn scan_process_range(
        &self,
        process: &Process,
        range: (u64, u64),
    ) -> impl Iterator<Item = u64> {
        const MEM_SIZE: usize = 0x1000;

        let mut addr: u64 = Into::into(range.0);
        let overall_end = addr + range.1;

        // The sigscan essentially works by reading one memory page (0x1000 bytes)
        // at a time and looking for the signature in each page. We will create a buffer
        // sligthly larger than 0x1000 bytes in order to accomodate the size of
        // the memory page + the signature - 1. The very first bytes of the
        // buffer are intended to be used as the tail of the previous memory page.
        // This allows to scan across the memory page boundaries.

        // We should use N - 1 but we resort to MEM_SIZE - 1 to avoid using [feature(generic_const_exprs)]
        #[repr(packed)]
        struct Buffer<const N: usize> {
            _head: [u8; N],
            _buffer: [u8; MEM_SIZE - 1],
        }

        // The tail of the previous memory page, if read correctly, is stored here
        let mut tail = [0; N];
        let mut last_page_success = false;

        iter::from_fn(move || {
            if addr >= overall_end {
                return None;
            }

            let mut global_buffer = Buffer {
                _head: [0; N],
                _buffer: [0; MEM_SIZE - 1],
            };

            let buf = {
                // SAFETY: The buffer is not initialized, but we are returning a slice of MaybeUninit, which do not require initialization
                unsafe {
                    slice::from_raw_parts_mut(
                        &mut global_buffer as *mut _ as *mut u8,
                        size_of::<Buffer<N>>(),
                    )
                }
            };

            // We round up to the 4 KiB address boundary as that's a single
            // page, which is safe to read either fully or not at all. We do
            // this to reduce the number of syscalls as much as possible, as the
            // syscall overhead is quite high.
            let end = ((addr & !((4 << 10) - 1)) + (4 << 10)).min(overall_end);
            let len = end.saturating_sub(addr) as usize;

            // If we read the previous memory page successfully, then we can copy the last
            // elements to the start of the buffer.
            if last_page_success {
                unsafe {
                    buf.as_mut_ptr().copy_from(tail.as_ptr(), tail.len() - 1);
                }
            }

            let current_page_success = process
                .read_into_uninit_buf(addr, &mut buf[N - 1..][..len])
                .is_ok();

            // We define the final slice on which to perform the memory scan into. If we failed to read the memory page,
            // this returns an empty slice so the subsequent iterator will result into an empty iterator.
            // If we managed to read the current memory page, instead, we check if we have the data from the previous
            // memory page if it got read successfully.
            let scan_buf = unsafe {
                let ptr = if current_page_success {
                    if last_page_success {
                        &buf[..len + N - 1]
                    } else {
                        &buf[N - 1..][..len]
                    }
                } else {
                    &[]
                };

                mem::transmute::<&[u8], &[u8]>(ptr)
            };

            if current_page_success {
                tail[..N - 1].copy_from_slice(&scan_buf[scan_buf.len() - (N - 1)..]);
            }

            let cur_addr = addr;
            let cur_suc = last_page_success;

            addr = end;
            last_page_success = current_page_success;

            Some(self.scan_internal(scan_buf).map(move |pos| {
                let mut address = cur_addr + pos as u64;

                if cur_suc {
                    address = address.checked_add_signed(-(N as i64 - 1)).expect("help")
                }

                address
            }))
        })
        .flatten()
    }
}
