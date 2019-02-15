# Monitoring Client

Client side of the Monitoring framework.   
```
monitoring-client 0.1.0
Monitoring client.

USAGE:
    monitoring-client [OPTIONS] --environment <ENV> --name <NAME> --region <REGION> --tags <TAG,TAG,...>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --concurrency <INT>        The maximum number of checks to run concurrently (1-256).
                                   Note:  Currently not used. [default: 10]
    -e, --environment <ENV>        The environment this monitoring client is running under.
                                   Not used if `--reg-parameter` is set.
                                   Parameter store path /<env>/monitoring/registration will be used.
    -l, --log-level <LEVEL>        Log level (TRACE, DEBUG, ERROR, WARN, INFO). [default: info]
    -n, --name <NAME>              The client-name to be registered with the monitoring backend.
    -p, --reg-parameter <PATH>     Explicitly set the parameter store name to use.
                                   Overrides `--environment`.
                                   eg. /dev/test/value
    -r, --region <REGION>          AWS region.
    -t, --tags <TAG,TAG,...>...    The check tags to run on this client.
```

## Packaging

### RPM

#### Mac
```
$ brew install rpm
```

#### Redhat
```
$ sudo yum install rpmbuild
```

#### General
```
$ cargo install cargo-rpm
$ cargo rpm build
```

## TODO
- [ ] Concurrency limit.
    - See `--concurrency` CLI parameter.
- [x] Proper logging.
- [ ] Package as Docker image.
- [ ] Implement check timeout.
- [ ] Package as RPM.
- [ ] Package as DEB.
