FROM golang:1.18.1-alpine AS build

WORKDIR /src
COPY . .

ENV CGO_ENABLED=0
RUN go build -o /bin/vclipbot .

FROM alpine:latest AS certificates

RUN apk --update add ca-certificates

FROM scratch

COPY --from=certificates /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=build /bin/vclipbot /bin/vclipbot

CMD ["vclipbot"]
