#!/usr/bin/env bash
#
# Required environment variables:
#   ENVIRONMENT
#   TAGS
# Optional:
#   REGION
#   LOG_LEVEL
#   NAME
#   AUTO_DEREGISTER

function ecs_task_id() {
    # https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-metadata-endpoint-v3.html
    TASK_ARN=$(
        curl --silent "${ECS_CONTAINER_METADATA_URI}/task" \
        | jq -r ' .TaskARN '
    )
    TASK_ID=${TASK_ARN##*/}
    echo "${TASK_ID}"
}

OPTS=
REGION=${REGION:-${AWS_REGION}}
LOG_LEVEL=${LOG_LEVEL:-INFO}
TASK_ID=$(ecs_task_id)
NAME=${NAME:-${TASK_ID}}

# Check that required environment variables are set.
# https://stackoverflow.com/a/307735
: "${NAME:?}"
: "${ENVIRONMENT:?}"
: "${TAGS:?}"

echo "Region:  ${REGION}"
echo "Log level:  ${LOG_LEVEL}"
echo "Environment:  ${ENVIRONMENT}"
echo "Name:  ${NAME}"
echo "Tags:  ${TAGS}"

if [[ -n $AUTO_DEREGISTER ]] ; then
    echo "Auto-deregister:  ${AUTO_DEREGISTER}"
    if [[ $AUTO_DEREGISTER = true ]] || [[ $AUTO_DEREGISTER = True ]]|| [[ $AUTO_DEREGISTER = 1 ]] ; then
        OPTS="${OPTS} --auto-deregister"
    fi
fi

CMD="$1 ${OPTS} --region ${REGION} --log-level ${LOG_LEVEL} --environment ${ENVIRONMENT} --name ${NAME} --tags ${TAGS}"
echo "Command:  ${CMD}"

exec ${CMD}
