#!/bin/sh

echo "hello run_finschia.sh"
#CONFIG_DIR=${CONFIG_DIR:-${SCRIPT_DIR}/.finschia}


set -o errexit -o nounset -o pipefail
cp -R "/tmp/.finschia" /root/.finschia
mkdir -p /root/log
fnsad start --rpc.laddr tcp://0.0.0.0:26658 --trace
