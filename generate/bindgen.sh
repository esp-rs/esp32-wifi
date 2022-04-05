#!/bin/bash

if [ -z "$IDF_PATH" ]
then
    IDF_PATH=~/esp/v4.1
fi
BASE=$IDF_PATH/components
LIBS=$(find $BASE -name "include" |grep -v esp32s2| xargs -I{} echo -n "-I {} ")

TOOLS=$(realpath $(dirname $(which xtensa-esp32-elf-gcc))/..)
TOOL_INCLUDE=$TOOLS/xtensa-esp32-elf/include

BINDGEN_OPTS="--no-layout-tests --use-core --size_t-is-usize --no-prepend-enum-name --ctypes-prefix cty --raw-line #![allow(non_camel_case_types)] --raw-line #![allow(broken_intra_doc_links)] --default-enum-style rust"
BINDGEN_CLANG_OPTS="-- -D__GLIBC_USE(x)=0 -DSSIZE_MAX -I ./generate -I $TOOL_INCLUDE -I $BASE/freertos/include $LIBS -I $BASE/lwip/include/apps/sntp/ -I $BASE/lwip/include/apps"

echo "Generating phy.rs"
bindgen $BASE/esp_wifi/include/phy.h -o src/binary/phy.rs \
    --whitelist-function "phy.*" --whitelist-function "coex.*" --whitelist-function ".*_phy" \
    $BINDGEN_OPTS $BINDGEN_CLANG_OPTS

WIFI_FUNCTIONS=$(xtensa-esp32-elf-objdump -t esp32-wifi-lib/esp32/libnet80211.a |grep esp_wifi|grep -v internal|grep "F .text"| awk '{printf("%s|",$6)}')

echo "Generating wifi.rs"
bindgen generate/wifi_wrapper.h  -o src/binary/wifi.rs \
    --with-derive-default \
    --whitelist-function $WIFI_FUNCTIONS"(.*wifi.*internal.*)" \
    $BINDGEN_OPTS $BINDGEN_CLANG_OPTS

echo "Generating coexist.rs"
bindgen generate/coexist_wrapper.h  -o src/binary/coexist.rs \
    --whitelist-function "coex.*" --whitelist-function "esp_coex.*" \
    $BINDGEN_OPTS $BINDGEN_CLANG_OPTS
