FROM node:15.5.1

ARG MAIN

RUN npm install -g typescript

WORKDIR /work
COPY /index.ts /work/index.ts

COPY /gen /work/jtd_codegen_e2e/
RUN sed -i -e "s/MAIN/$MAIN/g" /work/index.ts

RUN tsc --noEmit index.ts
ENTRYPOINT cat
