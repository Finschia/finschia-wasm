#!/bin/bash

script_dir=$(realpath $(dirname $0))

# Build collection.wasm if not exitst
. ${script_dir}/build_collection.sh

# Get the latest Finschia tag
TAG=$(curl -s "https://api.github.com/repos/Finschia/finschia/tags" | jq -r '.[0].name')

# Remove v from tag prefix
# ex. v1.0.0 -> 1.0.0
TAG_NUM=$(echo "$TAG" | cut -c 2-)

echo "##### Pull finschianode docker #####" >&2

TEST_DOCKER_IMAGE=finschia/finschianode:${TAG_NUM}
docker pull ${TEST_DOCKER_IMAGE}

echo "##### Init node for test #####" >&2

# Copy init_single.sh from Finschia/finschia/init_single.sh
init_single=$(mktemp)
curl -s "https://raw.githubusercontent.com/Finschia/finschia/${TAG}/init_single.sh" -o $init_single

# run Finschia/finschia/init_single.sh
env FNSAD="docker run -i --rm -v ${HOME}/.finschia:/root/.finschia ${TEST_DOCKER_IMAGE} fnsad" bash ${init_single}

echo "##### Start node #####" >&2

container_id=$(docker run -d -v ${HOME}/.finschia:/root/.finschia ${TEST_DOCKER_IMAGE} fnsad start)

echo "##### Install tools to container #####" >&2

docker exec ${container_id} apk add --no-cache jq bash curl &&

echo "##### Install scripts and contracts to container #####" >&2 &&
docker cp ${script_dir}/integration_test_contracts.sh ${container_id}:/root/integration_test_contracts.sh &&
docker cp ${script_dir}/../artifacts/collection.wasm ${container_id}:/root/ &&
echo "##### Start tests #####" >&2 &&
docker exec ${container_id} /root/integration_test_contracts.sh

echo "##### Tests Succeed #####" >&2
echo "##### Stop docker #####" >&2
docker stop ${container_id}

# Remove the temporary file `Finschia/init_single.sh`
rm $init_single
