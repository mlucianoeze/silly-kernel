use crate::node::DevTreeNode;

// Device Tree Blob header structure
#[repr(C)]
#[derive(Debug)]
pub struct DevTreeHeader {
    magic: u32,               // 0x00: Magic number (0xd00dfeed)
    totalsize: u32,           // 0x04: Total size of the DTB
    off_dt_struct: u32,       // 0x08: Offset to structure block
    off_dt_strings: u32,      // 0x0C: Offset to strings block
    off_mem_rsvmap: u32,      // 0x10: Offset to memory reservation block
    version: u32,             // 0x14: Version
    last_comp_version: u32,   // 0x18: Last compatible version
    boot_cpuid_phys: u32,     // 0x1C: Boot CPU physical ID
    size_dt_strings: u32,     // 0x20: Size of strings block
    size_dt_struct: u32,      // 0x24: Size of structure block
}

impl DevTreeHeader {
    #[inline(always)]
    pub fn magic(&self) -> u32 {
        u32::from_be(self.magic)
    }

    #[inline(always)]
    pub fn totalsize(&self) -> u32 {
        u32::from_be(self.totalsize)
    }
    
    #[inline(always)]
    pub fn off_dt_struct(&self) -> u32 {
        u32::from_be(self.off_dt_struct)
    }
    
    #[inline(always)]
    pub fn off_dt_strings(&self) -> u32 {
        u32::from_be(self.off_dt_strings)
    }
    
    #[inline(always)]
    pub fn off_mem_rsvmap(&self) -> u32 {
        u32::from_be(self.off_mem_rsvmap)
    }

    #[inline(always)]
    pub fn version(&self) -> u32 {
        u32::from_be(self.version)
    }

    #[inline(always)]
    pub fn last_comp_version(&self) -> u32 {
        u32::from_be(self.last_comp_version)
    }

    #[inline(always)]
    pub fn boot_cpuid_phys(&self) -> u32 {
        u32::from_be(self.boot_cpuid_phys)
    }

    #[inline(always)]
    pub fn size_dt_strings(&self) -> u32 {
        u32::from_be(self.size_dt_strings)
    }

    #[inline(always)]
    pub fn size_dt_struct(&self) -> u32 {
        u32::from_be(self.size_dt_struct)
    }
}

// Zero-copy Device Tree representation
#[derive(Debug)]
pub struct DevTree {
    header: &'static DevTreeHeader,
    // dt_struct: &'static [u32],
    // dt_strings: &'static [u8],
    mem_rsvmap: &'static [u64],
    root_node: DevTreeNode<'static>,
}

impl DevTree {
    pub fn new(fdt_ptr: *const u8) -> Option<Self> {
        unsafe {
            // Cast pointer to FDT header
            let header = &*(fdt_ptr as *const DevTreeHeader);
            
            // Validate magic number
            if header.magic() != 0xd00dfeed {
                return None;
            }
            
            // Validate total size
            if header.totalsize() == 0 {
                return None;
            }
            
            let off_dt_struct = header.off_dt_struct() as usize;
            let size_dt_struct = header.size_dt_struct() as usize;
            
            // Create slices for different sections
            let dt_struct: &'static [u8] = core::slice::from_raw_parts(
                fdt_ptr.add(off_dt_struct),
                size_dt_struct,
            );
            
            let size_dt_strings = header.size_dt_strings() as usize;
            let off_dt_strings = header.off_dt_strings() as usize;

            let dt_strings: &'static [u8] = core::slice::from_raw_parts(
                fdt_ptr.add(off_dt_strings),
                size_dt_strings,
            );
            
            let off_mem_rsvmap = header.off_mem_rsvmap() as usize;
            let size_mem_rsvmap = Self::count_mem_rsv_entries(fdt_ptr.add(off_mem_rsvmap) as *const u64);

            let mem_rsvmap: &'static [u64] = core::slice::from_raw_parts(
                fdt_ptr.add(off_mem_rsvmap) as *const u64,
                size_mem_rsvmap,
            );
            
            // Create root node
            let root_node = DevTreeNode::new_root(dt_struct, dt_strings)?;
            
            Some(DevTree {
                header,
                // dt_struct,
                // dt_strings,
                mem_rsvmap,
                root_node,
            })
        }
    }
    
    fn count_mem_rsv_entries(rsv_ptr: *const u64) -> usize {
        unsafe {
            let mut count = 0;
            let mut ptr = rsv_ptr as *const u64;
            
            loop {
                let addr = *ptr;
                let size = *ptr.add(1);
                
                if addr == 0 && size == 0 {
                    break;
                }
                
                count += 1;
                ptr = ptr.add(2);
            }
            
            count
        }
    }
    
    /// Get the root node of the device tree
    pub fn root(&self) -> &DevTreeNode<'_> {
        &self.root_node
    }
    
    /// Get the header information
    pub fn header(&self) -> &DevTreeHeader {
        self.header
    }
    
    /// Get memory reservation map
    pub fn mem_rsvmap(&self) -> &[u64] {
        self.mem_rsvmap
    }
}
