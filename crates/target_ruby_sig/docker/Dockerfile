FROM ruby:3.0.0

ARG MAIN

# At the time of writing, this test works but fails with high probability if we
# run multiple tests in parallel, with the docker client getting back invalid
# responses from the daemon. It's not clear why this happens.
#
# For the moment, we will live without having `steep` or any other RBS
# implementation verify that the generated types are semantically valid.
#
# An improvement that could be made for these tests is to run them alongside the
# generated Ruby code, to prevent generated rb and rbs files from diverging.

# WORKDIR /work
# COPY /main.rb /work/lib/main.rb
# COPY /Gemfile /work/Gemfile
# COPY /Gemfile.lock /work/Gemfile.lock
# COPY /Steepfile /work/Steepfile

# RUN bundle install

# COPY /gen /work/sig
# RUN sed -i -e "s/MAIN/$MAIN/g" /work/lib/main.rb

# RUN bundle exec steep check

ENTRYPOINT cat
