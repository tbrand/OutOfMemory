FROM node:lts-alpine as builder

ARG buildmode

WORKDIR /app

COPY package*.json \
  yarn.lock \
  tsconfig.json \
  babel.config.js \
  .env.* \
  .eslintrc.js \
  .eslintignore ./

COPY src ./src

RUN yarn install

ENV BUILD_MODE ${buildmode:-production}
RUN echo "build on ${BULID_MODE}" && \
  $(npm bin)/vue-cli-service build --mode $BUILD_MODE

FROM nginx:stable-alpine as runner

RUN rm /etc/nginx/conf.d/default.conf

COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx/default.conf /etc/nginx/conf.d/

EXPOSE 8080

CMD ["nginx", "-g", "daemon off;"]
