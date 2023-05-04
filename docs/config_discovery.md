# Config Discovery

Config discovery is the process of generating the config according to the host
and its configured services.

Usually, it started during the initial agent setup by command:

``` shell
gufo-agent --config-discovery
```

The resulting config will be dumped to the stdout. To save config file use shell redirection:

``` shell
gufo-agent --config-discovery > config.yml
```

The config discovery is the collaborative  process and includes phases:

* Built-in: Each built-in collector decides does it is worth
  contributing to the config. 
* Custom scripts: Custom scripts are provided in specified locations to analyze
  the system's environment and to provide appropriate parts of the config.

To disable parts of discovery use the `--config-discovery-opts` option. i.e. to disable checks for `uptime` and `sockets` run:

```
gufo-agent --config-discovery --config-discovery-opts=-uptime,-sockets
```

To disable all built-in collectors except the explicitly set (`cpu` and `memory` in our case):

```
gufo-agent --config-discovery --config-discovery-opts=-builtins,+cpu,+memory
```