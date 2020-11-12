FROM openjdk:15
ARG MAIN_CLASS
ARG CODEGEN_DIR

COPY ./jars /work/jars
COPY ./Main.java /work/src/com/example/Main.java
RUN sed -i -e "s/CODEGEN_DIR/$CODEGEN_DIR/g" -e "s/MAIN_CLASS/$MAIN_CLASS/g" /work/src/com/example/Main.java
COPY ./codegen/$CODEGEN_DIR/ /work/src/com/example/

WORKDIR /work
RUN javac -cp 'jars/*' -d classes src/com/example/*.java
ENTRYPOINT java -cp 'jars/*:classes' com.example.Main
