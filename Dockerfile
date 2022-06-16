FROM alpine:latest

ENV DATABASE_URL=postgres://postgres:ZH123as456@172.29.111.218:5432/zh

WORKDIR /app

COPY ./webservice webservice

CMD [ "./webservice" ]