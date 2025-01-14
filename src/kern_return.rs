//! This module corresponds to `mach/kern_return.h`.

// ...Except for this particular type, which is taken from
// `mach/i386/kern_return.h` and `mach/arm/kern_return.h` (also used for
// aarch64): it is the same type in both header files.
pub type kern_return_t = ::libc::c_int;

pub const KERN_SUCCESS: kern_return_t = 0;
pub const KERN_INVALID_ADDRESS: kern_return_t = 1;
pub const KERN_PROTECTION_FAILURE: kern_return_t = 2;
pub const KERN_NO_SPACE: kern_return_t = 3;
pub const KERN_INVALID_ARGUMENT: kern_return_t = 4;
pub const KERN_FAILURE: kern_return_t = 5;
pub const KERN_RESOURCE_SHORTAGE: kern_return_t = 6;
pub const KERN_NOT_RECEIVER: kern_return_t = 7;
pub const KERN_NO_ACCESS: kern_return_t = 8;
pub const KERN_MEMORY_FAILURE: kern_return_t = 9;
pub const KERN_MEMORY_ERROR: kern_return_t = 10;
pub const KERN_ALREADY_IN_SET: kern_return_t = 11;
pub const KERN_NOT_IN_SET: kern_return_t = 12;
pub const KERN_NAME_EXISTS: kern_return_t = 13;
pub const KERN_ABORTED: kern_return_t = 14;
pub const KERN_INVALID_NAME: kern_return_t = 15;
pub const KERN_INVALID_TASK: kern_return_t = 16;
pub const KERN_INVALID_RIGHT: kern_return_t = 17;
pub const KERN_INVALID_VALUE: kern_return_t = 18;
pub const KERN_UREFS_OVERFLOW: kern_return_t = 19;
pub const KERN_INVALID_CAPABILITY: kern_return_t = 20;
pub const KERN_RIGHT_EXISTS: kern_return_t = 21;
pub const KERN_INVALID_HOST: kern_return_t = 22;
pub const KERN_MEMORY_PRESENT: kern_return_t = 23;
pub const KERN_MEMORY_DATA_MOVED: kern_return_t = 24;
pub const KERN_MEMORY_RESTART_COPY: kern_return_t = 25;
pub const KERN_INVALID_PROCESSOR_SET: kern_return_t = 26;
pub const KERN_POLICY_LIMIT: kern_return_t = 27;
pub const KERN_INVALID_POLICY: kern_return_t = 28;
pub const KERN_INVALID_OBJECT: kern_return_t = 29;
pub const KERN_ALREADY_WAITING: kern_return_t = 30;
pub const KERN_DEFAULT_SET: kern_return_t = 31;
pub const KERN_EXCEPTION_PROTECTED: kern_return_t = 32;
pub const KERN_INVALID_LEDGER: kern_return_t = 33;
pub const KERN_INVALID_MEMORY_CONTROL: kern_return_t = 34;
pub const KERN_INVALID_SECURITY: kern_return_t = 35;
pub const KERN_NOT_DEPRESSED: kern_return_t = 36;
pub const KERN_TERMINATED: kern_return_t = 37;
pub const KERN_LOCK_SET_DESTROYED: kern_return_t = 38;
pub const KERN_LOCK_UNSTABLE: kern_return_t = 39;
pub const KERN_LOCK_OWNED: kern_return_t = 40;
pub const KERN_LOCK_OWNED_SELF: kern_return_t = 41;
pub const KERN_SEMAPHORE_DESTROYED: kern_return_t = 42;
pub const KERN_RPC_SERVER_TERMINATED: kern_return_t = 43;
pub const KERN_RPC_TERMINATE_ORPHAN: kern_return_t = 44;
pub const KERN_RPC_CONTINUE_ORPHAN: kern_return_t = 45;
pub const KERN_NOT_SUPPORTED: kern_return_t = 46;
pub const KERN_NODE_DOWN: kern_return_t = 47;
pub const KERN_NOT_WAITING: kern_return_t = 48;
pub const KERN_OPERATION_TIMED_OUT: kern_return_t = 49;
pub const KERN_CODESIGN_ERROR: kern_return_t = 50;
pub const KERN_POLICY_STATIC: kern_return_t = 51;
pub const KERN_RETURN_MAX: kern_return_t = 0x100;
