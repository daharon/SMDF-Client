#!/usr/bin/env bash

OPTS=

if [[ -n $REG_PARAMETER ]] ; then
    OPTS="${OPTS} --reg-parameter ${REG_PARAMETER}"
fi
if [[ -z $REGION ]] ; then
    REGION="${AWS_REGION}"
fi
if [[ -z $LOG_LEVEL ]] ; then
    LOG_LEVEL="INFO"
fi

echo "Region:  ${REGION}"
echo "Log level:  ${LOG_LEVEL}"
echo "Environment:  ${ENVIRONMENT}"
echo "Name:  ${NAME}"
echo "Tags:  ${TAGS}"

CMD="$1 ${OPTS} --region ${REGION} --log-level ${LOG_LEVEL} --environment ${ENVIRONMENT} --name ${NAME} --tags ${TAGS}"
echo "Command:  ${CMD}"

${CMD}
