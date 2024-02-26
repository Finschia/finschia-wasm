#!/bin/bash

# Commands
DOCKER=${DOCKER:-docker}
JQ=${JQ:-jq}
VERBOSE=${VERBOSE:-false}

script_dir=$(realpath $(dirname $0))

# Build collection.wasm if not exitst
echo "##### Build collection.wasm if there is not #####" >&2
env DOCKER=${DOCKER} ${script_dir}/build_collection.sh >&2

# Get the latest Finschia tag
TAG=$(curl -s "https://api.github.com/repos/Finschia/finschia/tags" | ${JQ} -r '.[0].name')

# Remove v from tag prefix
# ex. v1.0.0 -> 1.0.0
TAG_NUM=$(echo "$TAG" | cut -c 2-)

echo "##### Pull finschianode docker #####" >&2

TEST_DOCKER_IMAGE=finschia/finschianode:${TAG_NUM}
${DOCKER} pull ${TEST_DOCKER_IMAGE} >&2

echo "##### Init node for test #####" >&2

# Copy init_single.sh from Finschia/finschia/init_single.sh
init_single=$(mktemp)
curl -s "https://raw.githubusercontent.com/Finschia/finschia/${TAG}/init_single.sh" -o $init_single

# run Finschia/finschia/init_single.sh
env FNSAD="docker run -i --rm -v ${HOME}/.finschia:/root/.finschia ${TEST_DOCKER_IMAGE} fnsad" bash ${init_single} >&2

echo "##### Start node #####" >&2

container_id=$(${DOCKER} run -d -v ${HOME}/.finschia:/root/.finschia ${TEST_DOCKER_IMAGE} fnsad start)

echo "##### Install tools to container #####" >&2

${DOCKER} exec ${container_id} apk add --no-cache jq bash curl >&2 &&

echo "##### Install scripts and contracts to container #####" >&2 &&
${DOCKER} cp ${script_dir}/integration_test_contracts.sh ${container_id}:/root/integration_test_contracts.sh >&2 &&
docker cp -L ${script_dir}/../artifacts/collection.wasm ${container_id}:/root/ >&2 &&
echo "##### Start tests #####" >&2 &&
${DOCKER} exec ${container_id} env VERBOSE=${VERBOSE} /root/integration_test_contracts.sh

echo "##### Stop docker #####" >&2
${DOCKER} stop ${container_id} >&2

# Remove the temporary file `Finschia/init_single.sh`
rm $init_single
