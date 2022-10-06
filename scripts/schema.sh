START_DIR=$(pwd)
for f in ./contracts/*
do
  cd "$f"
  CMD="cargo run --example schema"
  eval $CMD
  cd "$START_DIR"
done

# Also generate schema for cw721 package.
cd packages/cw721 && cargo schema
