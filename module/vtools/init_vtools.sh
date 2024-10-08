#!/system/bin/sh
# Copyright 2024 shadow3aaa@gitbub.com
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

BASEDIR="$(dirname $(readlink -f "$0"))"

source $BASEDIR/gen_json.sh $1
echo "$json" >/data/powercfg.json

cp -af $BASEDIR/powercfg.sh /data/powercfg.sh
chmod 755 /data/powercfg.sh
