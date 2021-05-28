#!/usr/bin/sh

i=0;
while :
do
  echo $i
  curl localhost:8000/broadcast/$i
  ((i++))
  sleep 1
done
