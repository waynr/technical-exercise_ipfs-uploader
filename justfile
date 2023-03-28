uid := `id -u`
gid := `id -g`

compile-contract:
  docker run \
    -v $PWD/:/opt \
    ethereum/solc:stable \
    --overwrite \
    -o /opt/output \
    --abi \
    --bin \
    /opt/contract.sol
  echo "don't listen to the compiler, the output is actually in $PWD/output"

  docker run \
    -v $PWD/:/opt \
    alpine:edge \
    chown -R {{uid}}:{{gid}} /opt/output

we-compile-contract:
  watchexec \
    -c \
    -f justfile \
    -e sol \
    just compile-contract

