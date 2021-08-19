#!/bin/bash

docker run --rm -v $(PWD)/..:/home/user --security-opt seccomp=unconfined -d --name installabot -i installabot

docker exec -it installabot /bin/bash

exit 0