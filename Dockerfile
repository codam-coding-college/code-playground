# -----------------------------------------------------------------------------
# Codam Coding College, Amsterdam @ 2022.
# See README in the root project for more information.
# -----------------------------------------------------------------------------

FROM node:18-bullseye

LABEL author="Codam Coding College"
LABEL email="it@codam.nl"

RUN apt-get -y update && apt-get -y install build-essential python3

WORKDIR /app

COPY src/ /app/src/
COPY package.json /app/
COPY package-lock.json /app/
COPY tsconfig.json /app/

RUN npm ci
RUN npm run build

EXPOSE 4242
ENTRYPOINT [ "npm", "run", "start" ]
