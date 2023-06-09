pub const MPP_LOG_UNKNOWN: u32 = 0;
pub const MPP_LOG_FATAL: u32 = 1;
pub const MPP_LOG_ERROR: u32 = 2;
pub const MPP_LOG_WARN: u32 = 3;
pub const MPP_LOG_INFO: u32 = 4;
pub const MPP_LOG_DEBUG: u32 = 5;
pub const MPP_LOG_VERBOSE: u32 = 6;
pub const MPP_LOG_SILENT: u32 = 7;
pub type size_t = ::std::os::raw::c_ulong;
pub type wchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
#[test]
fn bindgen_test_layout_max_align_t() {
    assert_eq!(
        ::std::mem::size_of::<max_align_t>(),
        32usize,
        concat!("Size of: ", stringify!(max_align_t))
    );
    assert_eq!(
        ::std::mem::align_of::<max_align_t>(),
        16usize,
        concat!("Alignment of ", stringify!(max_align_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<max_align_t>())).__clang_max_align_nonce1 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<max_align_t>())).__clang_max_align_nonce2 as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce2)
        )
    );
}
pub type RK_U8 = ::std::os::raw::c_uchar;
pub type RK_U16 = ::std::os::raw::c_ushort;
pub type RK_U32 = ::std::os::raw::c_uint;
pub type RK_ULONG = ::std::os::raw::c_ulong;
pub type RK_U64 = ::std::os::raw::c_ulonglong;
pub type RK_S8 = ::std::os::raw::c_schar;
pub type RK_S16 = ::std::os::raw::c_short;
pub type RK_S32 = ::std::os::raw::c_int;
pub type RK_LONG = ::std::os::raw::c_long;
pub type RK_S64 = ::std::os::raw::c_longlong;
#[doc = "< decoder"]
pub const MppCtxType_MPP_CTX_DEC: MppCtxType = 0;
#[doc = "< encoder"]
pub const MppCtxType_MPP_CTX_ENC: MppCtxType = 1;
#[doc = "< isp"]
pub const MppCtxType_MPP_CTX_ISP: MppCtxType = 2;
#[doc = "< undefined"]
pub const MppCtxType_MPP_CTX_BUTT: MppCtxType = 3;
#[doc = " @ingroup rk_mpi"]
#[doc = " @brief The type of mpp context"]
#[doc = " @details This type is used when calling mpp_init(), which including decoder,"]
#[doc = "          encoder and Image Signal Process(ISP). So far decoder and encoder"]
#[doc = "          are supported perfectly, and ISP will be supported in the future."]
pub type MppCtxType = u32;
#[doc = "< Value when coding is N/A"]
pub const MppCodingType_MPP_VIDEO_CodingUnused: MppCodingType = 0;
#[doc = "< Autodetection of coding type"]
pub const MppCodingType_MPP_VIDEO_CodingAutoDetect: MppCodingType = 1;
#[doc = "< AKA: H.262"]
pub const MppCodingType_MPP_VIDEO_CodingMPEG2: MppCodingType = 2;
#[doc = "< H.263"]
pub const MppCodingType_MPP_VIDEO_CodingH263: MppCodingType = 3;
#[doc = "< MPEG-4"]
pub const MppCodingType_MPP_VIDEO_CodingMPEG4: MppCodingType = 4;
#[doc = "< Windows Media Video (WMV1,WMV2,WMV3)"]
pub const MppCodingType_MPP_VIDEO_CodingWMV: MppCodingType = 5;
#[doc = "< all versions of Real Video"]
pub const MppCodingType_MPP_VIDEO_CodingRV: MppCodingType = 6;
#[doc = "< H.264/AVC"]
pub const MppCodingType_MPP_VIDEO_CodingAVC: MppCodingType = 7;
#[doc = "< Motion JPEG"]
pub const MppCodingType_MPP_VIDEO_CodingMJPEG: MppCodingType = 8;
#[doc = "< VP8"]
pub const MppCodingType_MPP_VIDEO_CodingVP8: MppCodingType = 9;
#[doc = "< VP9"]
pub const MppCodingType_MPP_VIDEO_CodingVP9: MppCodingType = 10;
#[doc = "< Windows Media Video (WMV1,WMV2,WMV3)"]
pub const MppCodingType_MPP_VIDEO_CodingVC1: MppCodingType = 16777216;
#[doc = "< Sorenson H.263"]
pub const MppCodingType_MPP_VIDEO_CodingFLV1: MppCodingType = 16777217;
#[doc = "< DIVX3"]
pub const MppCodingType_MPP_VIDEO_CodingDIVX3: MppCodingType = 16777218;
pub const MppCodingType_MPP_VIDEO_CodingVP6: MppCodingType = 16777219;
#[doc = "< H.265/HEVC"]
pub const MppCodingType_MPP_VIDEO_CodingHEVC: MppCodingType = 16777220;
#[doc = "< AVS+"]
pub const MppCodingType_MPP_VIDEO_CodingAVSPLUS: MppCodingType = 16777221;
#[doc = "< AVS profile=0x20"]
pub const MppCodingType_MPP_VIDEO_CodingAVS: MppCodingType = 16777222;
#[doc = "< AVS2"]
pub const MppCodingType_MPP_VIDEO_CodingAVS2: MppCodingType = 16777223;
#[doc = "< av1"]
pub const MppCodingType_MPP_VIDEO_CodingAV1: MppCodingType = 16777224;
#[doc = "< Reserved region for introducing Khronos Standard Extensions"]
pub const MppCodingType_MPP_VIDEO_CodingKhronosExtensions: MppCodingType = 1862270976;
#[doc = "< Reserved region for introducing Vendor Extensions"]
pub const MppCodingType_MPP_VIDEO_CodingVendorStartUnused: MppCodingType = 2130706432;
pub const MppCodingType_MPP_VIDEO_CodingMax: MppCodingType = 2147483647;
#[doc = " @ingroup rk_mpi"]
#[doc = " @brief Enumeration used to define the possible video compression codings."]
#[doc = "        sync with the omx_video.h"]
#[doc = ""]
#[doc = " @note  This essentially refers to file extensions. If the coding is"]
#[doc = "        being used to specify the ENCODE type, then additional work"]
#[doc = "        must be done to configure the exact flavor of the compression"]
#[doc = "        to be used.  For decode cases where the user application can"]
#[doc = "        not differentiate between MPEG-4 and H.264 bit streams, it is"]
#[doc = "        up to the codec to handle this."]
pub type MppCodingType = u32;
pub type MppCtx = *mut ::std::os::raw::c_void;
pub type MppParam = *mut ::std::os::raw::c_void;
pub type MppFrame = *mut ::std::os::raw::c_void;
pub type MppPacket = *mut ::std::os::raw::c_void;
pub type MppBuffer = *mut ::std::os::raw::c_void;
pub type MppBufferGroup = *mut ::std::os::raw::c_void;
pub type MppTask = *mut ::std::os::raw::c_void;
pub type MppMeta = *mut ::std::os::raw::c_void;
#[link(name = "rockchip_mpp")]
extern "C" {
    pub fn _mpp_log_l(
        level: ::std::os::raw::c_int,
        tag: *const ::std::os::raw::c_char,
        fmt: *const ::std::os::raw::c_char,
        func: *const ::std::os::raw::c_char,
        ...
    );
}
#[link(name = "rockchip_mpp")]
extern "C" {
    pub fn mpp_set_log_level(level: ::std::os::raw::c_int);
}
#[link(name = "rockchip_mpp")]
extern "C" {
    pub fn mpp_get_log_level() -> ::std::os::raw::c_int;
}
#[link(name = "rockchip_mpp")]
extern "C" {
    pub fn _mpp_log(
        tag: *const ::std::os::raw::c_char,
        fmt: *const ::std::os::raw::c_char,
        func: *const ::std::os::raw::c_char,
        ...
    );
}
#[link(name = "rockchip_mpp")]
extern "C" {
    pub fn _mpp_err(
        tag: *const ::std::os::raw::c_char,
        fmt: *const ::std::os::raw::c_char,
        func: *const ::std::os::raw::c_char,
        ...
    );
}
