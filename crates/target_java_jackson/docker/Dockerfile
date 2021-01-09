FROM openjdk:15

ARG MAIN

COPY /*.jar /work/jars/
COPY /Main.java /work/src/com/example/Main.java
RUN sed -i -e "s/MAIN/$MAIN/g" /work/src/com/example/Main.java
COPY /gen /work/src/com/example/

WORKDIR /work
RUN javac -cp 'jars/*' -d classes src/com/example/*.java
ENTRYPOINT java -cp 'jars/*:classes' com.example.Main
