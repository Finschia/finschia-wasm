#!/bin/bash

TAG=2.0.1
TEST_DOCKER_IMAGE=finschia/finschianode:${TAG}
docker pull finschia/finschianode:${TAG}

# copy init_single.sh from Finschia/finschia/init_single.sh
export FNSAD="docker run -i --rm -p 26656:26656 -p 26657:26657 -v ${HOME}/.finschia:/root/.finschia ${TEST_DOCKER_IMAGE} fnsad"

init_single=$(mktemp); curl "https://raw.githubusercontent.com/Finschia/finschia/v${TAG}/init_single.sh" > $init_single

bash -xe $init_single

container_id=$(docker run -d -p 26656:26656 -p 26657:26657 -v ${HOME}/.finschia:/root/.finschia ${TEST_DOCKER_IMAGE} fnsad start)

docker exec ${container_id} apk add --no-cache jq bash curl && \
docker cp ./integration_test_contracts.sh ${container_id}:/root/integration_test_contracts.sh && \
docker cp ./../artifacts ${container_id}:/root/artifacts
docker exec ${container_id} /root/integration_test_contracts.sh

docker stop ${container_id}

# remove tempolary file
rm $init_single

