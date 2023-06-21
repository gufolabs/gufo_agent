# Gufo Agent Configuration

Gufo Agent reads configuration from YAML or JSO file.

## Global Configuration

### $version

Config format version. Must be set to `1.0`

### $type

Config type. Must be set to `zeroconf`.

## agent

Agent configuration

Example:

=== "YAML"

    ``` yaml
    agent:
        host: test
    ```

=== "JSON"

    ``` json
    {
        "agent": {
            "host": "test"
        }
    }
    ```

### host

Set hostname manually.

Example:

=== "YAML"

    ``` yaml
    host: test
    ```

=== "JSON"

    ``` json
    host: "test"
    ```

### labels

Optional agent-level labels. Agent-level labels are appended to all collected metrics.
Labels are set as key-value pairs.

Example:

=== "YAML"

    ``` yaml
    labels:
        dc: south
        zone: europe
    ```

=== "JSON"

    ``` json
    "labels": {
        "dc": "south",
        "zone": "europe"
    }
    ```

### defaults

Collectors' default configuration

Example:

=== "YAML"

    ``` yaml
    defaults:
        interval: 10
    ```

=== "JSON"

    ``` json
    "defaults": {
        "interval": 10
    }
    ```


#### interval

Default collectors' repetition interval in seconds.

## sender

Metrics sender configuration.

Example:

=== "YAML"

    ``` yaml
    sender:
        type: openmetrics
        mode: pull
        listen: "0.0.0.0:3000"
    ```

=== "JSON"

    ``` json
    {
        "sender": {
            "type": "openmetrics",
            "mode": "pull",
            "listen": "0.0.0.0:3000"
        }
    }
    ```

### type

Sender's type. Must be `openmetrics`.

### mode

Sender's mode of operation. Must be `pull`.

### listen

Address and port to listen the HTTP metrics endpoint. Don't run HTTP
endpoint when ommmited.

Example:

=== "YAML"

    ``` yaml
    listen: "0.0.0.0:3000"
    ```

=== "JSON"

    ``` json
    "listen": "0.0.0.0:3000"
    ```

### listen_tls

Address and port to listen the HTTPS metrics endpoint. Don't run HTTPS
endpoint when ommmited. When `listen_tls` is set, [cert_path](#cert_path)
and [key_path](#key_path) configuration parameters must be set

Example:

=== "YAML"

    ``` yaml
    listen_tls: "0.0.0.0:3001"
    cert_path: /etc/gufo-agent/tls/agent.crt
    key_path: /etc/gufo-agent/tls/agent.key
    ```

=== "JSON"

    ``` json
    "listen_tls": "0.0.0.0:3000",
    "cert_path": "/etc/gufo-agent/tls/agent.crt",
    "key_path": "/etc/gufo-agent/tls/agent.key"
    ```

### cert_path

A path to the TLS certificate (public key) for TLS endpoint.
Must be set only with [listen_tls](#listen_tls) parameter.

Example:

=== "YAML"

    ``` yaml
    listen_tls: "0.0.0.0:3001"
    cert_path: /etc/gufo-agent/tls/agent.crt
    key_path: /etc/gufo-agent/tls/agent.key
    ```

=== "JSON"

    ``` json
    "listen_tls": "0.0.0.0:3000",
    "cert_path": "/etc/gufo-agent/tls/agent.crt",
    "key_path": "/etc/gufo-agent/tls/agent.key"
    ```

### key_path

A path to the TLS private key for TLS endpoint.
Must be set only with [listen_tls](#listen_tls) parameter.

Example:

=== "YAML"

    ``` yaml
    listen_tls: "0.0.0.0:3001"
    cert_path: /etc/gufo-agent/tls/agent.crt
    key_path: /etc/gufo-agent/tls/agent.key
    ```

=== "JSON"

    ``` json
    "listen_tls": "0.0.0.0:3001",
    "cert_path": "/etc/gufo-agent/tls/agent.crt",
    "key_path": "/etc/gufo-agent/tls/agent.key"
    ```

### client_auth_required_path

Path to the trust ancors for the client authentication. Only authenticated clients
are accepted. `client_auth_required_path` effectively enforces mTLS authentication.
Used in combination with [listen_tls](#listen_tls) and other HTTPS endpoint configuration.

Example:

=== "YAML"

    ``` yaml
    listen_tls: "0.0.0.0:3001"
    cert_path: /etc/gufo-agent/tls/agent.crt
    key_path: /etc/gufo-agent/tls/agent.key
    client_auth_requred_path: /etc/gufo-agent/tls/second_ca.crt
    ```

=== "JSON"

    ``` json
    "listen_tls": "0.0.0.0:3001",
    "cert_path": "/etc/gufo-agent/tls/agent.crt",
    "key_path": "/etc/gufo-agent/tls/agent.key",
    "client_auth_requred_path": "/etc/gufo-agent/tls/second_ca.crt"
    ```

### tls_redirect

When set to `true`, HTTP endpoint will perform redirect to the HTTPS one.
Must be used only when [listen](#listen) and [listen_tls](#listen_tls) parameters
are set.

Example:

=== "YAML"

    ``` yaml
    listen: "0.0.0.0:3000"
    listen_tls: "0.0.0.0:3001"
    tls_redirect: true
    ```

=== "JSON"

    ``` json
    "listen": "0.0.0.0:3000",
    "listen_tls": "0.0.0.0:3001",
    "tls_redirect": true,
    ```

## collectors

List of configured collectors. Each collector has a common configuration part
and may have additional collector options.
Refer to the [Collectors Reference](collectors/index.md) for an additional
options.

Example:

=== "YAML"

    ``` yaml
    collectors:
      - id: Filesystem
        type: fs
        interval: 10
        labels:
          project: P1
      - id: Memory
        type: memory
        disabled: true
        interval: 10
        labels:
          project: P2
    ```

=== "JSON"

    ``` json
    {
        "collectors": [
            {
                "id": "Filesystem",
                "type": "fs",
                "interval": 10,
                "labels": {
                    "project": "P1"
                }
            },
            {
                "id": "Memory",
                "type": "memory",
                "disabled": true,
                "interval": 10,
                "labels": {
                    "project": "P2"
                } 
            }
        ]
    }   
    ```

### id

The unique collector instance's id. Must be unique within the agent's configuration.

### type

The collector type. Refer to the [Collectors Reference](collectors/index.md)
for the details.

### interval

Collector's running interval, in seconds.

### labels

Optional collector instance labels. These labels are appended to all metrics,
generated by instance.

### disabled

Setting to `true` allows to switch off the collector instance without removing
the config.

## Example

=== "YAML"

    ``` yaml
    $version: "1.0"
    $type: "zeroconf"
    agent:
        host: test
        labels:
            dc: south
            zone: europe
    sender:
        type: openmetrics
        mode: pull
        listen: "0.0.0.0:3000"
    collectors:
      - id: Filesystem
        type: fs
        interval: 10
        labels:
          project: P1
      - id: Memory
        type: memory
        disabled: true
        interval: 10
        labels:
          project: P2
    ```

=== "JSON"

    ``` json
    {
        "$version": "1.0",
        "$type": "zeroconf",
        "agent": {
            "host": "test",
            "labels": {
                "dc": "south",
                "zone": "europe"
            }
        },
        "sender": {
            "type": "openmetrics",
            "method": "pull",
            "listen": "0.0.0.0:3000"
        },
        "collectors": [
            {
                "id": "Filesystem",
                "type": "fs",
                "interval": 10,
                "labels": {
                    "project": "P1"
                }
            },
            {
                "id": "Memory",
                "type": "memory",
                "disabled": true,
                "interval": 10,
                "labels": {
                    "project": "P2"
                } 
            }
        ]
    }
    ```