#[allow(unused_imports)]
use std::{alloc::{alloc, dealloc, realloc,Layout}, ptr::{NonNull,copy}};


#[allow(unused)]
pub struct DeQueue<T>{
    ptr:NonNull<T>,
    head:usize,
    tail:usize,
    len:usize,
    size:usize
}
impl<T> DeQueue<T>{
    pub fn new()->Self{
        DeQueue { 
            ptr: {
                let l=Layout::array::<T>(12).unwrap();
                unsafe {
                    NonNull::new(alloc(l) as *mut T).unwrap()
                }
            }, 
            head: 0, 
            tail: 0, 
            len: 0, 
            size: 12 
        }
    }
    fn grow(&mut self){
        let new_size=self.size*2;
        // if isize::MAX==usize::MAX
        let new_layout=Layout::array::<T>(new_size).unwrap();
        let new_ptr={
            unsafe {
                NonNull::new(realloc(self.ptr.as_ptr() as *mut u8, new_layout, new_size) as *mut T).unwrap()
            }
        };
        self.ptr=new_ptr;
        self.size=new_size;
    }
    pub fn push_front(&mut self,data:T){

    }
}