FROM alpine:latest

ENV DATABASE_URL=postgres://postgres:ZH123as456@han1:5432/zh

WORKDIR /app

COPY ./webservice webservice

CMD [ "./webservice" ]