#!/bin/bash

if [ -z "$IDF_PATH" ]
then
    IDF_PATH=~/esp/v4.1
fi
BASE=$IDF_PATH/components
LIBS=$(find $BASE -name "include" |grep -v esp32s2| xargs -I{} echo -n "-I {} ")

TOOLS=$(realpath $(dirname $(which xtensa-esp32-elf-gcc))/..)
TOOL_INCLUDE=$TOOLS/xtensa-esp32-elf/include

BINDGEN_OPTS="--no-layout-tests --use-core --size_t-is-usize --no-prepend-enum-name --ctypes-prefix cty --raw-line #![allow(non_camel_case_types)] --raw-line #![allow(intra_doc_link_resolution_failure)]"
BINDGEN_CLANG_OPTS="-- -D__GLIBC_USE(x)=0 -DSSIZE_MAX -I $IDF_PATH/examples/wifi/getting_started/station/build/config -I $TOOL_INCLUDE -I $BASE/freertos/include $LIBS -I $BASE/lwip/include/apps/sntp/ -I $BASE/lwip/include/apps"

echo "Generating phy.rs"
bindgen $BASE/esp_wifi/include/phy.h -o src/binary/phy.rs \
    --whitelist-function "phy.*" --whitelist-function "coex.*" --whitelist-function ".*_phy" \
    $BINDGEN_OPTS $BINDGEN_CLANG_OPTS

echo "Generating wifi.rs"
bindgen $BASE/esp_wifi/include/esp_private/wifi.h  -o src/binary/wifi.rs \
    --whitelist-function ".*wifi.*internal.*" \
    $BINDGEN_OPTS $BINDGEN_CLANG_OPTS

echo "Generating coexist.rs"
bindgen generate/coexist_wrapper.h  -o src/binary/coexist.rs \
    --whitelist-function "coex.*" --whitelist-function "esp_coex.*" \
    $BINDGEN_OPTS $BINDGEN_CLANG_OPTS
