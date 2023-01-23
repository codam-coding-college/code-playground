# -----------------------------------------------------------------------------
# Codam Coding College, Amsterdam @ 2022.
# See README in the root project for more information.
# -----------------------------------------------------------------------------

FROM node:18-bullseye

LABEL author="W2Wizard"
LABEL email="it@codam.nl"

EXPOSE 4242
RUN apt-get -y update && apt-get -y install build-essential python3

COPY . .

RUN npm run build
ENTRYPOINT [ "npm", "run", "start" ]