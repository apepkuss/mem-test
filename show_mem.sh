#!/bin/bash

while true
do
    sleep 3
    date
    ps -p $(pidof mem-test) -o %mem,rss >> mem-stats.txt
done