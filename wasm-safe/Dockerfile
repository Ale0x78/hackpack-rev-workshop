FROM python:buster

COPY ./app /app

WORKDIR /app
EXPOSE 8000

RUN useradd -ms /bin/bash challenge
USER challenge

ENTRYPOINT [ "python3", "-m", "http.server" ]
