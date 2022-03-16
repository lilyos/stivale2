use super::tags::BaseTag;

#[derive(Debug)]
#[repr(C)]
pub struct Stivale2HeaderKernelToBootloader {
    pub entry_point: u64,
    pub stack: *const u8,
    pub flags: Stivale2HeaderFlags,
    pub tags: *const (),
}

unsafe impl Sync for Stivale2HeaderKernelToBootloader {}

#[derive(Debug)]
#[repr(C)]
pub struct Stivale2HeaderBootloaderToKernel {
    pub brand: [u8; 64],
    pub version: [u8; 64],
    pub tags: *const BaseTag,
}

#[derive(Debug)]
pub struct Stivale2HeaderFlagsBuilder {
    /// Upgrade Higher Half
    uhh: bool,
    /// Protected Memory Regions
    pmr: bool,
    /// Virtual Kernel Mappings
    vkm: bool,
}

impl Stivale2HeaderFlagsBuilder {
    pub const fn new() -> Self {
        Self {
            uhh: false,
            pmr: false,
            vkm: false,
        }
    }

    pub const fn upgrade_higher_half(mut self, val: bool) -> Self {
        self.uhh = val;
        self
    }

    pub const fn protected_memory_regions(mut self, val: bool) -> Self {
        self.pmr = val;
        self
    }

    pub const fn virtual_kernel_mappings(mut self, val: bool) -> Self {
        self.vkm = val;
        self
    }

    pub const fn finish(self) -> Stivale2HeaderFlags {
        let mut tmp = 0;
        if self.uhh {
            tmp |= Stivale2HeaderFlags::UPGRADE_HIGHER_HALF;
        }
        if self.pmr {
            tmp |= Stivale2HeaderFlags::PROTECTED_MEMORY_REGIONS;
        }
        if self.vkm {
            tmp |= Stivale2HeaderFlags::VIRTUAL_KERNEL_MAPPINGS;
        }
        Stivale2HeaderFlags(tmp)
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Stivale2HeaderFlags(pub u64);

impl Stivale2HeaderFlags {
    pub const UPGRADE_HIGHER_HALF: u64 = 1 << 1;

    pub fn new(
        upgrade_higher_half: bool,
        protected_memory_regions: bool,
        virtual_kernel_mappings: bool,
    ) -> Self {
        let mut tmp = Stivale2HeaderFlags(0);
        tmp.set_allow_low_memory_boot();
        if upgrade_higher_half {
            tmp.set_upgrade_higher_half();
        }
        if protected_memory_regions {
            tmp.set_protected_memory_regions();
        }
        if virtual_kernel_mappings {
            tmp.set_virtual_kernel_mappings();
        }
        tmp
    }

    pub fn get_upgrade_higher_half(&self) -> bool {
        self.0 & Self::UPGRADE_HIGHER_HALF != 0
    }

    pub fn set_upgrade_higher_half(&mut self) {
        self.0 |= Self::UPGRADE_HIGHER_HALF
    }

    pub fn clear_upgrade_higher_half(&mut self) {
        self.0 &= !Self::UPGRADE_HIGHER_HALF
    }

    pub const PROTECTED_MEMORY_REGIONS: u64 = 1 << 2;

    pub fn get_protected_memory_regions(&self) -> bool {
        self.0 & Self::PROTECTED_MEMORY_REGIONS != 0
    }

    pub fn set_protected_memory_regions(&mut self) {
        self.0 |= Self::PROTECTED_MEMORY_REGIONS
    }

    pub fn clear_protected_memory_regions(&mut self) {
        self.0 &= !Self::PROTECTED_MEMORY_REGIONS
    }

    pub const VIRTUAL_KERNEL_MAPPINGS: u64 = 1 << 3;

    pub fn get_virtual_kernel_mappings(&self) -> bool {
        self.0 & Self::VIRTUAL_KERNEL_MAPPINGS != 0
    }

    pub fn set_virtual_kernel_mappings(&mut self) {
        self.0 |= Self::VIRTUAL_KERNEL_MAPPINGS
    }

    pub fn clear_virtual_kernel_mappings(&mut self) {
        self.0 &= !Self::VIRTUAL_KERNEL_MAPPINGS
    }

    pub const ALLOW_LOW_MEMORY_BOOT: u64 = 1 << 4;

    pub fn get_allow_low_memory_boot(&self) -> bool {
        self.0 & Self::ALLOW_LOW_MEMORY_BOOT != 0
    }

    pub fn set_allow_low_memory_boot(&mut self) {
        self.0 |= Self::ALLOW_LOW_MEMORY_BOOT
    }

    pub fn clear_allow_low_memory_boot(&mut self) {
        self.0 &= !Self::ALLOW_LOW_MEMORY_BOOT
    }
}
