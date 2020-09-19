#![allow(unused_variables)]

use crate::compatibility::crypto::WPA_CRYPTO_FUNCS;
use crate::compatibility::osi::WIFI_OS_FUNCS;
use crate::wprintln;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub struct WiFi<'a, 'b> {
    _apb_lock: esp32_hal::clock_control::dfs::LockAPB,
    timer_factory: &'a mut dyn crate::timer::TimerFactory<'b>,
}

use crate::binary::wifi::{
    esp_err_t, esp_event_base_t, esp_interface_t, wifi_mode_t, wifi_sta_config_t, TickType_t,
};

pub type Mode = wifi_mode_t;

#[derive(FromPrimitive, Debug)]
#[allow(non_camel_case_types)]
pub enum Error {
    BASE = 0x3000,
    NOT_INIT,
    NOT_STARTED,
    NOT_STOPPED,
    IF,
    MODE,
    STATE,
    CONN,
    NVS,
    MAC,
    SSID,
    PASSWORD,
    TIMEOUT,
    WAKE_FAIL,
    WOULD_BLOCK,
    NOT_CONNECT,

    POST = 0x3000 + 18,
    INIT_STATE,
    STOP_STATE,

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

pub(crate) static mut WIFI: Option<WiFi<'_, '_>> = None;

impl<'a, 'b> Drop for WiFi<'a, 'b> {
    fn drop(&mut self) {
        self.stop().unwrap();
        unsafe { WIFI = None };
    }
}

impl<'a, 'b> WiFi<'a, 'b> {
    pub(crate) fn get() -> &'a mut WiFi<'a, 'b> {
        unsafe { core::mem::transmute::<_, Option<&mut WiFi<'a, 'b>>>(WIFI.as_mut()).unwrap() }
    }

    pub(crate) fn get_timer_factory() -> &'a mut dyn crate::timer::TimerFactory<'b> {
        WiFi::get().timer_factory
    }

    pub fn new(
        clock_config: esp32_hal::clock_control::ClockControlConfig,
        timer_factory: &'a mut dyn crate::timer::TimerFactory<'b>,
    ) -> Result<&'a mut WiFi<'a, 'b>, Error> {
        let wifi = WiFi {
            _apb_lock: clock_config.lock_apb_frequency(),
            timer_factory: timer_factory,
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

            wprintln!("test1");
            let wifi_static = Some(core::mem::transmute::<_, WiFi<'static, 'static>>(wifi));
            WIFI = wifi_static;
            wprintln!("test2");

            if let Some(error) = Error::from(crate::binary::wifi::esp_wifi_init_internal(&config)) {
                return Err(error);
            }
            // enable all logging
            if let Some(error) = Error::from(crate::binary::wifi::esp_wifi_internal_set_log_mod(
                crate::binary::wifi::wifi_log_module_t::WIFI_LOG_MODULE_ALL,
                0,
                true,
            )) {
                return Err(error);
            }

            // initialize the wpa
            if let Some(error) = Error::from(crate::binary::wpa::esp_wifi_register_wpa_cb_internal(unsafe { core::mem::uninitialized() })) {
                return Err(error);
            }

            Ok(core::mem::transmute::<_, Option<&mut WiFi<'a, 'b>>>(WIFI.as_mut()).unwrap())
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

    pub fn scan(&self) -> Result<u16, Error> {
        if let Some(error) =
            Error::from(unsafe { crate::binary::wifi::esp_wifi_scan_start(0 as *const _, true) })
        {
            return Err(error);
        }

        let mut count: u16 = 0;
        if let Some(error) =
            Error::from(unsafe { crate::binary::wifi::esp_wifi_scan_get_ap_num(&mut count) })
        {
            return Err(error);
        }

        /*        Error::convert(unsafe {
            crate::binary::wifi::esp_wifi_scan_get_ap_records(&mut count, true)
        });*/
        //        ESP_ERROR_CHECK(esp_wifi_scan_start(NULL, true));
        //        ESP_ERROR_CHECK(esp_wifi_scan_get_ap_num(&ap_count));
        //        ESP_ERROR_CHECK(esp_wifi_scan_get_ap_records(&number, ap_info));
        Ok(count)
    }

    pub fn stop(&self) -> Result<(), Error> {
        Error::convert(unsafe { crate::binary::wifi::esp_wifi_stop() })
    }
}
