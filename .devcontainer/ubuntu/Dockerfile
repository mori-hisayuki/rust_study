FROM ubuntu:20.04

ARG USER_ID
ARG USER_NAME
ARG GROUP_ID
ARG GROUP_NAME
ARG RUST_VERSION

# 証明書のDL
RUN set -x \
    && apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
    && apt-get clean && rm -rf /var/cache/apt/archives/* /var/lib/apt/lists/*
# Set TimeZone
RUN set -x \
    && apt-get update \
    && apt-get install -y --no-install-recommends \
        tzdata \
    && apt-get clean && rm -rf /var/cache/apt/archives/* /var/lib/apt/lists/* \
    && ln -sf /usr/share/zoneinfo/Asia/Tokyo /etc/localtime \
    && echo 'Asia/Tokyo' >/etc/timezone
# Install sudo
RUN set -x \
    && apt-get -y update \
    && apt-get install -y --no-install-recommends \
        sudo \
    && apt-get clean && rm -rf /var/cache/apt/archives/* /var/lib/apt/lists/*

# create group
RUN set -x \
    && groupadd --gid ${GROUP_ID} ${GROUP_NAME}

# create user
RUN set - x \
    && echo "$USER_NAME ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers.d/${USER_NAME} \
    && chmod 0440 /etc/sudoers.d/${USER_NAME} \
    && useradd \
        --uid ${USER_ID} \
        --gid ${GROUP_ID} \
        --home-dir /home/${USER_NAME} \
        --create-home \
        --shell /bin/bash \
        ${USER_NAME}

# common tools install
RUN set -x \
    && apt-get -y update \
    && apt-get install -y --no-install-recommends \
        git \
        curl \
        openssh-client \
        jq \
        unzip \
        wget \
        # gccのコンパイラが必要
        build-essential \
    && apt-get clean && rm -rf /var/cache/apt/archives/* /var/lib/apt/lists/*

# vscode extensions cache (extensionsの再インストールを防ぐ)
# https://code.visualstudio.com/docs/remote/containers-advanced#_avoiding-extension-reinstalls-on-container-rebuild
RUN set -x \
    && mkdir -p /home/${USER_NAME}/.vscode-server/extensions \
    && chown -R ${GROUP_NAME}:${USER_NAME} /home/${USER_NAME}/.vscode-server

USER ${USER_NAME}

# install Rust
# USERを切り替えてからUserHomeにインストール
RUN set -x \
    && curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain ${RUST_VERSION}
ENV PATH $PATH:$HOME/.cargo/bin

# create workingirectory
RUN set -x \
    && mkdir /home/${USER_NAME}/develop
WORKDIR /home/${USER_NAME}/develop
