 # modbus_tcp collector

`modbus_tcp` collector performs Modbus TCP requests to collect performance data.

## Configuration

| Parameter                | Type                | Default   | Description                                                                   |
| ------------------------ | ------------------- | --------- | ----------------------------------------------------------------------------- |
| `id`                     | String              |           | Collector's ID. Must be unique per agent instance.                            |
| `type`                   | String              |           | Must be `modbus_tcp`                                                          |
| `interval`               | Integer             |           | Repetition interval in seconds                                                |
| `labels`                 | Object              |           | Additional collector-level labels                                             |
| `address`                | String              |           | IP address of Modbus TCP server                                               |
| `port`                   | Integer             | 502       | Port of Modbus TCP server                                                     |
| `timeout_ms`             | Integer             | 5000      | Request timeout, ms.                                                          |
| `items`                  | Array {{ complex }} |           | Metrics to collect as a list of items                                         |
| {{ tab }}`name`          | String              |           | Metric name, as to be exposed                                                 |
| {{ tab }}`help`          | String              |           | Short help to be exposed along with metric                                    |
| {{ tab }}`labels`        | Object              |           | Metric labels                                                                 |
| {{ tab }}`register`      | Integer             |           | Starting register of modbus request, zero-based                               |
| {{ tab }}`register_type` | String              | `holding` | Modbus request type. Either `holding`, `input` or `coil`                      |
| {{ tab }}`format`        | String              |           | Expected response format. See [Response format](#response-format) for details |
| {{ tab }}`slave`         | Integer             | 255       | Optional slave id, see note below.                                            |

!!! warning "Check address notation"

    Take note the starting register address is zero-based, while vendors
    can document the registers starting from 1. Refer to the vendor documentation
    and subtract 1 when necessary.

!!! note "On Slave ID"

    Modbus TCP specification insists on using slave id of 255 for TCP connections.
    Meanwhile some implmenetations await broadcast id (0). Modbust TCP-to-RTU
    proxies also may expect explicit slave id to process the request.

Config Example:

``` yaml
- id: Modbus
  type: modbus_tcp
  interval: 10
  address: 192.168.0.2
  port: 502
  items:
    - name: dc_temp
      help: Temperature in celsius
      labels:
        side: east
      register: 12
      format: f32_be
    - name: dc_temp
      help: Temperature in celsius
      labels:
        side: west
      register: 14
      format: f32_be
```

### Response Format

Modbus' response is as an array of 16-bit integers. Actual data encoding
should be set as `format` parameter. Some encodings may require reading
2 or 4 adjacent registers.

| Format   | Count | Description                                  |
| -------- | ----: | -------------------------------------------- |
| `i16_be` |     1 | 16-bit signed integer, big-endian.           |
| `u16_be` |     1 | 16-bit unsigned integer, big-endian.         |
| `i32_be` |     2 | 32-bit signed integer, big-endian            |
| `i32_le` |     2 | 32-bit signed integer, low-endian            |
| `i32_bs` |     2 | 32-bit signed integer, big-endian, swapped   |
| `i32_ls` |     2 | 32-bit signed integer, low-endian, swapped   |
| `u32_be` |     2 | 32-bit unsigned integer, big-endian          |
| `u32_le` |     2 | 32-bit unsigned integer, low-endian          |
| `u32_bs` |     2 | 32-bit unsigned integer, big-endian, swapped |
| `u32_ls` |     2 | 32-bit unsigned integer, low-endian, swapped |
| `f32_be` |     2 | 32-bit floating point, big-endian            |
| `f32_le` |     2 | 32-bit floating point, low-endian            |
| `f32_bs` |     2 | 32-bit floating point, big-endian, swapped   |
| `f32_ls` |     2 | 32-bit floating point, low-endian, swapped   |

The 32-bit integer `0x01020304` stored as a sequence of 4 octets. 4 different
approaches widely used between modbus devices:

| Format                   |    1 |    2 |    3 |    4 |
| ------------------------ | ---: | ---: | ---: | ---: |
| Big-endian (be)          |   01 |   02 |   03 |   04 |
| Low-endian (le)          |   04 |   03 |   02 |   01 |
| Big-endian, swapped (bs) |   02 |   01 |   04 |   03 |
| Low-endian, swapped (ls) |   03 |   04 |   01 |   02 |

## Collected Metrics

`modbus_tcp` doesn't impose the specific format of the metrics. The generated
metrics are fully configurable.

## Labels

Metric labels depends on configuration.

## Sample Output