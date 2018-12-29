use std::marker::PhantomData;
use std::os::raw::c_void;
use std::cell::Cell;

#[repr(C)]
#[derive(Default, Copy, Clone, Debug)]
pub struct RUBase {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RUArray {
    pub delete_callback: extern "C" fn(data: *const c_void),
    pub priv_data: *const c_void,
    pub elements: *const c_void,
    pub all_funcs: *const c_void,
    pub owners: *const u8,
    pub count: i32,
}

pub struct PrimitiveArray<T> {
    data: RUArray,
    index: usize,
    _marker: PhantomData<T>,
}

impl<T> PrimitiveArray<T> {
    pub fn new(data: RUArray) -> PrimitiveArray<T> {
        PrimitiveArray {
            index: 0,
            _marker: PhantomData,
            data,
        }
    }
}

impl<T> Drop for PrimitiveArray<T> {
    fn drop(&mut self) {
        // Free the C++ allocated data
        (self.data.delete_callback)(self.data.elements);
    }
}

impl<T> Iterator for PrimitiveArray<T>
where
    T: std::marker::Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.index >= self.data.count as usize {
            None
        } else {
            unsafe {
                let data = std::slice::from_raw_parts(
                    self.data.elements as *const T,
                    self.data.count as usize,
                );
                let index = self.index;
                self.index += 1;
                Some(data[index])
            }
        }
    }
}


#[repr(C)]
#[derive(Clone, Copy)]
pub struct WrapperRcOwn {
    pub(crate) data: *const c_void,
    pub(crate) all_funcs: *const c_void,
    pub(crate) owned: bool,
}

pub struct RefArray<'a, T> {
    array: RUArray,
    index: isize,
    _owner: bool,
    _marker: PhantomData<Cell<&'a T>>,
}

impl<'a, T> RefArray<'a, T> {
    pub fn new(array: RUArray) -> RefArray<'a, T> {
        RefArray {
            array,
            index: 0,
            _owner: true,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for RefArray<'a, T>
where
    T: std::convert::From<WrapperRcOwn>,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        if index >= self.array.count as isize {
            None
        } else {
            self.index += 1;
            unsafe {
                let data = *(self.array.elements as *const *const c_void).offset(index);
                let owned = *self.array.owners.offset(index);

                let t = WrapperRcOwn {
                    data: data,
                    owned: if owned == 1 { true } else { false },
                    all_funcs: self.array.all_funcs,
                };

                Some(t.into())
            }
        }
    }
}

