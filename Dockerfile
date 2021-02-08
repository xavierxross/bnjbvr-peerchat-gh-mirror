FROM node:latest

COPY . /home/node

EXPOSE 8080

ENV HOST=0.0.0.0
ENV NODE_ENV production

RUN apt-get update -qq && \
    apt-get upgrade -qy && \
    apt-get install -qy

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# clean uploaded archives
RUN apt-get clean

# install FRONT dependenies
RUN cd /home/node/client && yarn

# install BACK dependencies
RUN cd /home/node/server && cargo build

WORKDIR /home/node/server

ENTRYPOINT ["cargo"]
CMD ["run"]
