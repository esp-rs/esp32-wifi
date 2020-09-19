


// pub(crate) static mut WPA_FUNCS: crate::binary::wpa::wpa_funcs = 
//     crate::binary::wpa::wpa_funcs {
//         pub wpa_sta_init: ::core::option::Option<unsafe extern "C" fn() -> bool>,
//         pub wpa_sta_deinit: ::core::option::Option<unsafe extern "C" fn() -> bool>,
//         pub wpa_sta_connect: ::core::option::Option<unsafe extern "C" fn(bssid: *mut u8)>,
//         pub wpa_sta_disconnected_cb: ::core::option::Option<unsafe extern "C" fn(reason_code: u8)>,
//         pub wpa_sta_rx_eapol: ::core::option::Option<
//             unsafe extern "C" fn(src_addr: *mut u8, buf: *mut u8, len: u32) -> cty::c_int,
//         >,
//         pub wpa_sta_in_4way_handshake: ::core::option::Option<unsafe extern "C" fn() -> bool>,
//         pub wpa_ap_init: ::core::option::Option<unsafe extern "C" fn() -> *mut cty::c_void>,
//         pub wpa_ap_deinit: ::core::option::Option<unsafe extern "C" fn(data: *mut cty::c_void) -> bool>,
//         pub wpa_ap_join: ::core::option::Option<
//             unsafe extern "C" fn(
//                 sm: *mut *mut cty::c_void,
//                 bssid: *mut u8,
//                 wpa_ie: *mut u8,
//                 wpa_ie_len: u8,
//             ) -> bool,
//         >,
//         pub wpa_ap_remove: ::core::option::Option<unsafe extern "C" fn(sm: *mut cty::c_void) -> bool>,
//         pub wpa_ap_get_wpa_ie: ::core::option::Option<unsafe extern "C" fn(len: *mut u8) -> *mut u8>,
//         pub wpa_ap_rx_eapol: ::core::option::Option<
//             unsafe extern "C" fn(
//                 hapd_data: *mut cty::c_void,
//                 sm: *mut cty::c_void,
//                 data: *mut u8,
//                 data_len: usize,
//             ) -> bool,
//         >,
//         pub wpa_config_parse_string: ::core::option::Option<
//             unsafe extern "C" fn(value: *const cty::c_char, len: *mut usize) -> *mut cty::c_char,
//         >,
//         pub wpa_parse_wpa_ie: ::core::option::Option<
//             unsafe extern "C" fn(
//                 wpa_ie: *const u8,
//                 wpa_ie_len: usize,
//                 data: *mut wifi_wpa_ie_t,
//             ) -> cty::c_int,
//         >,
//         pub wpa_config_bss: ::core::option::Option<unsafe extern "C" fn(bssid: *mut u8) -> cty::c_int>,
//         pub wpa_michael_mic_failure:
//             ::core::option::Option<unsafe extern "C" fn(is_unicast: u16) -> cty::c_int>,
//         pub wpa3_build_sae_msg: ::core::option::Option<
//             unsafe extern "C" fn(bssid: *mut u8, type_: u32, len: *mut usize) -> *mut u8,
//         >,
//         pub wpa3_parse_sae_msg: ::core::option::Option<
//             unsafe extern "C" fn(buf: *mut u8, len: usize, type_: u32, status: u16) -> cty::c_int,
//         >,
// };