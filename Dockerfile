FROM alpine:lastest

ENV DATABASE_URL=postgres://postgres:ZH123as456@localhost:5432/zh?charset=utf8&loc=Local

WORKDIR /app

COPY ./webservice webservice

CMD [ "./webservice" ]