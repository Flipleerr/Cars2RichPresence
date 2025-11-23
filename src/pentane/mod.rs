#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PentaneSemVer {
    pub major: i16,
    pub minor: i16,
    pub patch: i16,
}

impl PentaneSemVer {
    pub const fn new(major: i16, minor: i16, patch: i16) -> Self {
        PentaneSemVer {
            major,
            minor,
            patch,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PentaneUUID {
    pub data: [u8; 16],
}

impl PentaneUUID {
    /// Creates a new `PentaneUUID` from a 32-character hexadecimal string.
    pub const fn from_str(s: &str) -> Self {
        const fn hex_char_to_u8(c: char) -> u8 {
            match c {
                '0'..='9' => c as u8 - b'0',
                'a'..='f' => c as u8 - b'a' + 10,
                'A'..='F' => c as u8 - b'A' + 10,
                _ => panic!("Invalid hex character"),
            }
        }

        const fn hex_pair_to_byte(c1: char, c2: char) -> u8 {
            (hex_char_to_u8(c1) << 4) | hex_char_to_u8(c2)
        }

        if s.len() != 32 {
            panic!("UUID must be a 32-character hexadecimal string");
        }

        let mut data = [0u8; 16];
        let chars: [char; 32] = {
            let mut temp = ['\0'; 32];
            let mut i = 0;
            while i < 32 {
                temp[i] = s.as_bytes()[i] as char;
                i += 1;
            }
            temp
        };

        let mut i = 0;
        while i < 16 {
            data[i] = hex_pair_to_byte(chars[i * 2], chars[i * 2 + 1]);
            i += 1;
        }

        Self { data }
    }
}

unsafe impl Sync for PentaneUUID {}
unsafe impl Send for PentaneUUID {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PluginInformation {
    name: [u8; 0x100],
    author: [u8; 0x100],
    uuid: PentaneUUID,
    version: PentaneSemVer,
    minimum_pentane_version: PentaneSemVer,
	reserved: [u8; 484],
}

impl PluginInformation {
    /// Create a new PluginInformation instance using a const fn
    pub const fn new(
        name: &[u8],
        author: &[u8],
        uuid: PentaneUUID,
        version: PentaneSemVer,
        minimum_pentane_version: PentaneSemVer,
    ) -> Self {
        Self {
            name: Self::pad_to_fit(name),
            author: Self::pad_to_fit(author),
            version,
            minimum_pentane_version,
            uuid,
			reserved: [0u8; 484],
        }
    }

    /// Helper function to pad a byte slice to the required length
    const fn pad_to_fit(data: &[u8]) -> [u8; 0x100] {
        let mut padded = [0u8; 0x100];
        let mut i = 0;
        while i < data.len() && i < 0x100 {
            padded[i] = data[i];
            i += 1;
        }
        padded
    }
}

#[repr(C)]
pub struct PentaneCStringView {
    data: *const u8,
    data_len: usize,
}

unsafe extern "C" {
    pub unsafe fn Pentane_LogUTF8(c_str: *const PentaneCStringView);
}

pub fn log_newline() {
    unsafe {
        let view = PentaneCStringView {
            data: std::ptr::null(),
            data_len: 0,
        };
        Pentane_LogUTF8(&view);
    }
}

pub fn log_message(message: &str) {
    unsafe {
        let view = PentaneCStringView {
            data: message.as_bytes().as_ptr(),
            data_len: message.as_bytes().len(),
        };
        Pentane_LogUTF8(&view);
    }
}

/// Prints to the standard output, with a newline. For use in no_std plugins.
#[macro_export]
macro_rules! println {
    () => {
        $crate::log_newline();
    };
    ($($arg:tt)*) => {
        {
            use std::format;
            $crate::log_message(&format!(
                $($arg)*
            ));
        }
    };
}
