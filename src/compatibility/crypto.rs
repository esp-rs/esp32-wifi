pub fn init_crypto_funcs() -> crate::binary::wifi::wpa_crypto_funcs_t {
    crate::binary::wifi::wpa_crypto_funcs_t {
        size: core::mem::size_of::<crate::binary::wifi::wpa_crypto_funcs_t>() as u32,
        version: 0x00000001,
        aes_wrap: Some(aes_wrap),
        aes_unwrap: Some(aes_unwrap),
        hmac_sha256_vector: Some(hmac_sha256_vector),
        sha256_prf: Some(sha256_prf),
        hmac_md5: Some(hmac_md5),
        hamc_md5_vector: Some(hmac_md5_vector),
        hmac_sha1: Some(hmac_sha1),
        hmac_sha1_vector: Some(hmac_sha1_vector),
        sha1_prf: Some(sha1_prf),
        sha1_vector: Some(sha1_vector),
        pbkdf2_sha1: Some(pbkdf2_sha1),
        rc4_skip: Some(rc4_skip),

        md5_vector: Some(md5_vector),
        aes_encrypt: Some(aes_encrypt),
        aes_encrypt_init: Some(aes_encrypt_init),
        aes_encrypt_deinit: Some(aes_encrypt_deinit),
        aes_decrypt: Some(aes_decrypt),
        aes_decrypt_init: Some(aes_decrypt_init),
        aes_decrypt_deinit: Some(aes_decrypt_deinit),

        omac1_aes_128: Some(omac1_aes_128),
        ccmp_decrypt: Some(ccmp_decrypt),
        ccmp_encrypt: Some(ccmp_encrypt),
    }
}

pub unsafe extern "C" fn aes_wrap(
    kek: *const cty::c_uchar,
    n: cty::c_int,
    plain: *const cty::c_uchar,
    cipher: *mut cty::c_uchar,
) -> cty::c_int {
    unimplemented!();
}

pub unsafe extern "C" fn aes_unwrap(
    kek: *const cty::c_uchar,
    n: cty::c_int,
    cipher: *const cty::c_uchar,
    plain: *mut cty::c_uchar,
) -> cty::c_int {
    unimplemented!();
}

pub unsafe extern "C" fn hmac_sha256_vector(
    key: *const cty::c_uchar,
    key_len: cty::c_int,
    num_elem: cty::c_int,
    addr: *mut *const cty::c_uchar,
    len: *const cty::c_int,
    mac: *mut cty::c_uchar,
) -> cty::c_int {
    unimplemented!();
}

pub unsafe extern "C" fn sha256_prf(
    key: *const cty::c_uchar,
    key_len: cty::c_int,
    label: *const cty::c_char,
    data: *const cty::c_uchar,
    data_len: cty::c_int,
    buf: *mut cty::c_uchar,
    buf_len: cty::c_int,
) -> cty::c_int {
    unimplemented!();
}

pub unsafe extern "C" fn hmac_md5(
    key: *const cty::c_uchar,
    key_len: cty::c_uint,
    data: *const cty::c_uchar,
    data_len: cty::c_uint,
    mac: *mut cty::c_uchar,
) -> cty::c_int {
    unimplemented!();
}

pub unsafe extern "C" fn hmac_md5_vector(
    key: *const cty::c_uchar,
    key_len: cty::c_uint,
    num_elem: cty::c_uint,
    addr: *mut *const cty::c_uchar,
    len: *const cty::c_uint,
    mac: *mut cty::c_uchar,
) -> cty::c_int {
    unimplemented!();
}

pub unsafe extern "C" fn hmac_sha1(
    key: *const cty::c_uchar,
    key_len: cty::c_uint,
    data: *const cty::c_uchar,
    data_len: cty::c_uint,
    mac: *mut cty::c_uchar,
) -> cty::c_int {
    unimplemented!();
}

pub unsafe extern "C" fn hmac_sha1_vector(
    key: *const cty::c_uchar,
    key_len: cty::c_uint,
    num_elem: cty::c_uint,
    addr: *mut *const cty::c_uchar,
    len: *const cty::c_uint,
    mac: *mut cty::c_uchar,
) -> cty::c_int {
    unimplemented!();
}

pub unsafe extern "C" fn sha1_prf(
    key: *const cty::c_uchar,
    key_len: cty::c_uint,
    label: *const cty::c_char,
    data: *const cty::c_uchar,
    data_len: cty::c_uint,
    buf: *mut cty::c_uchar,
    buf_len: cty::c_uint,
) -> cty::c_int {
    unimplemented!();
}

pub unsafe extern "C" fn sha1_vector(
    num_elem: cty::c_uint,
    addr: *mut *const cty::c_uchar,
    len: *const cty::c_uint,
    mac: *mut cty::c_uchar,
) -> cty::c_int {
    unimplemented!();
}

pub unsafe extern "C" fn pbkdf2_sha1(
    passphrase: *const cty::c_char,
    ssid: *const cty::c_char,
    ssid_len: cty::c_uint,
    iterations: cty::c_int,
    buf: *mut cty::c_uchar,
    buflen: cty::c_uint,
) -> cty::c_int {
    unimplemented!();
}

pub unsafe extern "C" fn rc4_skip(
    key: *const cty::c_uchar,
    keylen: cty::c_uint,
    skip: cty::c_uint,
    data: *mut cty::c_uchar,
    data_len: cty::c_uint,
) -> cty::c_int {
    unimplemented!();
}

pub unsafe extern "C" fn md5_vector(
    num_elem: cty::c_uint,
    addr: *mut *const cty::c_uchar,
    len: *const cty::c_uint,
    mac: *mut cty::c_uchar,
) -> cty::c_int {
    unimplemented!();
}

pub unsafe extern "C" fn aes_encrypt(
    ctx: *mut cty::c_void,
    plain: *const cty::c_uchar,
    crypt: *mut cty::c_uchar,
) {
    unimplemented!();
}

pub unsafe extern "C" fn aes_encrypt_init(
    key: *const cty::c_uchar,
    len: cty::c_uint,
) -> *mut cty::c_void {
    unimplemented!();
}

pub unsafe extern "C" fn aes_encrypt_deinit(ctx: *mut cty::c_void) {
    unimplemented!();
}

pub unsafe extern "C" fn aes_decrypt(
    ctx: *mut cty::c_void,
    crypt: *const cty::c_uchar,
    plain: *mut cty::c_uchar,
) {
    unimplemented!();
}

pub unsafe extern "C" fn aes_decrypt_init(
    key: *const cty::c_uchar,
    len: cty::c_uint,
) -> *mut cty::c_void {
    unimplemented!();
}

pub unsafe extern "C" fn aes_decrypt_deinit(ctx: *mut cty::c_void) {
    unimplemented!();
}

pub unsafe extern "C" fn omac1_aes_128(
    key: *const u8,
    data: *const u8,
    data_len: usize,
    mic: *mut u8,
) -> cty::c_int {
    unimplemented!();
}

pub unsafe extern "C" fn ccmp_decrypt(
    tk: *const u8,
    ieee80211_hdr: *const u8,
    data: *const u8,
    data_len: usize,
    decrypted_len: *mut usize,
) -> *mut u8 {
    unimplemented!();
}

pub unsafe extern "C" fn ccmp_encrypt(
    tk: *const u8,
    frame: *mut u8,
    len: usize,
    hdrlen: usize,
    pn: *mut u8,
    keyid: cty::c_int,
    encrypted_len: *mut usize,
) -> *mut u8 {
    unimplemented!();
}
