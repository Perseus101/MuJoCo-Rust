/* automatically generated by rust-bindgen 0.60.1 */

pub use crate::no_render::*;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct mjrRect_ {
    pub left: ::std::os::raw::c_int,
    pub bottom: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_mjrRect_() {
    assert_eq!(
        ::std::mem::size_of::<mjrRect_>(),
        16usize,
        concat!("Size of: ", stringify!(mjrRect_))
    );
    assert_eq!(
        ::std::mem::align_of::<mjrRect_>(),
        4usize,
        concat!("Alignment of ", stringify!(mjrRect_))
    );
    fn test_field_left() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrRect_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).left) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrRect_),
                "::",
                stringify!(left)
            )
        );
    }
    test_field_left();
    fn test_field_bottom() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrRect_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).bottom) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrRect_),
                "::",
                stringify!(bottom)
            )
        );
    }
    test_field_bottom();
    fn test_field_width() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrRect_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrRect_),
                "::",
                stringify!(width)
            )
        );
    }
    test_field_width();
    fn test_field_height() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrRect_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrRect_),
                "::",
                stringify!(height)
            )
        );
    }
    test_field_height();
}
pub type mjrRect = mjrRect_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mjrContext_ {
    pub lineWidth: f32,
    pub shadowClip: f32,
    pub shadowScale: f32,
    pub fogStart: f32,
    pub fogEnd: f32,
    pub fogRGBA: [f32; 4usize],
    pub shadowSize: ::std::os::raw::c_int,
    pub offWidth: ::std::os::raw::c_int,
    pub offHeight: ::std::os::raw::c_int,
    pub offSamples: ::std::os::raw::c_int,
    pub fontScale: ::std::os::raw::c_int,
    pub auxWidth: [::std::os::raw::c_int; 10usize],
    pub auxHeight: [::std::os::raw::c_int; 10usize],
    pub auxSamples: [::std::os::raw::c_int; 10usize],
    pub offFBO: ::std::os::raw::c_uint,
    pub offFBO_r: ::std::os::raw::c_uint,
    pub offColor: ::std::os::raw::c_uint,
    pub offColor_r: ::std::os::raw::c_uint,
    pub offDepthStencil: ::std::os::raw::c_uint,
    pub offDepthStencil_r: ::std::os::raw::c_uint,
    pub shadowFBO: ::std::os::raw::c_uint,
    pub shadowTex: ::std::os::raw::c_uint,
    pub auxFBO: [::std::os::raw::c_uint; 10usize],
    pub auxFBO_r: [::std::os::raw::c_uint; 10usize],
    pub auxColor: [::std::os::raw::c_uint; 10usize],
    pub auxColor_r: [::std::os::raw::c_uint; 10usize],
    pub ntexture: ::std::os::raw::c_int,
    pub textureType: [::std::os::raw::c_int; 100usize],
    pub texture: [::std::os::raw::c_uint; 100usize],
    pub basePlane: ::std::os::raw::c_uint,
    pub baseMesh: ::std::os::raw::c_uint,
    pub baseHField: ::std::os::raw::c_uint,
    pub baseBuiltin: ::std::os::raw::c_uint,
    pub baseFontNormal: ::std::os::raw::c_uint,
    pub baseFontShadow: ::std::os::raw::c_uint,
    pub baseFontBig: ::std::os::raw::c_uint,
    pub rangePlane: ::std::os::raw::c_int,
    pub rangeMesh: ::std::os::raw::c_int,
    pub rangeHField: ::std::os::raw::c_int,
    pub rangeBuiltin: ::std::os::raw::c_int,
    pub rangeFont: ::std::os::raw::c_int,
    pub nskin: ::std::os::raw::c_int,
    pub skinvertVBO: *mut ::std::os::raw::c_uint,
    pub skinnormalVBO: *mut ::std::os::raw::c_uint,
    pub skintexcoordVBO: *mut ::std::os::raw::c_uint,
    pub skinfaceVBO: *mut ::std::os::raw::c_uint,
    pub charWidth: [::std::os::raw::c_int; 127usize],
    pub charWidthBig: [::std::os::raw::c_int; 127usize],
    pub charHeight: ::std::os::raw::c_int,
    pub charHeightBig: ::std::os::raw::c_int,
    pub glInitialized: ::std::os::raw::c_int,
    pub windowAvailable: ::std::os::raw::c_int,
    pub windowSamples: ::std::os::raw::c_int,
    pub windowStereo: ::std::os::raw::c_int,
    pub windowDoublebuffer: ::std::os::raw::c_int,
    pub currentBuffer: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_mjrContext_() {
    assert_eq!(
        ::std::mem::size_of::<mjrContext_>(),
        2304usize,
        concat!("Size of: ", stringify!(mjrContext_))
    );
    assert_eq!(
        ::std::mem::align_of::<mjrContext_>(),
        8usize,
        concat!("Alignment of ", stringify!(mjrContext_))
    );
    fn test_field_lineWidth() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).lineWidth) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(lineWidth)
            )
        );
    }
    test_field_lineWidth();
    fn test_field_shadowClip() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).shadowClip) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(shadowClip)
            )
        );
    }
    test_field_shadowClip();
    fn test_field_shadowScale() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).shadowScale) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(shadowScale)
            )
        );
    }
    test_field_shadowScale();
    fn test_field_fogStart() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).fogStart) as usize - ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(fogStart)
            )
        );
    }
    test_field_fogStart();
    fn test_field_fogEnd() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).fogEnd) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(fogEnd)
            )
        );
    }
    test_field_fogEnd();
    fn test_field_fogRGBA() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).fogRGBA) as usize - ptr as usize
            },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(fogRGBA)
            )
        );
    }
    test_field_fogRGBA();
    fn test_field_shadowSize() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).shadowSize) as usize - ptr as usize
            },
            36usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(shadowSize)
            )
        );
    }
    test_field_shadowSize();
    fn test_field_offWidth() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).offWidth) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(offWidth)
            )
        );
    }
    test_field_offWidth();
    fn test_field_offHeight() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).offHeight) as usize - ptr as usize
            },
            44usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(offHeight)
            )
        );
    }
    test_field_offHeight();
    fn test_field_offSamples() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).offSamples) as usize - ptr as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(offSamples)
            )
        );
    }
    test_field_offSamples();
    fn test_field_fontScale() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).fontScale) as usize - ptr as usize
            },
            52usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(fontScale)
            )
        );
    }
    test_field_fontScale();
    fn test_field_auxWidth() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).auxWidth) as usize - ptr as usize
            },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(auxWidth)
            )
        );
    }
    test_field_auxWidth();
    fn test_field_auxHeight() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).auxHeight) as usize - ptr as usize
            },
            96usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(auxHeight)
            )
        );
    }
    test_field_auxHeight();
    fn test_field_auxSamples() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).auxSamples) as usize - ptr as usize
            },
            136usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(auxSamples)
            )
        );
    }
    test_field_auxSamples();
    fn test_field_offFBO() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).offFBO) as usize - ptr as usize
            },
            176usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(offFBO)
            )
        );
    }
    test_field_offFBO();
    fn test_field_offFBO_r() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).offFBO_r) as usize - ptr as usize
            },
            180usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(offFBO_r)
            )
        );
    }
    test_field_offFBO_r();
    fn test_field_offColor() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).offColor) as usize - ptr as usize
            },
            184usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(offColor)
            )
        );
    }
    test_field_offColor();
    fn test_field_offColor_r() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).offColor_r) as usize - ptr as usize
            },
            188usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(offColor_r)
            )
        );
    }
    test_field_offColor_r();
    fn test_field_offDepthStencil() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).offDepthStencil) as usize - ptr as usize
            },
            192usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(offDepthStencil)
            )
        );
    }
    test_field_offDepthStencil();
    fn test_field_offDepthStencil_r() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).offDepthStencil_r) as usize - ptr as usize
            },
            196usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(offDepthStencil_r)
            )
        );
    }
    test_field_offDepthStencil_r();
    fn test_field_shadowFBO() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).shadowFBO) as usize - ptr as usize
            },
            200usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(shadowFBO)
            )
        );
    }
    test_field_shadowFBO();
    fn test_field_shadowTex() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).shadowTex) as usize - ptr as usize
            },
            204usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(shadowTex)
            )
        );
    }
    test_field_shadowTex();
    fn test_field_auxFBO() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).auxFBO) as usize - ptr as usize
            },
            208usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(auxFBO)
            )
        );
    }
    test_field_auxFBO();
    fn test_field_auxFBO_r() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).auxFBO_r) as usize - ptr as usize
            },
            248usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(auxFBO_r)
            )
        );
    }
    test_field_auxFBO_r();
    fn test_field_auxColor() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).auxColor) as usize - ptr as usize
            },
            288usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(auxColor)
            )
        );
    }
    test_field_auxColor();
    fn test_field_auxColor_r() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).auxColor_r) as usize - ptr as usize
            },
            328usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(auxColor_r)
            )
        );
    }
    test_field_auxColor_r();
    fn test_field_ntexture() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ntexture) as usize - ptr as usize
            },
            368usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(ntexture)
            )
        );
    }
    test_field_ntexture();
    fn test_field_textureType() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).textureType) as usize - ptr as usize
            },
            372usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(textureType)
            )
        );
    }
    test_field_textureType();
    fn test_field_texture() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).texture) as usize - ptr as usize
            },
            772usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(texture)
            )
        );
    }
    test_field_texture();
    fn test_field_basePlane() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).basePlane) as usize - ptr as usize
            },
            1172usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(basePlane)
            )
        );
    }
    test_field_basePlane();
    fn test_field_baseMesh() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).baseMesh) as usize - ptr as usize
            },
            1176usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(baseMesh)
            )
        );
    }
    test_field_baseMesh();
    fn test_field_baseHField() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).baseHField) as usize - ptr as usize
            },
            1180usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(baseHField)
            )
        );
    }
    test_field_baseHField();
    fn test_field_baseBuiltin() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).baseBuiltin) as usize - ptr as usize
            },
            1184usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(baseBuiltin)
            )
        );
    }
    test_field_baseBuiltin();
    fn test_field_baseFontNormal() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).baseFontNormal) as usize - ptr as usize
            },
            1188usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(baseFontNormal)
            )
        );
    }
    test_field_baseFontNormal();
    fn test_field_baseFontShadow() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).baseFontShadow) as usize - ptr as usize
            },
            1192usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(baseFontShadow)
            )
        );
    }
    test_field_baseFontShadow();
    fn test_field_baseFontBig() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).baseFontBig) as usize - ptr as usize
            },
            1196usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(baseFontBig)
            )
        );
    }
    test_field_baseFontBig();
    fn test_field_rangePlane() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rangePlane) as usize - ptr as usize
            },
            1200usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(rangePlane)
            )
        );
    }
    test_field_rangePlane();
    fn test_field_rangeMesh() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rangeMesh) as usize - ptr as usize
            },
            1204usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(rangeMesh)
            )
        );
    }
    test_field_rangeMesh();
    fn test_field_rangeHField() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rangeHField) as usize - ptr as usize
            },
            1208usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(rangeHField)
            )
        );
    }
    test_field_rangeHField();
    fn test_field_rangeBuiltin() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rangeBuiltin) as usize - ptr as usize
            },
            1212usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(rangeBuiltin)
            )
        );
    }
    test_field_rangeBuiltin();
    fn test_field_rangeFont() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rangeFont) as usize - ptr as usize
            },
            1216usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(rangeFont)
            )
        );
    }
    test_field_rangeFont();
    fn test_field_nskin() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).nskin) as usize - ptr as usize
            },
            1220usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(nskin)
            )
        );
    }
    test_field_nskin();
    fn test_field_skinvertVBO() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).skinvertVBO) as usize - ptr as usize
            },
            1224usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(skinvertVBO)
            )
        );
    }
    test_field_skinvertVBO();
    fn test_field_skinnormalVBO() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).skinnormalVBO) as usize - ptr as usize
            },
            1232usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(skinnormalVBO)
            )
        );
    }
    test_field_skinnormalVBO();
    fn test_field_skintexcoordVBO() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).skintexcoordVBO) as usize - ptr as usize
            },
            1240usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(skintexcoordVBO)
            )
        );
    }
    test_field_skintexcoordVBO();
    fn test_field_skinfaceVBO() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).skinfaceVBO) as usize - ptr as usize
            },
            1248usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(skinfaceVBO)
            )
        );
    }
    test_field_skinfaceVBO();
    fn test_field_charWidth() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).charWidth) as usize - ptr as usize
            },
            1256usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(charWidth)
            )
        );
    }
    test_field_charWidth();
    fn test_field_charWidthBig() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).charWidthBig) as usize - ptr as usize
            },
            1764usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(charWidthBig)
            )
        );
    }
    test_field_charWidthBig();
    fn test_field_charHeight() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).charHeight) as usize - ptr as usize
            },
            2272usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(charHeight)
            )
        );
    }
    test_field_charHeight();
    fn test_field_charHeightBig() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).charHeightBig) as usize - ptr as usize
            },
            2276usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(charHeightBig)
            )
        );
    }
    test_field_charHeightBig();
    fn test_field_glInitialized() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).glInitialized) as usize - ptr as usize
            },
            2280usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(glInitialized)
            )
        );
    }
    test_field_glInitialized();
    fn test_field_windowAvailable() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).windowAvailable) as usize - ptr as usize
            },
            2284usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(windowAvailable)
            )
        );
    }
    test_field_windowAvailable();
    fn test_field_windowSamples() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).windowSamples) as usize - ptr as usize
            },
            2288usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(windowSamples)
            )
        );
    }
    test_field_windowSamples();
    fn test_field_windowStereo() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).windowStereo) as usize - ptr as usize
            },
            2292usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(windowStereo)
            )
        );
    }
    test_field_windowStereo();
    fn test_field_windowDoublebuffer() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).windowDoublebuffer) as usize - ptr as usize
            },
            2296usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(windowDoublebuffer)
            )
        );
    }
    test_field_windowDoublebuffer();
    fn test_field_currentBuffer() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<mjrContext_>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).currentBuffer) as usize - ptr as usize
            },
            2300usize,
            concat!(
                "Offset of field: ",
                stringify!(mjrContext_),
                "::",
                stringify!(currentBuffer)
            )
        );
    }
    test_field_currentBuffer();
}
impl Default for mjrContext_ {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type mjrContext = mjrContext_;
extern "C" {
    pub fn mjr_defaultContext(con: *mut mjrContext);
}
extern "C" {
    pub fn mjr_makeContext(
        m: *const mjModel,
        con: *mut mjrContext,
        fontscale: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mjr_changeFont(fontscale: ::std::os::raw::c_int, con: *mut mjrContext);
}
extern "C" {
    pub fn mjr_addAux(
        index: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        samples: ::std::os::raw::c_int,
        con: *mut mjrContext,
    );
}
extern "C" {
    pub fn mjr_freeContext(con: *mut mjrContext);
}
extern "C" {
    pub fn mjr_uploadTexture(
        m: *const mjModel,
        con: *const mjrContext,
        texid: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mjr_uploadMesh(
        m: *const mjModel,
        con: *const mjrContext,
        meshid: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mjr_uploadHField(
        m: *const mjModel,
        con: *const mjrContext,
        hfieldid: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mjr_restoreBuffer(con: *const mjrContext);
}
extern "C" {
    pub fn mjr_setBuffer(framebuffer: ::std::os::raw::c_int, con: *mut mjrContext);
}
extern "C" {
    pub fn mjr_readPixels(
        rgb: *mut ::std::os::raw::c_uchar,
        depth: *mut f32,
        viewport: mjrRect,
        con: *const mjrContext,
    );
}
extern "C" {
    pub fn mjr_drawPixels(
        rgb: *const ::std::os::raw::c_uchar,
        depth: *const f32,
        viewport: mjrRect,
        con: *const mjrContext,
    );
}
extern "C" {
    pub fn mjr_blitBuffer(
        src: mjrRect,
        dst: mjrRect,
        flg_color: ::std::os::raw::c_int,
        flg_depth: ::std::os::raw::c_int,
        con: *const mjrContext,
    );
}
extern "C" {
    pub fn mjr_setAux(index: ::std::os::raw::c_int, con: *const mjrContext);
}
extern "C" {
    pub fn mjr_blitAux(
        index: ::std::os::raw::c_int,
        src: mjrRect,
        left: ::std::os::raw::c_int,
        bottom: ::std::os::raw::c_int,
        con: *const mjrContext,
    );
}
extern "C" {
    pub fn mjr_text(
        font: ::std::os::raw::c_int,
        txt: *const ::std::os::raw::c_char,
        con: *const mjrContext,
        x: f32,
        y: f32,
        r: f32,
        g: f32,
        b: f32,
    );
}
extern "C" {
    pub fn mjr_overlay(
        font: ::std::os::raw::c_int,
        gridpos: ::std::os::raw::c_int,
        viewport: mjrRect,
        overlay: *const ::std::os::raw::c_char,
        overlay2: *const ::std::os::raw::c_char,
        con: *const mjrContext,
    );
}
extern "C" {
    pub fn mjr_maxViewport(con: *const mjrContext) -> mjrRect;
}
extern "C" {
    pub fn mjr_rectangle(viewport: mjrRect, r: f32, g: f32, b: f32, a: f32);
}
extern "C" {
    pub fn mjr_label(
        viewport: mjrRect,
        font: ::std::os::raw::c_int,
        txt: *const ::std::os::raw::c_char,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
        rt: f32,
        gt: f32,
        bt: f32,
        con: *const mjrContext,
    );
}
extern "C" {
    pub fn mjr_figure(viewport: mjrRect, fig: *mut mjvFigure, con: *const mjrContext);
}
extern "C" {
    pub fn mjr_render(viewport: mjrRect, scn: *mut mjvScene, con: *const mjrContext);
}
extern "C" {
    pub fn mjr_finish();
}
extern "C" {
    pub fn mjr_getError() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mjr_findRect(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        nrect: ::std::os::raw::c_int,
        rect: *const mjrRect,
    ) -> ::std::os::raw::c_int;
}
