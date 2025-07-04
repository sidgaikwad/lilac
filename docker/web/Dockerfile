
FROM node:24-alpine

WORKDIR /opt/lilac/web

COPY ./frontend/ml-pipeline-frontend/package.json .

RUN npm install --legacy-peer-deps

RUN npm i -g serve

COPY ./frontend/ml-pipeline-frontend .
COPY ./frontend/ml-pipeline-frontend/.env.production ./.env.production

RUN npm run build

EXPOSE 8080

CMD [ "serve", "-s", "dist", "-l", "8080" ]