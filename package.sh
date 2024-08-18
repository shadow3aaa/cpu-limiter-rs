#!/bin/bash
#
# Copyright 2023 shadow3aaa@gitbub.com
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
SHDIR="$(dirname $(readlink -f "$0"))"

cd $SHDIR
rm -rf output/.temp
mkdir -p output/.temp
cp -r module/* output/.temp/
cp main.lua output/.temp/

cd output/.temp
package="$(awk -F= '/name/ {print $2}' module.prop).zip"
rm -f "../$package"
zip -9 -rq "../$package" .
echo Module packaged: $(realpath "../$package")
