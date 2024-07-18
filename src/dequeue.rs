#[allow(unused_imports)]
use std::{alloc::{alloc, dealloc, realloc,Layout}, ptr::{NonNull,copy}};
use std:: ptr;

#[allow(unused)]
pub struct DeQueue<T>{
    ptr:NonNull<T>,
    head:usize,
    tail:usize,
    len:usize,
    size:usize
}
impl<T> DeQueue<T>{
    #[allow(unused)]
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
    #[allow(unused)]
    fn grow(&mut self){
        let new_size=self.size*2;
        let new_layout=Layout::array::<T>(new_size).unwrap();
        let new_ptr={
            unsafe {
                NonNull::new(realloc(self.ptr.as_ptr() as *mut u8, new_layout, new_size) as *mut T).unwrap()
            }
        };
        self.ptr=new_ptr;
        unsafe {
            if self.tail>self.head{
                let new_tail=self.tail+new_size-self.size;
                ptr::copy(
                    self.ptr.as_ptr().add(self.tail),
                    self.ptr.as_ptr().add(new_tail),
                    self.size-self.tail
                );
                self.tail=new_tail;
            }
            else {
                let new_head=self.head+new_size-self.size;
                ptr::copy(
                    self.ptr.as_ptr().add(self.head),
                    self.ptr.as_ptr().add(new_head),
                    self.size-self.tail
                );
                self.head=new_head;
            }
            
        }
        self.size=new_size;
    }
    #[allow(unused)]
    pub fn push_front(&mut self,data:T){
        if self.head==0 && self.len==0{
            unsafe {
                ptr::write(self.ptr.as_ptr(), data)
            }
        }
        else {
            let idx=(self.head+1)%self.size;
            unsafe {
                ptr::write(self.ptr.as_ptr().add(idx), data)
            }
            self.head=idx;
        }
        self.len+=1;
        if self.size-self.len==1{
            self.grow()
        }
    }
    #[allow(unused)]
    pub fn push_back(&mut self,data:T){
        if self.len==0 && self.tail==0{
            unsafe {
                ptr::write(self.ptr.as_ptr(), data)
            }
        }
        else if self.tail==0 {
            let idx=self.size-1;
            unsafe {
                ptr::write(self.ptr.as_ptr().add(idx), data)
            }
            self.tail=idx;
        }
        else {
            let idx=(self.tail-1)%self.size;
            unsafe {
                ptr::write(self.ptr.as_ptr().add(idx), data)
            }
            self.tail=idx;
        }
        self.len+=1;
        if self.size-self.len==1{
            self.grow()
        }
    }
    #[allow(unused)]
    pub fn pop_front(&mut self)->Option<T>{
        let val=if self.len==0{
            None
        }else if self.len==1{
            unsafe {
                Some(self.ptr.as_ptr().add(self.head).read())    
            }            
        }else {
            if self.head==0{
                unsafe {
                    self.head=self.size;
                    Some(self.ptr.as_ptr().add(0).read())   
                }
            }else {
                let idx=(self.head-1);
                self.head=idx;
                unsafe {
                    Some(self.ptr.as_ptr().add(idx+1).read())
                }
            }
        };
        self.len-=1;
        val
        
    }
    #[allow(unused)]
    pub fn pop_back(&mut self)->Option<T>{
        todo!()
    }
    #[allow(unused)]
    pub fn len(&self)->usize{
        self.len
    }
}

unsafe  impl<T> Send for DeQueue<T>  {}