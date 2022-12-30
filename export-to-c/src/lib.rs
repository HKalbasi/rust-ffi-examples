use std::panic::catch_unwind;

fn primes_rusty(until: usize, mut output: &mut [i32]) -> i32 {
    // We can use allocation, threads, and everything ...
    println!("rust function called with {until}");
    let mut is_not_prime = vec![false; until];
    let mut cnt = 0;
    for i in 2..until {
        if is_not_prime[i] {
            continue;
        }
        if output.is_empty() {
            return -1;
        }
        cnt += 1;
        output[0] = i as i32;
        output = &mut output[1..];
        for j in (i * i..until).step_by(i) {
            is_not_prime[j] = true;
        }
    }
    cnt
}

#[no_mangle]
/// Finds primes until a specified number and put them in output buffer. Returns count of numbers
/// it found.
/// 
/// Returns -1 if output buffer doesn't have enough capacity
/// 
/// # Safety
/// 
/// `output_buf` should be a unique mutable pointer and has at least `output_size` capacity.
pub unsafe extern "C" fn primes(until: i32, output_buf: *mut i32, output_size: i32) -> i32 {
    // Using catch_unwind or panic=abort in Rust/C boundary is good practice, since a panic might happen
    // and cross language unwinding is UB.
    catch_unwind(|| {
        let output = unsafe { std::slice::from_raw_parts_mut(output_buf, output_size as usize) };
        primes_rusty(until as usize, output)
    })
    .unwrap_or(-1)
}
