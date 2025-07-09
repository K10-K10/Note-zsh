#!/bin/zsh

watch_dirs=("note" "note-rust")
tar_name="Note-zsh.tar.gz"

echo " Watching for changes in: ${watch_dirs[@]}..."

while true; do
  change=$(inotifywait -r -e modify,create,delete "${watch_dirs[@]}" 2>/dev/null)

  echo "Change detected: $change"
  echo "Building Rust (note-rust)..."

  if ! cargo build --release --manifest-path=note-rust/Cargo.toml; then
    echo "Build failed."
    continue
  fi

  echo "Creating archive $tar_name..."
  tar czf "$tar_name" note note-rust note.txt README.md images

  echo "Archive updated: $tar_name"
done
