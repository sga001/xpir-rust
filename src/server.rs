use libc;
use std::mem;
use std::slice;
use super::{PirQuery, PirReply};

extern "C" {
    fn cpp_server_setup(
        len: libc::uint64_t,
        collection: *const libc::uint8_t,
        num: libc::uint64_t,
        alpha: libc::uint64_t,
        depth: libc::uint64_t,
    ) -> *mut libc::c_void;

    fn cpp_server_process_query(
        server: *const libc::c_void,
        q: *const libc::uint8_t,
        q_len: libc::uint64_t,
        q_num: libc::uint64_t,
        r_len: *mut libc::uint64_t, // reply length
        r_num: *mut libc::uint64_t,
    ) -> *mut libc::uint8_t;

    fn cpp_server_free(server: *mut libc::c_void);
    fn cpp_buffer_free(buffer: *mut libc::uint8_t);
}

pub struct PirServer<'a> {
    server: &'a mut libc::c_void,
}

impl<'a> Drop for PirServer<'a> {
    fn drop(&mut self) {
        unsafe {
            cpp_server_free(self.server);
        }
    }
}

impl<'a> PirServer<'a> {
    pub fn new<T>(collection: &[T]) -> PirServer<'a> {
        let server_ptr: &'a mut libc::c_void = unsafe {
            &mut *(cpp_server_setup(
                (collection.len() * mem::size_of::<T>()) as u64,
                collection.as_ptr() as *const u8,
                collection.len() as u64,
                8,
                2,
            ))
        };

        PirServer { server: server_ptr }
    }

    pub fn with_params<T>(collection: &[T], alpha: u64, depth: u64) -> PirServer<'a> {
        let server_ptr: &'a mut libc::c_void = unsafe {
            &mut *(cpp_server_setup(
                (collection.len() * mem::size_of::<T>()) as u64,
                collection.as_ptr() as *const u8,
                collection.len() as u64,
                alpha,
                depth,
            ))
        };

        PirServer { server: server_ptr }
    }

    pub fn gen_reply(&self, query: &PirQuery) -> PirReply {
        let mut r_len: u64 = 0;
        let mut r_num: u64 = 0;

        let reply: Vec<u8> = unsafe {
            let ptr = cpp_server_process_query(
                self.server,
                query.query.as_ptr(),
                query.query.len() as u64,
                query.num,
                &mut r_len,
                &mut r_num,
            );
            let rep = slice::from_raw_parts_mut(ptr as *mut u8, r_len as usize).to_vec();
            cpp_buffer_free(ptr);
            rep
        };

        PirReply {
            reply: reply,
            num: r_num,
        }
    }
}
