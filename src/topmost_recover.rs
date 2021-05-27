use std::arch::x86_64::*;

macro_rules! loadargs {
    ( $i:expr, $iter:ident,
      $r0:ident, $r1:ident, $r2:ident, $r3:ident ) => {
        let $r0 = _mm256_load_si256($i.add($iter << 7) as *const std::arch::x86_64::__m256i);
        let $r1 = _mm256_load_si256($i.add(($iter << 7) + 32) as *const std::arch::x86_64::__m256i);
        let $r2 = _mm256_load_si256($i.add(($iter << 7) + 64) as *const std::arch::x86_64::__m256i);
        let $r3 = _mm256_load_si256($i.add(($iter << 7) + 96) as *const std::arch::x86_64::__m256i);
    };
}

macro_rules! loadargs_mut {
    ( $i:expr, $iter:ident,
      $r0:ident, $r1:ident, $r2:ident, $r3:ident ) => {
        let mut $r0 = _mm256_load_si256($i.add($iter << 7) as *const std::arch::x86_64::__m256i);
        let mut $r1 =
            _mm256_load_si256($i.add(($iter << 7) + 32) as *const std::arch::x86_64::__m256i);
        let mut $r2 =
            _mm256_load_si256($i.add(($iter << 7) + 64) as *const std::arch::x86_64::__m256i);
        let mut $r3 =
            _mm256_load_si256($i.add(($iter << 7) + 96) as *const std::arch::x86_64::__m256i);
    };
}

macro_rules! xors {
    (
        $x0:ident, $y0:ident,
        $x1:ident, $y1:ident,
        $x2:ident, $y2:ident,
        $x3:ident, $y3:ident
    ) => {
        let $x0 = _mm256_xor_si256($x0, $y0);
        let $x1 = _mm256_xor_si256($x1, $y1);
        let $x2 = _mm256_xor_si256($x2, $y2);
        let $x3 = _mm256_xor_si256($x3, $y3);
    };
}

macro_rules! xors_mut {
    (
        $x0:ident, $y0:ident,
        $x1:ident, $y1:ident,
        $x2:ident, $y2:ident,
        $x3:ident, $y3:ident
    ) => {
        $x0 = _mm256_xor_si256($x0, $y0);
        $x1 = _mm256_xor_si256($x1, $y1);
        $x2 = _mm256_xor_si256($x2, $y2);
        $x3 = _mm256_xor_si256($x3, $y3);
    };
}

macro_rules! load_compute {
    ( $i:expr, $iter:ident,
      $r0:ident, $r1:ident, $r2:ident, $r3:ident,
      $s0:ident, $s1:ident, $s2:ident, $s3:ident ) => {
        loadargs!($i, $iter, $s0, $s1, $s2, $s3);
        xors!($r0, $s0, $r1, $s1, $r2, $s2, $r3, $s3);
    };
}

macro_rules! load_compute_mut {
    ( $i:expr, $iter:ident,
      $r0:ident, $r1:ident, $r2:ident, $r3:ident,
      $s0:ident, $s1:ident, $s2:ident, $s3:ident ) => {
        loadargs!($i, $iter, $s0, $s1, $s2, $s3);
        xors_mut!($r0, $s0, $r1, $s1, $r2, $s2, $r3, $s3);
    };
}

pub fn recover_from_srcs_and_parity(dst: &[u8], v: &[&[u8]], parity: &[u8]) {
    unsafe {
        match v.len() {
            7 => {
                recover_from_7p(dst, v, parity);
            }
            8 => {
                recover_from_8p(dst, v, parity);
            }
            9 => {
                recover_from_9p(dst, v, parity);
            }
            _ => {
                unimplemented!("error");
            }
        }
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn recover_from_7p(dst: &[u8], v: &[&[u8]], parity: &[u8]) {
    assert!(dst.len() == v[0].len());
    assert!(dst.len() == parity.len());
    assert!(dst.len() % 128 == 0);

    let LEN = dst.len();

    let mut dst: *mut __m256i = dst.as_ptr() as *mut __m256i;
    let v1: *const u8 = v[0].as_ptr();
    let v2: *const u8 = v[1].as_ptr();
    let v3: *const u8 = v[2].as_ptr();
    let v4: *const u8 = v[3].as_ptr();
    let v5: *const u8 = v[4].as_ptr();
    let v6: *const u8 = v[5].as_ptr();
    let v7: *const u8 = v[6].as_ptr();
    let v8: *const u8 = parity.as_ptr();

    for iter in 0..(LEN / 128) {
        loadargs!(v1, iter, reg0, reg1, reg2, reg3);

        load_compute!(v2, iter, reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7);

        load_compute!(v3, iter, reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7);

        load_compute!(v4, iter, reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7);

        load_compute!(v5, iter, reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7);

        load_compute!(v6, iter, reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7);

        load_compute!(v7, iter, reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7);

        load_compute!(v8, iter, reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7);

        _mm256_store_si256(dst, reg0);
        _mm256_store_si256(dst.add(1), reg1);
        _mm256_store_si256(dst.add(2), reg2);
        _mm256_store_si256(dst.add(3), reg3);
        dst = dst.add(4);
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn recover_from_8p(dst: &[u8], v: &[&[u8]], parity: &[u8]) {
    assert!(dst.len() == v[0].len());
    assert!(dst.len() == parity.len());
    assert!(dst.len() % 128 == 0);

    let LEN = dst.len();

    let mut dst: *mut __m256i = dst.as_ptr() as *mut __m256i;
    let v1: *const u8 = v[0].as_ptr();
    let v2: *const u8 = v[1].as_ptr();
    let v3: *const u8 = v[2].as_ptr();
    let v4: *const u8 = v[3].as_ptr();
    let v5: *const u8 = v[4].as_ptr();
    let v6: *const u8 = v[5].as_ptr();
    let v7: *const u8 = v[6].as_ptr();
    let v8: *const u8 = v[7].as_ptr();
    let v9: *const u8 = parity.as_ptr();

    for iter in 0..(LEN / 128) {
        loadargs!(v1, iter, reg0, reg1, reg2, reg3);

        load_compute!(v2, iter, reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7);

        load_compute!(v3, iter, reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7);

        load_compute!(v4, iter, reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7);

        load_compute!(v5, iter, reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7);

        load_compute!(v6, iter, reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7);

        load_compute!(v7, iter, reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7);

        load_compute!(v8, iter, reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7);

        load_compute!(v9, iter, reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7);

        _mm256_store_si256(dst, reg0);
        _mm256_store_si256(dst.add(1), reg1);
        _mm256_store_si256(dst.add(2), reg2);
        _mm256_store_si256(dst.add(3), reg3);
        dst = dst.add(4);
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn recover_from_9p(dst: &[u8], v: &[&[u8]], parity: &[u8]) {
    assert!(dst.len() == v[0].len());
    assert!(dst.len() == parity.len());
    assert!(dst.len() % 128 == 0);

    let LEN = dst.len();

    let mut dst: *mut __m256i = dst.as_ptr() as *mut __m256i;
    let v1: *const u8 = v[0].as_ptr();
    let v2: *const u8 = v[1].as_ptr();
    let v3: *const u8 = v[2].as_ptr();
    let v4: *const u8 = v[3].as_ptr();
    let v5: *const u8 = v[4].as_ptr();
    let v6: *const u8 = v[5].as_ptr();
    let v7: *const u8 = v[6].as_ptr();
    let v8: *const u8 = v[7].as_ptr();
    let v9: *const u8 = v[8].as_ptr();
    let v10: *const u8 = parity.as_ptr();

    for iter in 0..(LEN / 128) {
        loadargs!(v1, iter, reg0, reg1, reg2, reg3);

        load_compute!(v2, iter, reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7);

        load_compute!(v3, iter, reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7);

        load_compute!(v4, iter, reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7);

        load_compute!(v5, iter, reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7);

        load_compute!(v6, iter, reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7);

        load_compute!(v7, iter, reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7);

        load_compute!(v8, iter, reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7);

        load_compute!(v9, iter, reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7);

        load_compute!(v10, iter, reg0, reg1, reg2, reg3, reg4, reg5, reg6, reg7);

        _mm256_store_si256(dst, reg0);
        _mm256_store_si256(dst.add(1), reg1);
        _mm256_store_si256(dst.add(2), reg2);
        _mm256_store_si256(dst.add(3), reg3);
        dst = dst.add(4);
    }
}
