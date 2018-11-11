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

///
/// The QBitmap class is a monochrome off-screen paint device used
/// mainly for creating custom QCursor and QBrush objects,
/// constructing QRegion objects, and for setting masks for pixmaps
/// and widgets.
///
/// QBitmap is a QPixmap subclass ensuring a depth of 1, except for
/// null objects which have a depth of 0. If a pixmap with a depth
/// greater than 1 is assigned to a bitmap, the bitmap will be
/// dithered automatically.
///
/// Use the QColor objects Qt::color0 and Qt::color1 when drawing on a
/// QBitmap object (or a QPixmap object with depth 1).
///
/// Painting with Qt::color0 sets the bitmap bits to 0, and painting
/// with Qt::color1 sets the bits to 1. For a bitmap, 0-bits indicate
/// background (or transparent pixels) and 1-bits indicate foreground
/// (or opaque pixels). Use the clear() function to set all the bits
/// to Qt::color0. Note that using the Qt::black and Qt::white colors
/// make no sense because the QColor::pixel() value is not necessarily
/// 0 for black and 1 for white.
///
/// The QBitmap class provides the transformed() function returning a
/// transformed copy of the bitmap; use the QTransform argument to
/// translate, scale, shear, and rotate the bitmap. In addition,
/// QBitmap provides the static fromData() function which returns a
/// bitmap constructed from the given `uchar` data, and the static
/// fromImage() function returning a converted copy of a QImage
/// object.
///
/// Just like the QPixmap class, QBitmap is optimized by the use of
/// implicit data sharing. For more information, see the [Implicit
/// Data Sharing](Implicit%0A%20%20%20%20Data%20Sharing)
/// documentation.
///
/// **See also:** QPixmap
/// QImage
/// QImageReader
/// QImageWriter
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct Bitmap<'a> {
    pub data: Rc<Cell<Option<*const RUBase>>>,
    pub all_funcs: *const RUBitmapAllFuncs,
    pub owned: bool,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> Bitmap<'a> {
    pub fn new() -> Bitmap<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_bitmap)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void,
            )
        };

        data.set(Some(ffi_data.qt_data));

        Bitmap {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    pub fn new_from_rc(ffi_data: RUBitmap) -> Bitmap<'a> {
        Bitmap {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    pub fn new_from_owned(ffi_data: RUBitmap) -> Bitmap<'a> {
        Bitmap {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    pub fn new_from_temporary(ffi_data: RUBitmap) -> Bitmap<'a> {
        Bitmap {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
}

pub struct BitmapStatic<'a> {
    pub all_funcs: *const RUBitmapAllFuncs,
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}
pub trait BitmapType<'a> {
    ///
    /// Swaps bitmap *other* with this bitmap. This operation is very
    /// fast and never fails.
    fn swap<B: BitmapType<'a>>(&self, other: &B) -> &Self {
        let (obj_other_1, _funcs) = other.get_bitmap_obj_funcs();

        let (obj_data, funcs) = self.get_bitmap_obj_funcs();
        unsafe {
            ((*funcs).swap)(obj_data, obj_other_1);
        }
        self
    }
    ///
    /// Clears the bitmap, setting all its bits to Qt::color0.
    fn clear(&self) -> &Self {
        let (obj_data, funcs) = self.get_bitmap_obj_funcs();
        unsafe {
            ((*funcs).clear)(obj_data);
        }
        self
    }

    #[inline]
    fn get_bitmap_obj_funcs(&self) -> (*const RUBase, *const RUBitmapFuncs);
}

impl<'a> PaintDeviceType<'a> for Bitmap<'a> {
    #[inline]
    fn get_paint_device_obj_funcs(&self) -> (*const RUBase, *const RUPaintDeviceFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).paint_device_funcs) }
    }
}

impl<'a> PixmapType<'a> for Bitmap<'a> {
    #[inline]
    fn get_pixmap_obj_funcs(&self) -> (*const RUBase, *const RUPixmapFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).pixmap_funcs) }
    }
}

impl<'a> BitmapType<'a> for Bitmap<'a> {
    #[inline]
    fn get_bitmap_obj_funcs(&self) -> (*const RUBase, *const RUBitmapFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).bitmap_funcs) }
    }
}
pub trait BitmapStaticType {
    ///
    /// Returns a copy of the given *image* converted to a bitmap using
    /// the specified image conversion *flags.*
    ///
    /// **See also:** fromData()
    fn from_image<'a, I: ImageType<'a>>(image: &I, flags: ImageConversionFlags) -> Bitmap<'a> {
        let (obj_image_1, _funcs) = image.get_image_obj_funcs();
        let enum_flags_2 = flags as i32;

        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_bitmap)(::std::ptr::null()).all_funcs).bitmap_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).from_image)(obj_data, obj_image_1, enum_flags_2);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Bitmap::new_from_rc(t);
            } else {
                ret_val = Bitmap::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Constructs a bitmap with the given *size,* and sets the contents to
    /// the *bits* supplied.
    ///
    /// The bitmap data has to be byte aligned and provided in in the bit
    /// order specified by *monoFormat.* The mono format must be either
    /// QImage::Format_Mono or QImage::Format_MonoLSB. Use
    /// QImage::Format_Mono to specify data on the XBM format.
    ///
    /// **See also:** fromImage()
    ///
    fn from_data<'a, S: SizeType<'a>>(
        size: &S,
        bits: &uchar<'a>,
        mono_format: Format,
    ) -> Bitmap<'a> {
        let (obj_size_1, _funcs) = size.get_size_obj_funcs();
        let enum_mono_format_3 = mono_format as i32;

        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_bitmap)(::std::ptr::null()).all_funcs).bitmap_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).from_data)(obj_data, obj_size_1, bits, enum_mono_format_3);
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = Bitmap::new_from_rc(t);
            } else {
                ret_val = Bitmap::new_from_owned(t);
            }
            ret_val
        }
    }
}

impl<'a> BitmapStaticType for Bitmap<'a> {}

impl<'a> BitmapStaticType for BitmapStatic<'a> {}