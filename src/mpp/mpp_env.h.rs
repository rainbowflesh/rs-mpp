#[link(name = "rockchip_mpp")]
extern "C" {
    pub fn mpp_env_get_u32(
        name: *const ::std::os::raw::c_char,
        value: *mut RK_U32,
        default_value: RK_U32,
    ) -> RK_S32;
}

#[link(name = "rockchip_mpp")]
extern "C" {
    pub fn mpp_env_get_str(
        name: *const ::std::os::raw::c_char,
        value: *mut *const ::std::os::raw::c_char,
        default_value: *const ::std::os::raw::c_char,
    ) -> RK_S32;
}

#[link(name = "rockchip_mpp")]
extern "C" {
    pub fn mpp_env_set_u32(name: *const ::std::os::raw::c_char, value: RK_U32) -> RK_S32;
}

#[link(name = "rockchip_mpp")]
extern "C" {
    pub fn mpp_env_set_str(
        name: *const ::std::os::raw::c_char,
        value: *mut ::std::os::raw::c_char,
    ) -> RK_S32;
}
