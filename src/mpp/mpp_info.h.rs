#[link(name = "rockchip_mpp")]
extern "C" {
    pub fn show_mpp_version();
}
#[link(name = "rockchip_mpp")]
extern "C" {
    pub fn get_mpp_version() -> *const ::std::os::raw::c_char;
}
