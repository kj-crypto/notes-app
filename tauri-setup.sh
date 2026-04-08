#!/usr/bin/env bash

IMAGE_NAME="tauri_env:0.2.0"
CONTAINER_NAME="tauri_env"

show_help() {
  cat <<EOF
Usage: $0 <command> [options]

Commands:
  build [--force|-f] [<docker build args>...]
      Build the Docker image for tauri environment.
      Only rebuild image if Dockerfile has changed.
      Use --force or -f to rebuild image even if Dockerfile has not changed.
  run <directory>
      Run the Docker container, mounting <directory> as a volume.
  check
      Check if dockerfile changed.
  --help
      Show this help message.
EOF
}

DOCKERFILE_NAME=$(find . -name '*.dockerfile' -type f | head -1)
if [ -z "$DOCKERFILE_NAME" ]; then
    echo "Error: No dockerfile found"
    exit 1
fi

build_image() {
  BUILD_TIME="$(date --iso-8601=seconds)"
  BUILD_NEW_IMAGE=0
  EXTRA_ARGS=()
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --force|-f)
        BUILD_NEW_IMAGE=1
        shift
        ;;
      *)
        EXTRA_ARGS+=("$1")
        shift
        ;;
    esac
  done

  # 1. Retrieve previous dockerfile_hash from image (if exists)
  local OLD_HASH=""
  if docker inspect $IMAGE_NAME >/dev/null 2>&1; then
    OLD_HASH=$(docker inspect $IMAGE_NAME | jq -r '.[0].Config.Labels.dockerfile_hash // empty')
  fi

  # 2. Hash the Dockerfile with dockerfile_hash replaced by zeros
  dummy_hash="0000000000000000000000000000000000000000000000000000000000000000"
  NEW_HASH=$(sed -E 's/(dockerfile_hash=")[a-fA-F0-9]{64}/\1'$dummy_hash'/' $DOCKERFILE_NAME | sha256sum | awk '{print $1}')


  # 3. Compare hashes, update Dockerfile if needed
  if [[ "$OLD_HASH" != "$NEW_HASH" ]]; then
    # Update the hash in the Dockerfile
    sed -i -E "s/(dockerfile_hash=\")[a-fA-F0-9]{64}/\1$NEW_HASH/" $DOCKERFILE_NAME
    echo "Updated dockerfile_hash label to $NEW_HASH"
    BUILD_NEW_IMAGE=1
  else
    echo "dockerfile_hash unchanged"
  fi

  # 4. Build image
  if [[ $BUILD_NEW_IMAGE -eq 1 ]]; then
    docker build --no-cache --progress=plain --build-arg BUILD_TIME="$BUILD_TIME" -t $IMAGE_NAME -f $DOCKERFILE_NAME "${EXTRA_ARGS[@]}" .
  fi
}

run_container() {
  if [[ -z "$1" ]]; then
    echo "Error: Directory argument required for run"
    show_help
    exit 1
  fi
  DIR=$(readlink -f "$1")
  docker run -v "$DIR":/app/data --name $CONTAINER_NAME -it --rm --user $(id -u):$(id -g) --workdir /app/data $IMAGE_NAME bash
}

check_dockerfile() {
  dummy_hash="0000000000000000000000000000000000000000000000000000000000000000"
  NEW_HASH=$(sed -E 's/(dockerfile_hash=")[a-fA-F0-9]{64}/\1'$dummy_hash'/' $DOCKERFILE_NAME | sha256sum | awk '{print $1}')
  OLD_HASH=$(sed -En 's/.*dockerfile_hash="([a-fA-F0-9]{64})"/\1/p' $DOCKERFILE_NAME)

  if [[ "$OLD_HASH" != "$NEW_HASH" ]]; then
    echo "Dockerfile has changed."
    read -p "Do you want to update dockerfile? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
      sed -i -E "s/(dockerfile_hash=\")[a-fA-F0-9]{64}/\1$NEW_HASH/" $DOCKERFILE_NAME
      echo "Updated dockerfile_hash label to $NEW_HASH"
    fi
  else
    echo "Dockerfile has not changed"
  fi
}

case "$1" in
  build)
    shift
    build_image "$@"
    ;;
  run)
    shift
    run_container "$@"
    ;;
  check)
    shift
    check_dockerfile
    ;;
  --help|-h)
    show_help
    ;;
  *)
    show_help
    exit 1
    ;;
esac
