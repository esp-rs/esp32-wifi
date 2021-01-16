
use crate::binary::wpa::wifi_wpa_ie_t;

const TRUE: i32 = 1;
const FALSE: i32 = 0;

const PASS: i32 = TRUE;
const ESP_OK: i32 = 0;


pub(crate) static mut WPA_FUNCS: crate::binary::wpa::wpa_funcs = 
    crate::binary::wpa::wpa_funcs {
        wpa_sta_init: Some(_wpa_sta_init),
        wpa_sta_deinit: Some(_wpa_sta_deinit),
        wpa_sta_connect: Some(_wpa_sta_connect),
        wpa_sta_disconnected_cb: Some(_wpa_sta_disconnected_cb),
        wpa_sta_rx_eapol: Some(_wpa_sta_rx_eapol),
        wpa_sta_in_4way_handshake: Some(_wpa_sta_in_4way_handshake),
        wpa_ap_init: Some(_wpa_ap_init),
        wpa_ap_deinit: Some(_wpa_ap_deinit),
        wpa_ap_join: Some(_wpa_ap_join),
        wpa_ap_remove: Some(_wpa_ap_remove),
        wpa_ap_get_wpa_ie: Some(_wpa_ap_get_wpa_ie),
        wpa_ap_rx_eapol: Some(_wpa_ap_rx_eapol),
        wpa_config_parse_string: Some(_wpa_config_parse_string),
        wpa_parse_wpa_ie: Some(_wpa_parse_wpa_ie),
        wpa_config_bss: Some(_wpa_config_bss),
        wpa_michael_mic_failure: Some(_wpa_michael_mic_failure),
        wpa3_build_sae_msg: Some(_wpa3_build_sae_msg),
        wpa3_parse_sae_msg: Some(_wpa3_parse_sae_msg),
};

unsafe extern "C" fn _wpa_sta_init() -> bool {
    // bool ret = true; 
    // ret = wpa_sm_init(NULL, wpa_sendto_wrapper,
    //              wpa_config_assoc_ie, wpa_install_key, wpa_get_key, wpa_deauthenticate, wpa_neg_complete);
    // if(ret) {   
        // }
        // return ret;
        
    let ret = crate::binary::wpa::esp_wifi_register_tx_cb_internal(Some(eapol_txcb), 3) == ESP_OK;
    // unimplemented!()

    ret
}
unsafe extern "C" fn _wpa_sta_deinit() -> bool {
    unimplemented!()
}

unsafe extern "C" fn _wpa_sta_connect(bssid: *mut u8) {
    unimplemented!()
}

unsafe extern "C" fn _wpa_sta_disconnected_cb(reason_code: u8) {
    unimplemented!()
}

unsafe extern "C" fn _wpa_sta_rx_eapol(src_addr: *mut u8, buf: *mut u8, len: u32) -> cty::c_int {
    unimplemented!()
}

unsafe extern "C" fn _wpa_sta_in_4way_handshake() -> bool {
    unimplemented!() // TODO
}


unsafe extern "C" fn _wpa_ap_init() -> *mut cty::c_void {
    unimplemented!() // TODO
}

unsafe extern "C" fn _wpa_ap_deinit(data: *mut cty::c_void) -> bool {
    unimplemented!() // TODO
}

unsafe extern "C" fn _wpa_ap_join(
                sm: *mut *mut cty::c_void,
                bssid: *mut u8,
                wpa_ie: *mut u8,
                wpa_ie_len: u8,
            ) -> bool {
                unimplemented!() // TODO
            }

unsafe extern "C" fn _wpa_ap_remove(sm: *mut cty::c_void) -> bool {
    unimplemented!() // TODO
}

unsafe extern "C" fn _wpa_ap_get_wpa_ie(len: *mut u8) -> *mut u8 {
    unimplemented!() // TODO
}

unsafe extern "C" fn _wpa_ap_rx_eapol(
                hapd_data: *mut cty::c_void,
                sm: *mut cty::c_void,
                data: *mut u8,
                data_len: usize,
            ) -> bool {
                unimplemented!() // TODO
            }

unsafe extern "C" fn _wpa_config_parse_string(value: *const cty::c_char, len: *mut usize) -> *mut cty::c_char {
    unimplemented!() // TODO
}

unsafe extern "C" fn _wpa_parse_wpa_ie(
                wpa_ie: *const u8,
                wpa_ie_len: usize,
                data: *mut wifi_wpa_ie_t,
            ) -> cty::c_int {
                unimplemented!() // TODO
            }

unsafe extern "C" fn _wpa_config_bss(bssid: *mut u8) -> cty::c_int {
    unimplemented!() // TODO
}

unsafe extern "C" fn _wpa_michael_mic_failure(is_unicast: u16) -> cty::c_int {
    unimplemented!() // TODO
}

unsafe extern "C" fn _wpa3_build_sae_msg(bssid: *mut u8, type_: u32, len: *mut usize) -> *mut u8 {
    unimplemented!() // TODO
}

unsafe extern "C" fn _wpa3_parse_sae_msg(buf: *mut u8, len: usize, type_: u32, status: u16) -> cty::c_int {
    unimplemented!() // TODO
}

unsafe extern "C" fn eapol_txcb(eb: *mut cty::c_void) {
    unimplemented!()
}