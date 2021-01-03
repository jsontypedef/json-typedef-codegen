FROM golang:1.15.6

ARG MAIN

WORKDIR /work
COPY /go.mod /work/go.mod
COPY /main.go /work/main.go

COPY /gen /work/jtd_codegen_e2e/
RUN sed -i -e "s/MAIN/$MAIN/g" /work/main.go

ENTRYPOINT go run main.go
