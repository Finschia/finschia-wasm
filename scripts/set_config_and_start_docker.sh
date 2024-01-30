#!/bin/sh

# Finschia app start
CONFIG_DIR=$(mktemp -d)
./finschia/setup.sh ${CONFIG_DIR}
./finschia/start.sh ${CONFIG_DIR}
