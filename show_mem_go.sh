#!/bin/bash

while true
do
    sleep 3
    date
    ps -p $(pidof hello) -o %mem,rss >> mem-stats-go.txt
done