# Monitoring Client Docker Image

## Usage

### Environment Variables
- `LOG_LEVEL` - Defaults to `INFO`.
- `ENVIRONMENT` - Corresponds to `--environment` parameter. Required.
- `NAME` - Required.
- `REGION` - Defaults to `AWS_REGION` provided by ECS.
- `TAGS` - Corresponds to `--tags` parameter. Required.
- `REG_PARAMETER` - Corresponds to `--reg-parameter`. Optional.

### Checks


## Build
```
$ docker build --file ./package/docker/Dockerfile --tag monitoring-client .
```
