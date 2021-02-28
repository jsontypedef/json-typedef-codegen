FROM ruby:3.0.0

ARG MAIN

WORKDIR /work
COPY /main.rb /work/main.rb

COPY /gen /work/gen
RUN sed -i -e "s/MAIN/$MAIN/g" /work/main.rb

ENTRYPOINT ruby main.rb
