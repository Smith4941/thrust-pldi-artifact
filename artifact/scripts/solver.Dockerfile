# The solver image of the replication package, built from the unpacked coar.zip beside this file:
#   (cd solver && unzip -q coar.zip && docker build -t coar:<commit>-pkg .)
# The zip carries its own loader and libraries, so the base only provides a shell. main.exe is
# coar.sh, which starts the binary through that loader, so the scripts that run
# `main.exe -c <config> -p pcsp <query>` in /root/coar work unchanged. make-package.sh appends
# `LABEL fptprove=<commit>`, the commit the zip was built from, which run.py checks.
FROM ubuntu:24.04
COPY coar /root/coar
RUN mv /root/coar/main.exe /root/coar/coar.exe \
 && sed -i 's#/main.exe"#/coar.exe"#' /root/coar/coar.sh \
 && ln -s /root/coar/coar.sh /root/coar/main.exe
ENV PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/root/coar
WORKDIR /root/coar
CMD ["/bin/bash"]
