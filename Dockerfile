# Build with podman/docker build . --platform=linux/amd64 -t hackpack_rev
# Run with podman/docker run -cap-add=SYS_PTRACE --security-opt seccomp=unconfined -it hackpack_rev /bin/bash
FROM ubuntu:latest

RUN apt-get update && apt-get install -y \
    radare2 \
    gdb \
    binutils \
    coreutils
    
ENTRYPOINT ["/bin/bash"]
