uid := `id -u`
gid := `id -g`

compile-contract:
  docker run \
    -v $PWD/:/opt \
    ethereum/solc:stable \
    --overwrite \
    -o /opt/output \
    --combined-json srcmap,abi,bin,opcodes \
    /opt/contract.sol
  echo "don't listen to the compiler, the output is actually in $PWD/output"

  docker run \
    -v $PWD/:/opt \
    alpine:edge \
    chown -R {{uid}}:{{gid}} /opt/output

  jq -r \
    '.contracts."opt/contract.sol:IPFSCIDStorage" | { abi: .abi, bytecode: { object: .bin, opcodes: .opcodes, sourceMap: .srcmap} }' \
    output/combined.json > output/IPFSCIDStorage.json

we-compile-contract:
  watchexec \
    -c \
    -f justfile \
    -e sol \
    just compile-contract

