#!/usr/bin/env bash
#
# Required environment variables:
#   TAGS
#   ENVIRONMENT
# Optional:
#   REGION
#   LOG_LEVEL
#   NAME

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

: "${NAME:?}"
: "${ENVIRONMENT:?}"

if [[ -n $REG_PARAMETER ]] ; then
    OPTS="${OPTS} --reg-parameter ${REG_PARAMETER}"
fi

echo "Region:  ${REGION}"
echo "Log level:  ${LOG_LEVEL}"
echo "Environment:  ${ENVIRONMENT}"
echo "Name:  ${NAME}"
echo "Tags:  ${TAGS}"

CMD="$1 ${OPTS} --region ${REGION} --log-level ${LOG_LEVEL} --environment ${ENVIRONMENT} --name ${NAME} --tags ${TAGS}"
echo "Command:  ${CMD}"

exec ${CMD}
