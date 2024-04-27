#* ************************************************************************** *#
##         .-.
##   __   /   \   __
##  (  `'.\   /.'`  )  Roze - Dockerfile
##   '-._.(;;;)._.-'
##   .-'  ,`"`,  '-.
##  (__.-'/   \'-.__)  By: Rosie (https://github.com/BlankRose)
##      //\   /        Created at: April 27, 2024 [2:17 PM]
##     ||  '-'
#* ************************************************************************** *#

FROM rust:1.77-alpine3.19 AS builder
WORKDIR /src
COPY . .
RUN cargo install --path . --root /out

FROM alpine:3.19 AS execution
LABEL authors="rosie"
COPY --from=builder /out/roze /app
ENTRYPOINT ["/app"]