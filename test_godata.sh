#!/bin/bash --login

docker build -t godata-test:latest . && docker run --env DATA_PATH=/home/data -v $GODATA_TEST_ROOT/test_data:/home/data -it godata-test:latest 
# Run this script from the root of the project to test the godata server and
# client. This docker daemon must be running.
# Assumes there is an environment variable $GODATA_TEST_ROOT that points to the 
# root of the test, for attachign the volume to the container.
