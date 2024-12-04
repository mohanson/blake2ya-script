#![no_main]
#![no_std]

extern crate alloc;
extern crate ckbes;
use alloc::format;

#[no_mangle]
pub unsafe extern "C" fn main() -> u64 {
    let data = [0; 4096];
    let e = [
        0x89, 0x81, 0xdc, 0x7f, 0x54, 0x1b, 0x4c, 0x77, 0x74, 0x4c, 0xe3, 0x2b, 0x8e, 0x9b, 0x03, 0xa6, 0x9e, 0xf8,
        0x31, 0x80, 0x79, 0x73, 0xee, 0x60, 0x5e, 0x3e, 0xb8, 0x43, 0x37, 0xce, 0x39, 0x9c,
    ];

    {
        ckbes::syscall::debug("bench blake2b_ref");
        let tic = ckbes::syscall::current_cycles();
        for _ in 0..1 {
            let mut h = blake2b_ref::Blake2bBuilder::new(32).personal(b"ckb-default-hash").build();
            let mut r = [0; 32];
            h.update(&data);
            h.finalize(&mut r);
            assert_eq!(r, e);
        }
        let toc = ckbes::syscall::current_cycles();
        ckbes::syscall::debug(&format!("{}", toc - tic));
    }

    {
        ckbes::syscall::debug("bench blake2ya");
        let tic = ckbes::syscall::current_cycles();
        for _ in 0..1 {
            let mut p = blake2ya::blake2b_params();
            p.digest(32);
            p.person(b"ckb-default-hash");
            let mut h = blake2ya::blake2b(p);
            h.update(&data);
            let mut r = [0; 32];
            h.digest(&mut r);
            assert_eq!(r, e);
        }
        let toc = ckbes::syscall::current_cycles();
        ckbes::syscall::debug(&format!("{}", toc - tic));
    }

    return 0;
}
