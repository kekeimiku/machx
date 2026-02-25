//! This module roughly corresponds to `mach-o/loader.h`.
use core::{
    ffi::{c_char, c_int},
    mem::transmute,
};

pub type cpu_type_t = c_int;
pub type cpu_subtype_t = c_int;

pub const LC_SEGMENT_64: u32 = 25;
pub const SEG_TEXT: &[c_char; 7usize] = unsafe { transmute(b"__TEXT\0") };

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Copy, Clone, Debug)]
pub struct mach_header {
    pub magic: u32,
    pub cputype: cpu_type_t,
    pub cpusubtype: cpu_subtype_t,
    pub filetype: u32,
    pub ncmds: u32,
    pub sizeofcmds: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct mach_header_64 {
    pub magic: u32,
    pub cputype: i32,
    pub cpusubtype: i32,
    pub filetype: u32,
    pub ncmds: u32,
    pub sizeofcmds: u32,
    pub flags: u32,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct segment_command {
    pub cmd: u32,
    pub cmdsize: u32,
    pub segname: [c_char; 16usize],
    pub vmaddr: u32,
    pub vmsize: u32,
    pub fileoff: u32,
    pub filesize: u32,
    pub maxprot: i32,
    pub initprot: i32,
    pub nsects: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct segment_command_64 {
    pub cmd: u32,
    pub cmdsize: u32,
    pub segname: [c_char; 16usize],
    pub vmaddr: u64,
    pub vmsize: u64,
    pub fileoff: u64,
    pub filesize: u64,
    pub maxprot: i32,
    pub initprot: i32,
    pub nsects: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct section {
    pub sectname: [c_char; 16usize],
    pub segname: [c_char; 16usize],
    pub addr: u32,
    pub size: u32,
    pub offset: u32,
    pub align: u32,
    pub reloff: u32,
    pub nreloc: u32,
    pub flags: u32,
    pub reserved1: u32,
    pub reserved2: u32,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct section_64 {
    pub sectname: [c_char; 16usize],
    pub segname: [c_char; 16usize],
    pub addr: u64,
    pub size: u64,
    pub offset: u32,
    pub align: u32,
    pub reloff: u32,
    pub nreloc: u32,
    pub flags: u32,
    pub reserved1: u32,
    pub reserved2: u32,
    pub reserved3: u32,
}
