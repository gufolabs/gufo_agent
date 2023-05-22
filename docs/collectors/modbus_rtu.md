 # modbus_tcp collector

`modbus_tcp` collector performs Modbus RTU requests to collect performance data.

## Configuration

| Parameter                | Type                | Default                   | Description                                                                   |
| ------------------------ | ------------------- | ------------------------- | ----------------------------------------------------------------------------- |
| `id`                     | String              |                           | Collector's ID. Must be unique per agent instance.                            |
| `type`                   | String              |                           | Must be `modbus_rtu`                                                          |
| `interval`               | Integer             | `agent.defaults.interval` | Repetition interval in seconds                                                |
| `labels`                 | Object              |                           | Additional collector-level labels                                             |
| `timeout_ms`             | Integer             | 5000                      | Request timeout, ms.                                                          |
| `default_serial_path`    | String              |                           | Default path to the serial port device (i.e. `/dev/ttyS1`)                    |
| `default_slave`          | Integer             |                           | Default modbus RTU slave id                                                   |
| `default_baud_rate`      | Integer             |                           | Default serial port speed                                                     |
| `default_data_bits`      | Integer             |                           | Default serial port data bits: 5, 6, 7 or 8                                   |
| `default_parity`         | String              | `none`                    | Default serial port parity, either `none`, `even` or `odd`                    |
| `default_stop_bits`      | Integer             |                           | Default serial port stop bits, either `1` or `2`                              |
| `items`                  | Array {{ complex }} |                           | Metrics to collect as a list of items                                         |
| {{ tab }}`name`          | String              |                           | Metric name, as to be exposed                                                 |
| {{ tab }}`help`          | String              |                           | Short help to be exposed along with metric                                    |
| {{ tab }}`labels`        | Object              |                           | Metric labels                                                                 |
| {{ tab }}`serial_path`   | String              |                           | Path to the serial port device (i.e. `/dev/ttyS1`). Use defaults when not set |
| {{ tab }}`slave`         | Integer             |                           | Modbus RTU slave id. Use defaults when not set                                |
| {{ tab }}`baud_rate`     | Integer             |                           | Serial port speed. Use defaults when not set                                  |
| {{ tab }}`data_bits`     | Integer             |                           | Serial port data bits: 5, 6, 7 or 8. Use defaults when not set                |
| {{ tab }}`parity`        | String              | `none`                    | Serial port parity, either `none`, `even` or `odd`. Use defaults when not set |
| {{ tab }}`stop_bits`     | Integer             |                           | Serial port stop bits, either `1` or `2`. Use defaults when not set           |
| {{ tab }}`register`      | Integer             |                           | Starting register of modbus request, zero-based                               |
| {{ tab }}`register_type` | String              | `holding`                 | Modbus request type. Either `holding`, `input` or `coil`                      |
| {{ tab }}`format`        | String              |                           | Expected response format. See [Response format](#response-format) for details |
| {{ tab }}`slave`         | Integer             | 255                       | Optional slave id, see note below.                                            |

!!! warning "Check address notation"

    Take note the starting register address is zero-based, while vendors
    can document the registers starting from 1. Refer to the vendor documentation
    and subtract 1 when necessary.

Config Example:

``` yaml
- id: Modbus
  type: modbus_rtu
  default_serial_path: /dev/ttyS1
  default_baud_rate: 9600
  default_bits: 8
  default_stop_bits: 1
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

`modbus_rtu` doesn't impose the specific format of the metrics. The generated
metrics are fully configurable.

## Labels

Metric labels depends on configuration.

## Sample Output