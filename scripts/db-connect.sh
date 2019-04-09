#!/bin/sh

. .env

dbtype=$1

if [ $dbtype = "read" ]
then
  connection=$DATABASE_URL_READ
else
  connection=$DATABASE_URL_EVENT_STORE
fi

psql "$connection"