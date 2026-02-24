# Stage 1 (Build)
#
FROM rust:1.82-slim-bullseye AS build
WORKDIR /fintrack
COPY . .
RUN cargo build --release

#
# Stage 2 (Run)
#
FROM gcr.io/distroless/cc
WORKDIR /fintrack
COPY --from=build /fintrack/target/release/fintrack .
EXPOSE 80
CMD ["./fintrack"]