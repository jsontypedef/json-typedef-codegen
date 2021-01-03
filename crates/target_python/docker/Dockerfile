FROM python:3.9

ARG MAIN

WORKDIR /work
COPY /main.py /work/main.py

COPY /gen /work/gen
RUN sed -i -e "s/MAIN/$MAIN/g" /work/main.py

ENTRYPOINT python3 -u main.py
