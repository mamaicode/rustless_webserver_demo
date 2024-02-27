# Use rust base image
FROM rust
# Copy over the source code that we have built in the container
COPY . .
# Expose and map port 8080 to internal containers 8080
EXPOSE 8080:8080
# Use cargo, a rust package manager to produce a binary
RUN cargo build --release
# Run rust as binary that has been produced during the build process from our release folder
CMD [ "./target/release/rustless" ]
