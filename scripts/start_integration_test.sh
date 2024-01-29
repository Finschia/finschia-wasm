#!/bin/sh

# Finschia app start
TMP_DIR=$(mktemp -d)
env TMP_DIR=${TMP_DIR} ./finschia/setup.sh
env TMP_DIR=${TMP_DIR} ./finschia/start.sh
