FROM centos:latest

ENV DATABASE_URL=postgres://postgres:ZH123as456@172.29.111.218:5432/zh
ENV REDIS_URL=redis://:ZH123as456@han2:32263

WORKDIR /app

COPY ./webservice webservice

CMD [ "./webservice" ]