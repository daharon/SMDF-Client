# SMDF Client Docker Image

## Usage

### Environment Variables

#### Required
- `ENVIRONMENT` - Corresponds to `--environment` parameter.
- `TAGS` - Corresponds to `--tags` parameter.

#### Optional
- `LOG_LEVEL` - Defaults to `INFO`.
- `AUTO_DEREGISTER` - Corresponds to `--auto-deregister` parameter.
- `REGION` - Defaults to `AWS_REGION` provided by ECS.
- `NAME` - Override the client's name.

## Build
```
$ docker build --file ./package/docker/Dockerfile --tag smdf-client .
```
