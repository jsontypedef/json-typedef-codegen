FROM python:3.8.6
ARG MAIN_CLASS
ARG CODEGEN_DIR

COPY ./main.py /
COPY ./codegen/$CODEGEN_DIR/ /$CODEGEN_DIR
RUN sed -i -e "s/CODEGEN_DIR/$CODEGEN_DIR/g" -e "s/MAIN_CLASS/$MAIN_CLASS/g" main.py
ENTRYPOINT ["python3", "main.py"]
