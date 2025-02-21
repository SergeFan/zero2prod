#!/usr/bin/env bash
set -x
set -eo pipefail

# Launch Redis using Docker
CONTAINER_NAME="newsletter_redis"

# Stop and delete last used container
if [[ $(! docker container ls -a | grep "${CONTAINER_NAME}") ]]
then
  docker container stop ${CONTAINER_NAME}
  docker container rm ${CONTAINER_NAME}
fi

docker run -p "6379:6379" -d --name $CONTAINER_NAME redis:7
>&2 echo "Redis is ready to go!"
