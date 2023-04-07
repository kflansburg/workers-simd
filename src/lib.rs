use std::arch::wasm32::*;
use js_sys::Date;

pub unsafe fn multiply_arrays(out: &mut [i32], in_a: &[i32], in_b: &[i32]) {
  in_a.chunks(4)
    .zip(in_b.chunks(4))
    .zip(out.chunks_mut(4))
    .for_each(|((a, b), dst)| {
      let a = v128_load(a.as_ptr() as *const v128);
      let b = v128_load(b.as_ptr() as *const v128);
      let prod = i32x4_mul(a, b);
      v128_store(dst.as_mut_ptr() as *mut v128, prod);
    });
}

use worker::*;

#[event(fetch)]
async fn main(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let a = [2i32; 1000];
    let b = [3i32; 1000];
    let mut out = [0i32; 1000];
    let tic = Date::now();
    unsafe { multiply_arrays(&mut out, &a, &b); }
    let toc = Date::now();
    // Note: timer shows no time because we do not allow precise timings to mitigate Spectre. 
    // We should explore timing from the client side for now. 
    console_log!("{} in {:.4} ms", out[0], toc - tic);
    Response::ok("Hello, World!")
}
