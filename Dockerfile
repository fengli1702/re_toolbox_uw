# Stage 1: Base image and environment
FROM rust:latest
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update \
    && apt-get install -y apt-transport-https gpg \
    && wget -qO - https://packages.adoptium.net/artifactory/api/gpg/key/public | gpg --dearmor | tee /etc/apt/trusted.gpg.d/adoptium.gpg > /dev/null \
    && echo "deb https://packages.adoptium.net/artifactory/deb $(awk -F= '/^VERSION_CODENAME/{print$2}' /etc/os-release) main" | tee /etc/apt/sources.list.d/adoptium.list \
    && rm -rf /var/lib/apt/lists/*
    
# Stage 2: Install system dependencies, Python tools, LLVM
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        # For adding repos and basic utilities
        ca-certificates \
        # Python
        python3 \
        python3-pip \
        # Pipenv for managing Python dependencies from Pipfiles in /app/tools/*
        pipenv \
        # KLEE + Z3 system dependencies & build tools
        build-essential \
        cmake \
        curl \
        file \
        g++-multilib \
        gcc-multilib \
        git \
        unzip \
        libcap-dev \
        libgoogle-perftools-dev \
        libncurses-dev \
        libsqlite3-dev \
        libtcmalloc-minimal4 \
        graphviz \
        doxygen \
        # From user's original Dockerfile context
        temurin-21-jdk\
        clang-format \
    # Install LLVM 15 directly
    && apt-get install -y --no-install-recommends \
        clang-15 \
        llvm-15 \
        llvm-15-dev \
        llvm-15-tools \
    # Clean up apt cache
    && rm -rf /var/lib/apt/lists/*

# Install KLEE's core Python dependencies (lit, wllvm, tabulate) globally
RUN pip3 install --break-system-packages lit wllvm tabulate
RUN pip3 install --break-system-packages cxxfilt angr
# --- Building KLEE and dependencies from source ---

# Stage 3: Build and install Z3 from source
WORKDIR /tmp/build_z3_src
RUN git clone https://github.com/Z3Prover/z3.git . && \
    python3 scripts/mk_make.py && \
    cd build && \
    make -j$(nproc) && \
    make install && \
    # Clean up the build directory
    rm -rf /tmp/build_z3_src

# Stage 4: Build klee-uclibc from source and move to a persistent location
WORKDIR /tmp/build_klee_uclibc_src
RUN git clone https://github.com/klee/klee-uclibc.git . && \
    chmod -R +x . && \
    ./configure --make-llvm-lib --with-cc=clang-15 --with-llvm-config=llvm-config-15 && \
    make -j$(nproc) && \
    # Move built klee-uclibc to /opt for KLEE to find
    mkdir -p /opt/klee-uclibc-built && \
    cp -R ./* /opt/klee-uclibc-built/ && \
    # Clean up the temporary build directory
    rm -rf /tmp/build_klee_uclibc_src

# KLEE will be configured with -DKLEE_UCLIBC_PATH=/opt/klee-uclibc-built

# Stage 5: Copy googletest source to a persistent location
# KLEE's unit tests require the googletest source directory.
RUN mkdir -p /opt && cd /opt && \
    curl -L https://github.com/google/googletest/archive/release-1.11.0.zip -o googletest.zip && \
    unzip googletest.zip && \
    rm googletest.zip && \
    mv googletest-release-1.11.0 googletest-source
# KLEE will be configured with -DGTEST_SRC_DIR=/opt/googletest-source/googletest
# (assuming 'googletest' is the subdirectory within your source_googletest folder that contains CMakeLists.txt for gtest)
# If your source_googletest *is* the googletest directory itself, then use /opt/googletest-source

# Stage 6: Build and install KLEE from source
WORKDIR /tmp/build_klee_src
RUN git clone https://github.com/klee/klee.git . && \
    mkdir build && cd build && \
    cmake \
        -DENABLE_SOLVER_STP=OFF \
        -DENABLE_POSIX_RUNTIME=ON \
        -DENABLE_UNIT_TESTS=ON \
        -DKLEE_UCLIBC_PATH=/opt/klee-uclibc-built \
        -DGTEST_SRC_DIR=/opt/googletest-source \
        -DLLVM_DIR=/usr/lib/llvm-15/lib/cmake/llvm \
        -Dgtest_build_tests=OFF \
        # Add any other KLEE cmake options you need
        .. && \
    make -j$(nproc) && \
    make install && \
    # Clean up the build directory
    rm -rf /tmp/build_klee_src
# --- End of Building KLEE and dependencies ---

# Stage 7: Application setup - copy your project and install Pipfile dependencies
WORKDIR /app
# Copy your entire project context
COPY . .
RUN chmod +x .
# Stage 8: Build your Rust application
RUN cargo build

# Stage 9: Set runtime environment variables
ENV PLUGIN_PATH=/app/plugins
ENV GHIDRA_DIR=/app/tools/ghidra
ENV ANGR_DIR=/app/tools/angr
# KLEE and Z3 'make install' should add their binaries to the system PATH.

# Stage 10: Define the default command
CMD [ "/bin/bash", "-c", "/app/target/debug/re_toolbox" ]
