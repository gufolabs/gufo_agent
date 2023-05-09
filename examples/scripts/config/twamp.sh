#!/bin/sh
# Configure local twamp sender and reflector
cat << __EOF__
- id: Twamp Reflector
  type: twamp_reflector
  disabled: true
  interval: 10
- id: Twamp Sender
  type: twamp_sender
  disabled: true
  interval: 10
  reflector: 127.0.0.1
  n_packets: 100
  model: g711
__EOF__