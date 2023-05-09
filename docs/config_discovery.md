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

## Config Discovery Scripts

Gufo Agent allows using third-party scripts to generate default config. 
The agent looks at:

* `GA_CONFIG_SCRIPTS` environment variable, which contains colon-separated paths (Just like the PATH variable)
* `--config-scripts` command line option, which may be used multiple times.
  
Then, the agent runs each script found and parses its output in YAML format. Each script may emit a config for one or more collector instances. Then the agent checks and collect all configs together.

### Examples

Script, generating a single instance of collector:

``` txt title="examples/scripts/config/gufolabs.sh" linenums="1"
--8<-- "examples/scripts/config/gufolabs.sh"
```

Script, generating a multiple instances of collector:

``` txt title="examples/scripts/config/twamp.sh" linenums="1"
--8<-- "examples/scripts/config/twamp.sh"
```

The overall result:
```
$ gufo-agent --config-discovery --config-discovery-opts=-builtins --config-scripts=examples/scripts/config
...
collectors:
- id: GufoLabs dns
  type: dns
  interval: 15
  disabled: false
  query: gufolabs.com
  n: 10
- id: Twamp Reflector
  type: twamp_reflector
  interval: 10
  disabled: true
- id: Twamp Sender
  type: twamp_sender
  interval: 10
  disabled: true
  reflector: 127.0.0.1
  n_packets: 100
  model: g711
```

