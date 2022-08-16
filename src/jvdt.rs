// FFI どうやるんじゃ？ -> Win32API によるアプローチに変更
// NOTE: jv-link cls id = 2ab1774d-0c41-11d7-916f-0003479beb3f

use windows::{core::GUID, w, Win32::System::Com::CLSIDFromProgID};

/// recreate AxJVLink class
#[derive(Debug)]
pub struct AxJVLink {
    pub class_id: GUID,
}

impl AxJVLink {
    pub fn new() -> AxJVLink {
        return AxJVLink {
            class_id: get_class_id().unwrap(),
        };
    }
}

fn get_class_id() -> Result<GUID, windows::core::Error> {
    unsafe {
        let class_id: GUID = CLSIDFromProgID(w!("JVDTLab.JVLink"))?;
        Ok(class_id)
    }
}
