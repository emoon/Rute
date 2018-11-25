// This file is auto-generated by rute_gen. DO NOT EDIT.
use std::cell::Cell;
use std::rc::Rc;

#[allow(unused_imports)]
use std::marker::PhantomData;

#[allow(unused_imports)]
use std::os::raw::c_void;

#[allow(unused_imports)]
use std::mem::transmute;

#[allow(unused_imports)]
use std::ffi::{CStr, CString};

use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::*;

/// **Notice these docs are heavy WIP and not very relevent yet**
///
/// In Qt there is a often a need to represent the layout of the pixels in a
/// graphics buffer. Internally QPixelFormat stores everything in a 64 bit
/// datastructure. This gives performance but also some limitations.
///
/// QPixelFormat can describe 5 color channels and 1 alpha channel, each can use
/// 6 bits to describe the size of the color channel.
///
/// The position of the alpha channel is described with a separate enum. This is
/// to make it possible to describe QImage formats like ARGB32, and also
/// describe typical OpenGL formats like RBGA8888.
///
/// How pixels are suppose to be read is determined by the TypeInterpretation
/// enum. It describes if color values are suppose to be read byte per byte,
/// or if a pixel is suppose to be read as a complete int and then masked.
/// **See also:** TypeInterpretation
///
/// There is no support for describing YUV's macro pixels. Instead a list of YUV
/// formats has been made. When a QPixelFormat is describing a YUV format, the
/// bitsPerPixel value has been deduced by the YUV Layout enum. Also, the color
/// channels should all be set to zero except the fifth color channel that
/// should store the bitsPerPixel value.
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct PixelFormat<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUPixelFormatAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> PixelFormat<'a> {
    pub fn new() -> PixelFormat<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_pixel_format)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void,
            )
        };

        data.set(Some(ffi_data.qt_data));

        PixelFormat {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUPixelFormat) -> PixelFormat<'a> {
        PixelFormat {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUPixelFormat) -> PixelFormat<'a> {
        PixelFormat {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUPixelFormat) -> PixelFormat<'a> {
        PixelFormat {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Accessor function for getting the colorModel.
    pub fn color_model(&self) -> ColorModel {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).color_model)(obj_data);
            let ret_val = { transmute::<i32, ColorModel>(ret_val) };
            ret_val
        }
    }
    ///
    /// Accessor function for getting the channelCount. Channel Count is deduced
    /// by color channels with a size > 0 and if the size of the alpha channel is > 0.
    pub fn channel_count(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).channel_count)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the size of the red color channel.
    pub fn red_size(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).red_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the size of the green color channel.
    pub fn green_size(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).green_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the size of the blue color channel.
    pub fn blue_size(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).blue_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the cyan color channel.
    pub fn cyan_size(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).cyan_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the megenta color channel.
    pub fn magenta_size(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).magenta_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the yellow color channel.
    pub fn yellow_size(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).yellow_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the black/key color channel.
    pub fn black_size(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).black_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the hue channel size.
    pub fn hue_size(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).hue_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the saturation channel size.
    pub fn saturation_size(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).saturation_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the lightness channel size.
    pub fn lightness_size(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).lightness_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the brightness channel size.
    pub fn brightness_size(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).brightness_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the alpha channel size.
    pub fn alpha_size(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).alpha_size)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for the bits used per pixel. This function returns the
    /// sum of the color channels + the size of the alpha channel.
    pub fn bits_per_pixel(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).bits_per_pixel)(obj_data);
            ret_val
        }
    }
    ///
    /// Accessor function for alphaUsage.
    pub fn alpha_usage(&self) -> AlphaUsage {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).alpha_usage)(obj_data);
            let ret_val = { transmute::<i32, AlphaUsage>(ret_val) };
            ret_val
        }
    }
    ///
    /// Accessor function for alphaPosition.
    pub fn alpha_position(&self) -> AlphaPosition {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).alpha_position)(obj_data);
            let ret_val = { transmute::<i32, AlphaPosition>(ret_val) };
            ret_val
        }
    }
    ///
    /// Accessor function for the AlphaPremultiplied enum. This indicates if the
    /// alpha channel is multiplied in to the color channels.
    ///
    pub fn premultiplied(&self) -> AlphaPremultiplied {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).premultiplied)(obj_data);
            let ret_val = { transmute::<i32, AlphaPremultiplied>(ret_val) };
            ret_val
        }
    }
    ///
    /// Accessor function for the type representation of a color channel or a pixel.
    ///
    /// **See also:** TypeInterpretation
    pub fn type_interpretation(&self) -> TypeInterpretation {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).type_interpretation)(obj_data);
            let ret_val = { transmute::<i32, TypeInterpretation>(ret_val) };
            ret_val
        }
    }
    ///
    /// The byte order is almost always set the the byte order of the current
    /// system. However, it can be useful to describe some YUV formats. This
    /// function should never return QPixelFormat::CurrentSystemEndian as this
    /// value is translated to a endian value in the constructor.
    pub fn byte_order(&self) -> ByteOrder {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).byte_order)(obj_data);
            let ret_val = { transmute::<i32, ByteOrder>(ret_val) };
            ret_val
        }
    }
    ///
    /// Accessor function for the YUVLayout. It is difficult to describe the color
    /// channels of a YUV pixel format since YUV color model uses macro pixels.
    /// Instead the layout of the pixels are stored as an enum.
    pub fn yuv_layout(&self) -> YUVLayout {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).yuv_layout)(obj_data);
            let ret_val = { transmute::<i32, YUVLayout>(ret_val) };
            ret_val
        }
    }
    pub fn sub_enum(&self) -> u8 {
        let (obj_data, funcs) = self.get_pixel_format_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).sub_enum)(obj_data);
            ret_val
        }
    }
}
pub trait PixelFormatTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_pixel_format_obj_funcs(&self) -> (*const RUBase, *const RUPixelFormatFuncs);
}

impl<'a> PixelFormatTrait<'a> for PixelFormat<'a> {
    #[doc(hidden)]
    fn get_pixel_format_obj_funcs(&self) -> (*const RUBase, *const RUPixelFormatFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).pixel_format_funcs) }
    }
}
#[repr(u32)]
pub enum ColorModel {
    Rgb,
    Bgr,
    Indexed,
    Grayscale,
    Cmyk,
    Hsl,
    Hsv,
    Yuv,
    Alpha,
}

#[repr(u32)]
pub enum AlphaUsage {
    UsesAlpha,
    IgnoresAlpha,
}

#[repr(u32)]
pub enum AlphaPosition {
    AtBeginning,
    AtEnd,
}

#[repr(u32)]
pub enum AlphaPremultiplied {
    NotPremultiplied,
    Premultiplied,
}

#[repr(u32)]
pub enum TypeInterpretation {
    UnsignedInteger,
    UnsignedShort,
    UnsignedByte,
    FloatingPoint,
}

#[repr(u32)]
pub enum YUVLayout {
    YuV444,
    YuV422,
    YuV411,
    YuV420P,
    YuV420Sp,
    YV12,
    Uyvy,
    Yuyv,
    NV12,
    NV21,
    ImC1,
    ImC2,
    ImC3,
    ImC4,
    Y8,
    Y16,
}

#[repr(u32)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
    CurrentSystemEndian,
}
