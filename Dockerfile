FROM rust:latest

RUN apt-get update \
    && apt-get install -y apt-transport-https gpg \
    && wget -qO - https://packages.adoptium.net/artifactory/api/gpg/key/public | gpg --dearmor | tee /etc/apt/trusted.gpg.d/adoptium.gpg > /dev/null \
    && echo "deb https://packages.adoptium.net/artifactory/deb $(awk -F= '/^VERSION_CODENAME/{print$2}' /etc/os-release) main" | tee /etc/apt/sources.list.d/adoptium.list \
    && rm -rf /var/lib/apt/lists/*
    
RUN apt-get update \
    && apt-get install -y \
        clang-format \
        temurin-21-jdk \
        python3 \
        python3-pip \
    && rm -rf /var/lib/apt/lists/*

RUN pip3 install --break-system-packages cxxfilt angr
COPY . /app
WORKDIR /app
RUN cargo build

ENV PLUGIN_PATH=/app/plugins
ENV GHIDRA_DIR=/app/tools/ghidra
ENV ANGR_DIR=/app/tools/angr

CMD [ "/bin/bash", "-c", "/app/target/debug/re_toolbox" ]