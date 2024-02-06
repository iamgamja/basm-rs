use alloc::vec;
use crate::math::{modadd, modsub, modmul};
use super::{polymul_u64, polymod_u64};

/// Computes the `n`-th term `a[n]` of a linear recurrence specified by `first_terms` and `coeff`.
/// The recurrence is `a[k] = coeff[0] * a[k-1] + coeff[1] * a[k-2] + ... + coeff[m-1] * a[k-m-1]`
/// where `m` is the length of the `coeff` slice. Also, `a[i] = first_terms[i]` for `0 <= i < m`.
/// 
/// Checks are done to ensure that `first_terms.len() == coeff.len()` and that both are nonempty.
/// 
/// The result is computed in modulo `modulo`.
/// If `modulo` equals 0, it is treated as `2**64`.
/// Note that `modulo` does not need to be a prime.
/// 
/// Current implementation uses the Kitamasa algorithm along with the O(n lg n) NTT division.
/// This is subject to change (e.g., Bostan-Mori).
pub fn linear_nth(first_terms: &[u64], coeff: &[u64], mut n: u128, modulo: u64) -> u64 {
    let m = first_terms.len();
    assert!(m == coeff.len());
    assert!(m > 0);
    if modulo == 1 {
        0
    } else {
        let mut p_base = vec![]; // The modulo base polynomial of Kitamasa
        for x in coeff.iter().rev() {
            p_base.push(if modulo == 0 { 0u64.wrapping_sub(modulo) } else { modsub(0, *x, modulo) });
        }
        p_base.push(1);
        let mut p_pow2 = vec![0, 1];
        let mut p_out = vec![1];
        while n > 0 {
            if (n & 1) != 0 {
                p_out = polymod_u64(&polymul_u64(&p_pow2, &p_out, modulo), &p_base, modulo).unwrap();
            }
            p_pow2 = polymod_u64(&polymul_u64(&p_pow2, &p_pow2, modulo), &p_base, modulo).unwrap();
            n >>= 1;
        }
        let mut ans = 0u64;
        for i in 0..m {
            if i >= p_out.len() { break; }
            let term = if modulo == 0 {
                first_terms[i].wrapping_mul(p_out[i])
            } else {
                modmul(first_terms[i], p_out[i], modulo)
            };
            ans = if modulo == 0 { ans.wrapping_add(term) } else { modadd(ans, term, modulo) };
        }
        ans
    }
}