# SMDF Client

Client side of the Serverless Monitoring Development Framework.   
```
smdf-client 0.1.0
SMDF client.

USAGE:
    smdf-client [OPTIONS] --environment <ENV> --name <NAME> --region <REGION> --tags <TAG,TAG,...>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --concurrency <INT>        The maximum number of checks to run concurrently (1-256).
                                   Note:  Currently not used. [default: 10]
    -e, --environment <ENV>        The environment this monitoring client is running under.
                                   Not used if `--reg-parameter` is set.
                                   Parameter store path /<env>/smdf/registration will be used.
    -l, --log-level <LEVEL>        Log level (TRACE, DEBUG, ERROR, WARN, INFO). [default: info]
    -n, --name <NAME>              The client-name to be registered with the monitoring backend.
    -p, --reg-parameter <PATH>     Explicitly set the parameter store name to use.
                                   Overrides `--environment`.
                                   eg. /dev/test/value
    -r, --region <REGION>          AWS region.
    -t, --tags <TAG,TAG,...>...    The check tags to run on this client.
```

## Packaging

#### CentOS
```
$ docker build --file package/el/Dockerfile --tag smdf-client-centos-7 .
$ docker run --volume .:/tmp/src smdf-client-centos-7
```
The RPM files will be found in the project's `target/rpmbuild/{RPM,SRPM}/x86_64` directory.

#### General
```
$ cargo install cargo-rpm
$ cargo rpm build
```

## TODO
- [ ] Auto-deactivate client on process termination.
    - Only on `SIGINT` and `SIGTERM`.
- [ ] Timestamp to three decimal places.
    - Receiving check results with timestamps like the following:  `2019-03-16T14:53:25.470766743Z`
- [ ] Concurrency limit.
    - See `--concurrency` CLI parameter.
- [x] Proper logging.
- [x] Package as Docker image.
- [x] Implement check timeout.
- [ ] Package as RPM.
- [ ] Package as DEB.
