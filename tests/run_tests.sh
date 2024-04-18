#!/bin/bash
# Get the port environment variable

if [[ -z "${PORT}" ]]; then
  poetry run godata server start
else
  poetry run godata server start --port $PORT
fi

poetry run pytest -W ignore::ResourceWarning
poetry run godata server stop

cp $HOME/godata/logs/* /home/logs
