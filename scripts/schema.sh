for d in contracts/*; do
  if [ -d "$d" ]; then
    cd $d
    cargo schema
    cd ../..
  fi
done
