FROM rustlang/rust:nightly

EXPOSE 8000

ENV ROOT_PATH=/root/crablog/content
ENV DATABASE_URL=${ROOT_PATH}/db.sqlite3

RUN mkdir -p /root/crablog

COPY ./site /root/crablog

# set up database
WORKDIR /root/crablog

# install crablog
RUN cargo install --path . --root / -j $(nproc)

CMD ["crablog"]
