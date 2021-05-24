#!/usr/bin/sh

i=0;
while :
do
  echo $i | tee /home/michael/Dev/Stellarust/html_dummy/sample_file.out

  ((i++))
  sleep 1
done
