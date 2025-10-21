use crate::property::Property;

// FDT Token constants
const FDT_BEGIN_NODE: u32 = 0x00000001;
const FDT_END_NODE: u32 = 0x00000002;
const FDT_PROP: u32 = 0x00000003;
const FDT_NOP: u32 = 0x00000004;
const FDT_END: u32 = 0x00000009;

// Device Tree Node representation
#[derive(Debug, Clone, Copy)]
pub struct DevTreeNode<'a> {
    name: &'a str,
    dt_struct: &'a [u8],
    dt_strings: &'a [u8],
    struct_offset: usize,
}

impl<'a> DevTreeNode<'a> {
    /// Read a 32-bit token from dt_struct at the given byte offset
    fn read_token(&self, offset: usize) -> Option<u32> {
        Self::read_struct_token(self.dt_struct, offset)
    }

    fn read_struct_token(dt_struct: &'a [u8], offset: usize) -> Option<u32> {
        if offset + 4 > dt_struct.len() {
            return None;
        }
        let bytes = &dt_struct[offset..offset + 4];
        Some(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
    
    /// Read a byte from dt_struct at the given offset
    fn read_byte(&self, offset: usize) -> Option<u8> {
        if offset >= self.dt_struct.len() {
            return None;
        }
        Some(self.dt_struct[offset])
    }
    
    /// Align offset to 4-byte boundary
    fn align_offset(offset: usize) -> usize {
        (offset + 3) & !3
    }
    
    pub fn new_root(dt_struct: &'a [u8], dt_strings: &'a [u8]) -> Option<Self> {
        // The root node starts at offset 0 in the structure block
        Self::new_at_offset(dt_struct, dt_strings, 0)
    }
    
    fn new_at_offset(dt_struct: &'a [u8], dt_strings: &'a [u8], offset: usize) -> Option<Self> {
        // Check FDT_BEGIN_NODE token
        if Self::read_struct_token(dt_struct, offset)? != FDT_BEGIN_NODE {
            return None;
        }
        
        let mut current_offset = offset + 4; // Skip token (4 bytes)
        
        // Read node name
        let name_start = current_offset;
        while current_offset < dt_struct.len() && dt_struct[current_offset] != 0 {
            current_offset += 1;
        }
        current_offset += 1; // Skip null terminator
        
        // // Align to 4-byte boundary
        // current_offset = Self::align_offset(current_offset);
        
        let name = unsafe {
            let name_len = current_offset - name_start - 1;
            core::str::from_utf8_unchecked(&dt_struct[name_start..name_start + name_len])
        };
        
        Some(DevTreeNode {
            name,
            dt_struct,
            dt_strings,
            struct_offset: offset,
        })
    }
    
    fn parse_property(dt_struct: &'a [u8], dt_strings: &'a [u8], offset: usize) -> Option<(Property<'a>, usize)> {
        if offset + 8 > dt_struct.len() {
            return None;
        }
        
        // Read property length and name offset (both 32-bit values)
        let len_bytes = &dt_struct[offset..offset + 4];
        let nameoff_bytes = &dt_struct[offset + 4..offset + 8];
        
        let len = u32::from_be_bytes([len_bytes[0], len_bytes[1], len_bytes[2], len_bytes[3]]) as usize;
        let nameoff = u32::from_be_bytes([nameoff_bytes[0], nameoff_bytes[1], nameoff_bytes[2], nameoff_bytes[3]]) as usize;
        
        if nameoff >= dt_strings.len() {
            return None;
        }
        
        // Find property name
        let name_start = nameoff;
        let mut name_end = nameoff;
        while name_end < dt_strings.len() && dt_strings[name_end] != 0 {
            name_end += 1;
        }
        
        let name = unsafe {
            core::str::from_utf8_unchecked(&dt_strings[name_start..name_end])
        };
        
        // Get property value
        let value_offset = offset + 8; // Skip len and nameoff (8 bytes total)
        let value_end = value_offset + len;
        
        if value_end > dt_struct.len() {
            return None;
        }
        
        let value = &dt_struct[value_offset..value_end];
        
        let next_offset = Self::align_offset(value_end);
        
        Some((Property::new(name, value), next_offset))
    }
    
    fn parse_child_node(dt_struct: &'a [u8], dt_strings: &'a [u8], offset: usize) -> Option<(DevTreeNode<'a>, usize)> {
        let node = Self::new_at_offset(dt_struct, dt_strings, offset)?;
        
        // Find the end of this node by counting BEGIN_NODE/END_NODE pairs
        let mut current_offset = offset + size_of_val(&FDT_BEGIN_NODE) + node.name().len() + 1;
        current_offset = Self::align_offset(current_offset);
        let mut depth = 1;
        
        while let Some(token) = Self::read_struct_token(dt_struct, current_offset) {
            current_offset += size_of_val(&token);
            
            match token {
                FDT_BEGIN_NODE => {
                    depth += 1;
                }
                FDT_END_NODE => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                FDT_PROP => {
                    // Property - skip it
                    if let Some((_, new_offset)) = Self::parse_property(
                        dt_struct, 
                        dt_strings, 
                        current_offset // Skip token
                    ) {
                        current_offset = new_offset;
                    }
                }
                _ => {
                    // Skip other tokens
                }
            }
        }
        
        Some((node, current_offset))
    }
    
    /// Get the node name
    pub fn name(&self) -> &'a str {
        self.name
    }
    
    /// Get all properties of this node
    pub fn properties(&'a self) -> PropertyIterator<'a> {
        PropertyIterator::new(self)
    }
    
    /// Get all child nodes
    pub fn children(&'a self) -> ChildNodeIterator<'a> {
        ChildNodeIterator::new(self)
    }
    
    /// Find a property by name
    pub fn property(&'a self, name: &str) -> Option<Property<'a>> {
        self.properties().find(|prop| prop.name() == name)
    }
    
    /// Find a child node by name
    pub fn child(&'a self, name: &str) -> Option<DevTreeNode<'a>> {
        self.children().find(|child| child.name() == name)
    }
    
    /// Iterate over all descendant nodes (depth-first)
    pub fn iter_descendants(&'a self) -> NodeIterator<'a> {
        NodeIterator::new(self)
    }
}

// Iterator for properties
pub struct PropertyIterator<'a> {
    node: &'a DevTreeNode<'a>,
    current_offset: usize,
}

impl<'a> PropertyIterator<'a> {
    fn new(node: &'a DevTreeNode<'a>) -> Self {
        // Find the start of properties after the node name
        let mut offset = node.struct_offset + 4; // Skip FDT_BEGIN_NODE token
        
        // Skip node name
        while offset < node.dt_struct.len() && node.dt_struct[offset] != 0 {
            offset += 1;
        }
        offset += 1; // Skip null terminator
        offset = DevTreeNode::align_offset(offset); // Align to 4-byte boundary
        
        PropertyIterator {
            node,
            current_offset: offset,
        }
    }
}

impl<'a> Iterator for PropertyIterator<'a> {
    type Item = Property<'a>;
    
    fn next(&mut self) -> Option<Self::Item> {
        // Check for FDT_PROP token
        if self.node.read_token(self.current_offset)? != FDT_PROP {
            return None; // Not a property token
        }
        
        if let Some((prop, new_offset)) = DevTreeNode::parse_property(
            self.node.dt_struct, 
            self.node.dt_strings, 
            self.current_offset + 4 // Skip token
        ) {
            self.current_offset = new_offset;
            Some(prop)
        } else {
            None
        }
    }
}

// Iterator for child nodes
pub struct ChildNodeIterator<'a> {
    node: &'a DevTreeNode<'a>,
    current_offset: usize,
}

impl<'a> ChildNodeIterator<'a> {
    fn new(node: &'a DevTreeNode<'a>) -> Self {
        // Find the start of children after properties
        let mut offset = node.struct_offset + 4; // Skip FDT_BEGIN_NODE token
        
        // Skip node name
        while offset < node.dt_struct.len() && node.dt_struct[offset] != 0 {
            offset += 1;
        }
        offset += 1; // Skip null terminator
        offset = DevTreeNode::align_offset(offset); // Align to 4-byte boundary
        
        // Skip properties
        while let Some(token) = node.read_token(offset) {
            if token == FDT_PROP {
                // Property - skip it
                if let Some((_, new_offset)) = DevTreeNode::parse_property(
                    node.dt_struct, 
                    node.dt_strings, 
                    offset + 4 // Skip token
                ) {
                    offset = new_offset;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        ChildNodeIterator {
            node,
            current_offset: offset,
        }
    }
}

impl<'a> Iterator for ChildNodeIterator<'a> {
    type Item = DevTreeNode<'a>;
    
    fn next(&mut self) -> Option<Self::Item> {
        // Check for FDT_BEGIN_NODE token
        if self.node.read_token(self.current_offset)? != FDT_BEGIN_NODE {
            return None; // Not a begin node token
        }
        
        if let Some((child, new_offset)) = DevTreeNode::parse_child_node(
            self.node.dt_struct,
            self.node.dt_strings,
            self.current_offset
        ) {
            self.current_offset = new_offset;
            Some(child)
        } else {
            None
        }
    }
}

// Iterator for traversing all descendant nodes
pub struct NodeIterator<'a> {
    current: Option<DevTreeNode<'a>>,
    depth: usize,
}

impl<'a> NodeIterator<'a> {
    fn new(root: &'a DevTreeNode<'a>) -> Self {
        NodeIterator {
            current: Some(*root),
            depth: 0,
        }
    }
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = DevTreeNode<'a>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.current {
            // For now, just return the current node
            // A full implementation would need to track the traversal state
            self.current = None;
            Some(node)
        } else {
            None
        }
    }
}
