#![allow(dead_code)]

use kernel_macros::form_tag;

#[repr(C)]
/// Base tag
pub struct BaseTag {
    pub identifier: u64,
    pub next: u64,
}

unsafe impl Sync for BaseTag {}

#[derive(Debug)]
pub enum TagTryFromError {
    NoMatch,
}

/// Header tags
pub mod headers {
    use super::*;

    #[repr(C)]
    #[form_tag(0xc75c9fa92a44c4db)]
    /// Set the preferred video type
    pub struct AnyVideoHeader {
        /// The preference for the video type
        ///
        /// # Values
        /// * `0` - Prefer a linear framebuffer
        /// * `1` - Prefer no linear framebuffer, and CGA Text Mode if available
        pub preference: u64,
    }

    #[repr(C)]
    #[form_tag(0x3ecc1bc43d0f7971)]
    /// Header to specify the Framebuffer.
    /// If all fields are 0, the bootloader will decide.
    pub struct FramebufferHeader {
        /// Width of the Framebuffer
        pub width: u16,
        /// Height of the Framebuffer
        pub height: u16,
        /// Colour mode of the Framebuffer
        pub bpp: u16,
        unused: u16,
    }

    #[repr(C)]
    #[form_tag(0xa85d499b1823be72)]
    pub struct TerminalHeader {
        /// If bit 0 is set, then there is a callback. All other bits are reserved
        pub flags: u64,
        /// The address of the callback function
        pub callback: u64,
    }

    #[repr(C)]
    #[form_tag(0x932f477032007e8f)]
    /// Enable level 5 paging, if it available
    pub struct Level5Paging {}

    #[repr(C)]
    #[form_tag(0xdc29269c2af53d1d)]
    /// This tells the bootloader to add a random slide to the base address of the higher half direct map (HHDM)
    pub struct SlideHHDM {
        /// These should all be 0
        flags: u64,
        /// This value must be non-zero and 2MiB aligned
        pub alignment: u64,
    }

    #[repr(C)]
    #[form_tag(0x92919432b16fe7e7)]
    pub struct UnmapNull {}

    #[repr(C)]
    #[form_tag(0x1ab015085f3273df)]
    pub struct SMP {
        /// # Values (for bit 1)
        ///
        /// * `0` - Use xAPIC
        /// * `1` - Use x2APIC
        ///
        /// All other bits are reserved and must be 0
        pub flags: u64,
    }
}

/// Structure tags
pub mod structures {
    use kernel_macros::form_tag_unsized;

    use super::*;

    #[repr(C)]
    #[form_tag_unsized(0x5df266a64047b6bd)]
    /// Reports the created PMRs from mapping the ELF
    pub struct PMRStructure {
        /// The number of entries in the PMR array
        pub length: u64,
        /// An [PMR] array of `length` size
        pub pmrs: [Pmr],
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct Pmr {
        /// The base address
        pub base: u64,
        /// Length of the region
        pub length: u64,
        /// # Bit positions
        ///
        /// * `0` - Executable
        /// * `1` - Writable
        /// * `3` - Readable
        pub permissions: u64,
    }

    #[repr(C)]
    #[form_tag(0x060d78874a2a8af0)]
    /// Struct with information if "Fully Virtual Kernel Mappings" is enabled
    ///
    /// `PHYSICAL_ADDRESS = PHYSICAL_BASE_ADDRESS + (VIRTUAL_ADDRESS - VIRTUAL_BASE_ADDRESS)`
    pub struct KernelBaseAddressStructure {
        pub phys_base: u64,
        pub virt_base: u64,
    }

    #[repr(C)]
    #[form_tag(0xe5e76a1b4597a781)]
    /// Reports the command line args passed to the bootloader
    pub struct CommandLineStructure {
        /// Pointer to a null-terminated string
        pub cmdline: u64,
    }

    #[repr(C)]
    #[form_tag_unsized(0x2187f79e8612de07)]
    pub struct MemoryMapStructure {
        /// How many entries are in the array
        pub length: u64,
        /// The base address of the entry of type [MMapEntry]
        pub memmap: [MMapEntry],
    }

    #[derive(PartialEq, Eq)]
    #[repr(u32)]
    pub enum MMapEntryKind {
        Usable = 1,
        Reserved = 2,
        ACPIReclaimable = 3,
        ACPINvs = 4,
        BadMemory = 5,
        BootloaderReclaimable = 0x1000,
        KernelAndModules = 0x1001,
        Framebuffer = 0x1002,
    }

    #[repr(C)]
    pub struct MMapEntry {
        pub base: u64,
        pub length: u64,
        pub kind: MMapEntryKind,
        unused: u32,
    }

    impl MMapEntry {
        pub fn end(&self) -> u64 {
            self.base + self.length
        }
    }

    #[repr(C)]
    #[form_tag(0x506461d2950408fa)]
    /// Details on the framebuffer structure
    pub struct FramebufferStructure {
        pub address: u64,

        pub width: u64,
        pub height: u64,

        pub pitch: u64,
        /// Bits per pixel
        pub bpp: u64,

        /// 1 = RGB, else is undefined
        pub memory_model: u8,

        pub red_mask_size: u8,
        pub red_mask_shift: u8,

        pub green_mask_size: u8,
        pub green_mask_shift: u8,

        pub blue_mask_size: u8,
        pub blue_mask_shift: u8,

        unused: u8,
    }

    #[repr(C)]
    #[form_tag(0x38d74c23e0dca893)]
    /// Gives information on the terminal if it's in CGA Text Mode
    pub struct TextModeStructure {
        pub address: u64,
        unused: u16,
        pub rows: u16,
        pub cols: u16,
        pub bytes_per_char: u16,
    }

    #[repr(C)]
    #[form_tag(0x968609d7af96b845)]
    pub struct EDIDStructure {
        pub size: u64,
        pub info: u8,
    }

    #[repr(C)]
    #[form_tag(0xc2b3f4c3233b0974)]
    /// Information on the terminal, if provided by the bootloader
    ///
    /// The terminal callback must have a function prototype of the type `
    pub struct TerminalStructure {
        /// # Bit Values
        /// * `0` - Columns and Rows are provided
        /// * `1` - Max length is provided. Assume 1024 if not.
        /// * `2` - If a callback was requested and the bootloader supports it
        /// * `3` - Context control available
        pub flags: u32,

        pub cols: u16,
        pub rows: u16,

        /// Address to the write function, with the type
        /// `fn(ptr: u64, len: u64)` and the `SysV` ABI
        /// # Context Control
        /// If context control is available, then special values are used as the length, see [TerminalContextControl]
        pub write: u64,

        pub max_len: u64,
    }

    /// Terminal context control values
    #[repr(u64)]
    enum TerminalContextControl {
        /// The pointer must be a u64 which will have the size of the terminal context written to it
        Size = (-1i64) as u64,

        /// The pointer must point to where the terminal will save its context
        Save = (-2i64) as u64,

        /// The pointe rmust point to where the terminal will restore its context
        Restore = (-3i64) as u64,

        /// The pointer is unused
        Refresh = (-4i64) as u64,
    }

    #[repr(u64)]
    pub enum TerminalCallbackTypes {
        /// DEC Private Mode (DECSET/DECRST) sequence
        ///
        /// # Callback Arguments
        /// * `values_count` - Length of array pointed to by values (`u64`)
        /// * `values` - Pointer to the beginning of the array (`*const u32`)
        /// * `final` - Final character in the sequence
        Dec = 10,

        /// BELL character is encountered
        ///
        /// # Arguments
        /// * `unused` x 3
        Bell = 20,

        /// Kernel must respond to a DEC private identification request
        ///
        /// # Arguments
        /// * `unused` x 3
        PrivateId = 30,

        /// Kernel must respond to to a ECMA-48 status report request
        ///
        /// # Arguments
        /// * `unused` x 3
        StatusReport = 40,

        /// Kernel must respond to a ECMA-48 cursor position request
        ///
        /// # Arguments
        /// * `x` - The cursor row
        /// * `y` - The cursor column
        /// * `unused`
        PositionReport = 50,

        /// Kernel must respond to a Keyboard LED state change request
        ///
        /// # Arguments
        /// * `led_state` - Can be valued from `0..=3`,
        /// with the meanings "Clear all LEDs", "Set Scroll Lock", "Set Num Lock", and "Set Caps Lock" LED respectively
        KeyboardLED = 60,

        /// The terminal encountered an ECMA-48 mode switch it cannot handle on its own
        ///
        /// # Arguments
        /// * `values_count` - Length of `values`
        /// * `values` - Array of length `values_count`, type `*const u32`
        /// * `final` - The final character in the escape sequence
        TerminalMode = 70,

        /// The terminal encountered a private Linux escape sequence that it cannot handle on its own
        ///
        /// # Arguments
        /// * `values_count` - Length of `values`
        /// * `values` - Array of length `values_count`, type `*const u32`
        /// * `unused`
        LinuxEscape = 80,
    }

    /// Contains the modules loaded by the bootloader
    #[repr(C)]
    #[form_tag_unsized(0x4b6fe466aade04ce)]
    pub struct ModuleStructure {
        pub count: u64,
        pub pmrs: [Module],
    }

    #[derive(Debug)]
    /// Data on a module
    pub struct Module {
        /// The beginning of the module
        pub begin: u64,

        /// The ending of the module
        pub end: u64,

        /// A null-terminated string
        pub string: *const u8,
    }

    /// Structure with the pointer to the RSDP structure
    #[repr(C)]
    #[form_tag(0x9e1786930a375e78)]
    pub struct RSDP {
        /// Pointer to the RSDP structure
        pub rsdp: u64,
    }

    /// Structure for the SMBIOS info
    #[repr(C)]
    #[form_tag(0x274bd246c62bf7d1)]
    pub struct SMBIOS {
        /// Unused, must be 0
        flags: u64,
        /// The 32 bit SMBIOS entry, 0 if unavailable
        pub smbios_32: u64,
        /// The 64 bit SMBIOS entry, 0 if unavailable
        pub smbios_64: u64,
    }

    /// Struct with the linux epoch
    #[repr(C)]
    #[form_tag(0x566a7bed888e1407)]
    pub struct Epoch {
        /// The linux epoch
        pub epoch: u64,
    }

    /// Info on the firmware
    #[repr(C)]
    #[form_tag(0x359d837855e3858c)]
    pub struct FirmwareStructure {
        /// If bit 0, UEFI, if 1, BIOS
        pub flags: u64,
    }

    /// Address for the UEFI system table
    #[repr(C)]
    #[form_tag(0x4bc5ec15845b558e)]
    pub struct EFISystemTable {
        /// The address to the system table
        pub system_table: u64,
    }

    /// The raw kernel file
    #[repr(C)]
    #[form_tag(0xe599d90c2975584a)]
    pub struct KernelFileStructure {
        /// Address of the raw kernel file
        pub kernel_file: u64,
    }

    /// The raw kernel file
    #[repr(C)]
    #[form_tag(0x37c13018a02c6ea2)]
    pub struct KernelFileStructureV2 {
        /// Address of the raw kernel file
        pub kernel_file: u64,

        /// Size of the raw kernel file
        pub kernel_size: u64,
    }

    /// Information on the boot volume
    #[repr(C)]
    #[form_tag(0x9b4358364c19ee62)]
    pub struct BootVolumeStructure {
        /// If 0, GUID is valid, if 1, the Partition GUID is valid
        pub flags: u64,

        /// The GUID
        pub guid: Guid,

        /// The Partition GUID
        pub part_guid: Guid,
    }

    /// A globally unique identifier
    #[repr(packed)]
    #[derive(Default)]
    pub struct Guid {
        pub a: u32,
        pub b: u16,
        pub c: u16,
        pub d: [u8; 8],
    }

    /// Defines the kernel's load offset
    #[repr(C)]
    #[form_tag(0xee80847d01506c57)]
    pub struct KernelSlideStructure {
        /// Positive offset of the kernel
        pub slide: u64,
    }

    /// Information on the multiprocessor environment
    #[repr(C)]
    #[form_tag_unsized(0xee80847d01506c57)]
    pub struct SMPStructure {
        /// Bit 0 is set if x2APIC was requested and it was supported and enabled
        pub flags: u64,

        /// LAPIC ID of the bootstrap processor
        pub lapic_id: u32,

        /// Reserved
        reserved: u32,

        /// The number of CPUs
        pub cpu_count: u64,

        /// The array of CPU info
        pub smp_info: [SMPInfo],
    }

    #[repr(C, packed)]
    #[derive(Debug, Clone, Copy)]
    pub struct SMPInfo {
        /// ACPI processor UID
        pub processor_id: u32,

        /// LAPIC ID
        pub lapic_id: u32,

        /// The stack area to load, it must be at least 256 bytes and 16 byte aligned
        pub target_stack: u64,

        /// The address to jump to after an atomic write
        pub goto_address: u64,

        /// Any extra arguments the Kernel wants to pass on
        pub args: u64,
    }

    /// Info on the PXE server booted from
    #[repr(C, packed)]
    #[form_tag(0x29d1e96239247032)]
    pub struct PXEServerInfoStructure {
        /// The IP in Big-Endian
        pub server_ip: u32,
    }

    /// Info on the memory-mapped UART
    #[repr(C)]
    #[form_tag(0x29d1e96239247032)]
    pub struct MMIOUart {
        /// The address
        pub addr: u64,
    }

    /// Describes the Device Tree Blob for the platform
    #[repr(C)]
    #[form_tag(0xabb29bd49a2833fa)]
    pub struct DeviceTreeBlobStructure {
        /// The address of the DTB
        pub address: u64,

        /// The size of the DTB
        pub size: u64,
    }

    /// Describes the HHDM
    #[repr(C)]
    #[form_tag(0xb0ed257db18cb58f)]
    pub struct HigherHalfDirectMapStructure {
        /// Beginning of the HHDM (Virtual address)
        pub addr: u64,
    }
}
