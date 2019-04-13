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

if [ $dbtype = "read" ]
then
  connection=$DATABASE_URL_READ
else
  connection=$DATABASE_URL_EVENT_STORE
fi

psql "$connection"