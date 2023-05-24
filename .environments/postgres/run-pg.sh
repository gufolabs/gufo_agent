#!/bin/sh
createdb db1
createdb db2
createdb db3
/opt/ga/gufo-agent --config=/etc/gufo-agent.yml --test