use anyhow::Result;
fn main() -> Result<()> {
    println!("init");
    Ok(())
}
#[cfg(test)]
mod tests {
    use anyhow::Result;
    use mpp_ffi::{_mpp_log, get_mpp_version, mpp_compat_show, mpp_env_set_u32, show_mpp_version};
    use std::ffi::CString;

    #[test]
    fn logger() -> Result<()> {
        println!("logger started");
        unsafe {
            let mpp_show_history = CString::new("mpp_show_history").unwrap().as_ptr() as *const i8;
            let tag = CString::new("iamtag").unwrap().as_ptr();
            let tag1 = CString::new("iamtag1").unwrap().as_ptr();
            let fmt = CString::new("iamfmt").unwrap().as_ptr();
            let fmt1 = CString::new("iamfmt1").unwrap().as_ptr();
            let func = CString::new("iamfunc").unwrap().as_ptr();
            let func1 = CString::new("iamfunc1").unwrap().as_ptr();
            get_mpp_version();
            // u8/i8 depends on the architecture arm/x86
            // forgive static check error
            (mpp_env_set_u32(mpp_show_history as *const u8, 0));
            println!("woca");
            _mpp_log(tag, fmt, func);
            show_mpp_version();
            mpp_env_set_u32(mpp_show_history as *const u8, 1);
            _mpp_log(tag1, fmt1, func1);
            show_mpp_version();
            mpp_env_set_u32(mpp_show_history as *const u8, 0);
            mpp_compat_show();
            Ok(())
        }
    }
}
