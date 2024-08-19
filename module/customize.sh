#!/system/bin/sh
#
# Copyright 2024 shadow3aaa@gitbub.com
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
#  You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
LOCALE=$(getprop persist.sys.locale)

local_print() {
	if [ $LOCALE = zh-CN ]; then
		ui_print "$1"
	else
		ui_print "$2"
	fi
}

mkdir "/data/adb/cpulimiter_rs"
if [ ! -f "/data/adb/cpulimiter_rs/config.toml" ]; then
	echo "powersave = 60000
balance = 75000
performance = 95000
fast = 85000" >"/data/adb/cpulimiter_rs/config.toml"
fi

set_perm "$MODPATH/cpu-limiter-rs" 0 0 0755

if [ "$(getprop fas-rs-installed)" = "" ]; then
	sh $MODPATH/vtools/init_vtools.sh $(realpath $MODPATH/module.prop)
fi
