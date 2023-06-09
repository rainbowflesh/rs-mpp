pub const RK_OK: u32 = 0;
pub const RK_SUCCESS: u32 = 0;
pub const MPP_RET_MPP_SUCCESS: MPP_RET = 0;
pub const MPP_RET_MPP_OK: MPP_RET = 0;
pub const MPP_RET_MPP_NOK: MPP_RET = -1;
pub const MPP_RET_MPP_ERR_UNKNOW: MPP_RET = -2;
pub const MPP_RET_MPP_ERR_NULL_PTR: MPP_RET = -3;
pub const MPP_RET_MPP_ERR_MALLOC: MPP_RET = -4;
pub const MPP_RET_MPP_ERR_OPEN_FILE: MPP_RET = -5;
pub const MPP_RET_MPP_ERR_VALUE: MPP_RET = -6;
pub const MPP_RET_MPP_ERR_READ_BIT: MPP_RET = -7;
pub const MPP_RET_MPP_ERR_TIMEOUT: MPP_RET = -8;
pub const MPP_RET_MPP_ERR_PERM: MPP_RET = -9;
pub const MPP_RET_MPP_ERR_BASE: MPP_RET = -1000;
pub const MPP_RET_MPP_ERR_LIST_STREAM: MPP_RET = -1001;
pub const MPP_RET_MPP_ERR_INIT: MPP_RET = -1002;
pub const MPP_RET_MPP_ERR_VPU_CODEC_INIT: MPP_RET = -1003;
pub const MPP_RET_MPP_ERR_STREAM: MPP_RET = -1004;
pub const MPP_RET_MPP_ERR_FATAL_THREAD: MPP_RET = -1005;
pub const MPP_RET_MPP_ERR_NOMEM: MPP_RET = -1006;
pub const MPP_RET_MPP_ERR_PROTOL: MPP_RET = -1007;
pub const MPP_RET_MPP_FAIL_SPLIT_FRAME: MPP_RET = -1008;
pub const MPP_RET_MPP_ERR_VPUHW: MPP_RET = -1009;
pub const MPP_RET_MPP_EOS_STREAM_REACHED: MPP_RET = -1011;
pub const MPP_RET_MPP_ERR_BUFFER_FULL: MPP_RET = -1012;
pub const MPP_RET_MPP_ERR_DISPLAY_FULL: MPP_RET = -1013;
pub type MPP_RET = i32;
pub const MppCompatId_e_MPP_COMPAT_INC_FBC_BUF_SIZE: MppCompatId_e = 0;
pub const MppCompatId_e_MPP_COMPAT_ENC_ASYNC_INPUT: MppCompatId_e = 1;
pub const MppCompatId_e_MPP_COMPAT_DEC_FBC_HDR_256_ODD: MppCompatId_e = 2;
pub const MppCompatId_e_MPP_COMPAT_BUTT: MppCompatId_e = 3;
pub type MppCompatId_e = u32;
pub use self::MppCompatId_e as MppCompatId;
pub const MppCompatType_e_MPP_COMPAT_BOOL: MppCompatType_e = 0;
pub const MppCompatType_e_MPP_COMPAT_S32: MppCompatType_e = 1;
pub const MppCompatType_e_MPP_COMPAT_TYPE_BUTT: MppCompatType_e = 2;
pub type MppCompatType_e = u32;
pub use self::MppCompatType_e as MppCompatType;
pub type MppCompat = MppCompat_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MppCompat_t {
    pub feature_id: MppCompatId,
    pub feature_type: MppCompatType,
    pub value_mpp: RK_S32,
    pub value_usr: RK_S32,
    pub name: *const ::std::os::raw::c_char,
    pub next: *mut MppCompat,
}
#[test]
fn bindgen_test_layout_MppCompat_t() {
    assert_eq!(
        ::std::mem::size_of::<MppCompat_t>(),
        32usize,
        concat!("Size of: ", stringify!(MppCompat_t))
    );
    assert_eq!(
        ::std::mem::align_of::<MppCompat_t>(),
        8usize,
        concat!("Alignment of ", stringify!(MppCompat_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MppCompat_t>())).feature_id as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MppCompat_t),
            "::",
            stringify!(feature_id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MppCompat_t>())).feature_type as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(MppCompat_t),
            "::",
            stringify!(feature_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MppCompat_t>())).value_mpp as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(MppCompat_t),
            "::",
            stringify!(value_mpp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MppCompat_t>())).value_usr as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(MppCompat_t),
            "::",
            stringify!(value_usr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MppCompat_t>())).name as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(MppCompat_t),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MppCompat_t>())).next as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(MppCompat_t),
            "::",
            stringify!(next)
        )
    );
}

#[link(name = "rockchip_mpp")]
extern "C" {
    pub fn mpp_compat_query() -> *mut MppCompat;
}

#[link(name = "rockchip_mpp")]
extern "C" {
    pub fn mpp_compat_query_by_id(id: MppCompatId) -> *mut MppCompat;

}

#[link(name = "rockchip_mpp")]
extern "C" {
    pub fn mpp_compat_update(compat: *mut MppCompat, value: RK_S32) -> MPP_RET;
}

#[link(name = "rockchip_mpp")]
extern "C" {
    pub fn mpp_compat_show();
}
