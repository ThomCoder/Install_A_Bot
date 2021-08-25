FROM ubuntu:20.04
ENV LC_CTYPE C.UTF-8
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update -y
RUN apt-get install -y sudo cargo
# RUN useradd -ms /bin/bash user
RUN useradd -m user && echo "user:user" | chpasswd && adduser user sudo
USER user
WORKDIR /home/user