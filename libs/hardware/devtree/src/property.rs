// Device Tree Property representation
#[derive(Debug, Clone, Copy)]
pub struct Property<'a> {
    name: &'a str,
    value: &'a [u8],
}

impl<'a> Property<'a> {
    pub fn new(name: &'a str, value: &'a [u8]) -> Self {
        Property { name, value }
    }
    
    /// Get the property name
    pub fn name(&self) -> &'a str {
        self.name
    }
    
    /// Get the property value as bytes
    pub fn value(&self) -> &'a [u8] {
        self.value
    }
    
    /// Get the property value as a string (if it's null-terminated)
    pub fn as_string(&self) -> Option<&'a str> {
        if self.value.is_empty() {
            return None;
        }
        
        // Check if the last byte is null
        if self.value.last() == Some(&0) {
            core::str::from_utf8(&self.value[..self.value.len() - 1]).ok()
        } else {
            None
        }
    }
    
    /// Get the property value as a u32 (if it's 4 bytes)
    pub fn as_u32(&self) -> Option<u32> {
        if self.value.len() == 4 {
            Some(u32::from_be_bytes([
                self.value[0],
                self.value[1],
                self.value[2],
                self.value[3],
            ]))
        } else {
            None
        }
    }
    
    /// Get the property value as a u64 (if it's 8 bytes)
    pub fn as_u64(&self) -> Option<u64> {
        if self.value.len() == 8 {
            Some(u64::from_be_bytes([
                self.value[0],
                self.value[1],
                self.value[2],
                self.value[3],
                self.value[4],
                self.value[5],
                self.value[6],
                self.value[7],
            ]))
        } else {
            None
        }
    }
}
