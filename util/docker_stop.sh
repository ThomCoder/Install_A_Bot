#!/bin/bash

docker stop installabot
# Uncomment to also remove the created docker image
sleep 1
docker rmi installabot