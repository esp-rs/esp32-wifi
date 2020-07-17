use crate::compatibility::crypto::init_crypto_funcs;
use crate::compatibility::osi::init_osi_funcs;

pub struct WiFi {
    apb_lock: esp32_hal::clock_control::dfs::LockAPB,
    osi_funcs: crate::binary::wifi::wifi_osi_funcs_t,
}

use crate::binary::wifi::{esp_err_t, esp_event_base_t, TickType_t};

unsafe extern "C" fn system_event_handler(
    event_base: esp_event_base_t,
    event_id: i32,
    event_data: *mut cty::c_void,
    event_data_size: usize,
    ticks_to_wait: TickType_t,
) -> esp_err_t {
    unimplemented!();
}

impl WiFi {
    pub fn new(clock_config: esp32_hal::clock_control::ClockControlConfig) -> WiFi {
        let mut wifi = WiFi {
            apb_lock: clock_config.lock_apb_frequency(),
            osi_funcs: init_osi_funcs(),
        };

        let config = crate::binary::wifi::wifi_init_config_t {
            event_handler: Some(system_event_handler),
            osi_funcs: &mut wifi.osi_funcs,
            wpa_crypto_funcs: init_crypto_funcs(),
            static_rx_buf_num: 10,
            dynamic_rx_buf_num: 32,
            tx_buf_type: 1,
            static_tx_buf_num: 0,
            dynamic_tx_buf_num: 32,
            csi_enable: 0,
            ampdu_rx_enable: 1,
            ampdu_tx_enable: 1,
            nvs_enable: 0,
            nano_enable: 0,
            tx_ba_win: 6,
            rx_ba_win: 6,
            wifi_task_core_id: 0,
            beacon_max_len: 752,
            mgmt_sbuf_num: 32,
            feature_caps: 1, // CONFIG_FEATURE_WPA3_SAE_BIT
            magic: 0x1F2F3F4F,
        };

        unsafe { crate::binary::wifi::esp_wifi_init_internal(&config) };

        wifi
    }
}
