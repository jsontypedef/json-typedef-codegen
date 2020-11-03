FROM node:15.0.1
ARG MAIN_CLASS
ARG CODEGEN_DIR

# First, copy over infrequently-changing files.
COPY ./package.json /work/
COPY ./tsconfig.json /work/
COPY ./main.ts /work/

# Then, prepare these infrequently-changing files and install dependencies.
#
# Our goal here is to avoid reinstalling dependencies after running jtd-codegen.
WORKDIR /work
RUN sed -i -e "s/CODEGEN_DIR/$CODEGEN_DIR/g" -e "s/MAIN_CLASS/$MAIN_CLASS/g" main.ts
RUN npm install > /dev/null 2>&1

# Finally, copy over frequently-changing files and compile the code.
COPY ./codegen/$CODEGEN_DIR/ /work/$CODEGEN_DIR
RUN ./node_modules/.bin/tsc

ENTRYPOINT ["node", "main.js"]
