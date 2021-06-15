# stage 0 - build html
FROM node:16.3-alpine AS html

WORKDIR /app

COPY ./package-lock.json ./package.json ./
RUN npm ci

COPY . .
RUN npm run build


# stage 1 - build go web 
FROM golang:1.16.0-buster AS web

WORKDIR /src

COPY --from=html /app/build ./build
COPY ./server ./
RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -a -ldflags '-w -extldflags "-static"' -o /go_web main.go


# stage 2 - move bin
FROM scratch

COPY --from=web /go_web /app/

ENTRYPOINT ["/app/go_web"]