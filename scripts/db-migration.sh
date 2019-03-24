#!/bin/sh

. .env

dbtype=$1
cmd=$2
cmd2=$3

if [ $dbtype = "read" ]
then
  connection=$DATABASE_URL_READ
else
  connection=$DATABASE_URL_WRITE
fi

diesel migration $cmd $cmd2 \
  --database-url $connection \
  --migration-dir migrations/$dbtype \
  --config-file config/diesel-$dbtype.toml

echo "migration $cmd, db: $dbtype - Ok!"