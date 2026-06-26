# Base image with Rust + Cargo preinstalled.
# Slim variant, available for both amd64 and arm64 (required for multi-arch tests).
FROM docker.io/library/rust:slim

# wget to download the tile dataset, unzip to extract it.
RUN apt-get update \
    && apt-get install -y --no-install-recommends wget unzip \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /moseiik

# Copy the project sources (assets/images is gitignored, so not included here).
COPY . .

# Download and extract the tile dataset into assets/.
# -o: overwrite without prompting (the build is non-interactive).
RUN wget -q https://nasext-vaader.insa-rennes.fr/ietr-vaader/moseiik_test_images.zip \
    && unzip -q -o moseiik_test_images.zip -d assets/ \
    && rm moseiik_test_images.zip

# Precompile tests so that container startup runs them directly.
RUN cargo test --release --no-run

# Run tests on container start. Trailing -- forwards args to the tests.
ENTRYPOINT ["cargo", "test", "--release", "--"]
