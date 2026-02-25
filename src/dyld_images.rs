//! This module roughly corresponds to `mach-o/dyld_images.h`.

#![allow(non_snake_case)]

use core::ffi::{c_char, c_void};

use super::{loader::mach_header, port::mach_port_t};

pub const DYLD_AOT_IMAGE_KEY_SIZE: u32 = 32;
pub const DYLD_MAX_PROCESS_INFO_NOTIFY_COUNT: u32 = 8;

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum dyld_image_mode {
    dyld_image_adding = 0,
    dyld_image_removing = 1,
    dyld_image_info_change = 2,
    dyld_image_dyld_moved = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dyld_image_info {
    pub imageLoadAddress: *mut mach_header,
    pub imageFilePath: *const c_char,
    pub imageFileModDate: usize,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct dyld_uuid_info {
    pub imageLoadAddress: *const mach_header,
    pub imageUUID: [u8; 16usize],
}

#[derive(Debug, Copy, Clone)]
pub struct dyld_aot_image_info {
    pub x86LoadAddress: *const mach_header,
    pub aotLoadAddress: *const mach_header,
    pub aotImageSize: u64,
    pub aotImageKey: [u8; 32usize],
}

#[derive(Debug, Copy, Clone)]
pub struct dyld_aot_shared_cache_info {
    pub cacheBaseAddress: usize,
    pub cacheUUID: [u8; 16usize],
}

pub type dyld_image_notifier = ::core::option::Option<
    unsafe extern "C" fn(mode: dyld_image_mode, infoCount: u32, info: *const dyld_image_info),
>;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
#[repr(align(16))]
pub struct dyld_all_image_infos {
    pub version: u32,
    pub infoArrayCount: u32,
    pub infoArray: *const dyld_image_info,
    pub notification: dyld_image_notifier,
    pub processDetachedFromSharedRegion: bool,
    pub libSystemInitialized: bool,
    pub dyldImageLoadAddress: *const mach_header,
    pub jitInfo: *mut c_void,
    pub dyldVersion: *const c_char,
    pub errorMessage: *const c_char,
    pub terminationFlags: usize,
    pub coreSymbolicationShmPage: *mut c_void,
    pub systemOrderFlag: usize,
    pub uuidArrayCount: usize,
    pub uuidArray: *const dyld_uuid_info,
    pub dyldAllImageInfosAddress: *mut dyld_all_image_infos,
    pub initialImageCount: usize,
    pub errorKind: usize,
    pub errorClientOfDylibPath: *const c_char,
    pub errorTargetDylibPath: *const c_char,
    pub errorSymbol: *const c_char,
    pub sharedCacheSlide: usize,
    pub sharedCacheUUID: [u8; 16usize],
    pub sharedCacheBaseAddress: usize,
    pub infoArrayChangeTimestamp: u64,
    pub dyldPath: *const c_char,
    pub notifyPorts: [mach_port_t; 8usize],
    pub reserved: [usize; 7usize],
    pub sharedCacheFSID: u64,
    pub sharedCacheFSObjID: u64,
    pub compact_dyld_image_info_addr: usize,
    pub compact_dyld_image_info_size: usize,
    pub platform: u32,
    pub aotInfoCount: u32,
    pub aotInfoArray: *const dyld_aot_image_info,
    pub aotInfoArrayChangeTimestamp: u64,
    pub aotSharedCacheBaseAddress: usize,
    pub aotSharedCacheUUID: [u8; 16usize],
}
