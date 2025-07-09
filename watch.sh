#!/bin/zsh

watch_targets=("note" "note-rust" "update.sh")
tar_name="Note-zsh.tar.gz"

echo "Watching for changes in: ${watch_targets[@]}..."

while true; do
  change=$(inotifywait -r -e modify,create,delete --format '%w%f' "${watch_targets[@]}" 2>/dev/null)

  if [[ -z "$change" ]]; then
    continue
  fi

  echo "Change detected: $change"
  if [[ "$change" == *"note-rust"* ]]; then
    echo "Building Rust (note-rust)..."
    if ! cargo build --release --manifest-path=note-rust/Cargo.toml; then
      echo "Rust build failed."
      continue
    fi
  else
    echo "Rust source not changed, skipping build."
  fi

  echo "Creating archive $tar_name..."
  tar czf "$tar_name" note note-rust note.txt README.md images
  echo "Archive updated: $tar_name"
  sleep 1
done
