#![allow(unused_variables)]

use crate::compatibility::crypto::WPA_CRYPTO_FUNCS;
use crate::compatibility::osi::WIFI_OS_FUNCS;
use crate::wprintln;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub struct WiFi {
    _apb_lock: esp32_hal::clock_control::dfs::LockAPB,
}

use crate::binary::wifi::{
    esp_err_t, esp_event_base_t, esp_interface_t, wifi_mode_t, wifi_sta_config_t, TickType_t,
};

pub type Mode = wifi_mode_t;

#[derive(FromPrimitive, Debug)]
pub enum Error {
    Unknown,
}

impl Error {
    fn from(res: i32) -> Option<Error> {
        if res == 0 {
            None
        } else {
            if let Some(error) = FromPrimitive::from_i32(res) {
                Some(error)
            } else {
                Some(Self::Unknown)
            }
        }
    }
    fn check_and_return<T>(res: i32, value: T) -> Result<T, Error> {
        match Error::from(res) {
            None => return Ok(value),
            Some(error) => Err(error),
        }
    }
    fn convert(res: i32) -> Result<(), Error> {
        Error::check_and_return(res, ())
    }
}

unsafe extern "C" fn system_event_handler(
    event_base: esp_event_base_t,
    event_id: i32,
    event_data: *mut cty::c_void,
    event_data_size: usize,
    ticks_to_wait: TickType_t,
) -> esp_err_t {
    wprintln!(
        "system_event_handler({:x}, {}, {:x}, {}, {})",
        event_base as u32,
        event_id,
        event_data as u32,
        event_data_size,
        ticks_to_wait
    );
    // unimplemented!();
    0
}

impl WiFi {
    pub fn new(clock_config: esp32_hal::clock_control::ClockControlConfig) -> Result<WiFi, Error> {
        let wifi = WiFi {
            _apb_lock: clock_config.lock_apb_frequency(),
        };

        unsafe {
            let config = crate::binary::wifi::wifi_init_config_t {
                event_handler: Some(system_event_handler),
                osi_funcs: &mut WIFI_OS_FUNCS,
                wpa_crypto_funcs: WPA_CRYPTO_FUNCS,
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

            // init wifi
            let res =
                Error::check_and_return(crate::binary::wifi::esp_wifi_init_internal(&config), wifi);

            // enable all logging
            if let Some(error) = Error::from(crate::binary::wifi::esp_wifi_internal_set_log_mod(
                crate::binary::wifi::wifi_log_module_t::WIFI_LOG_MODULE_ALL,
                0,
                true,
            )) {
                Err(error)
            } else {
                res
            }
        }
    }

    pub fn set_mode(&self, mode: Mode) -> Result<(), Error> {
        Error::convert(unsafe { crate::binary::wifi::esp_wifi_set_mode(mode) })
    }
    pub fn set_station_config(&self, config: &mut wifi_sta_config_t) -> Result<(), Error> {
        Error::convert(unsafe {
            crate::binary::wifi::esp_wifi_set_config(
                esp_interface_t::ESP_IF_WIFI_STA,
                config as *mut _ as *mut crate::binary::wifi::wifi_config_t,
            )
        })
    }
    pub fn start(&self) -> Result<(), Error> {
        Error::convert(unsafe { crate::binary::wifi::esp_wifi_start() })
    }
}
