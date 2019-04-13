#!/bin/sh

echo $ENV

if [ $ENV = "development" ]
then
  . config/.env-development
elif [ $ENV = "staging" ]
then
  . config/.env-staging
else
  . config/.env-development
fi

dbtype=$1
cmd=$2
cmd2=$3

if [ $dbtype = "read" ]
then
  connection=$DATABASE_URL_READ
else
  connection=$DATABASE_URL_EVENT_STORE
fi

diesel migration $cmd $cmd2 \
  --database-url $connection \
  --migration-dir migrations/$dbtype \
  --config-file config/diesel-$dbtype.toml

echo "migration $cmd, db: $dbtype - Ok!"