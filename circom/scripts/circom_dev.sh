

### Boilerplate circuit compilation and vkey/zkey generation for development

# Powers of tau selection for Hermez Rollup
PTAU=../circuits/pot25_final.ptau
CIRCUIT_NAME=signature
BUILD_DIR=../build/"$CIRCUIT_NAME"

# Compile circuit
echo "****COMPILING CIRCUIT****"
start=`date +%s`
circom "$CIRCUIT_NAME".circom --r1cs --wasm --sym --output "$BUILD_DIR"
mv verif-manager_js/"$CIRCUIT_NAME".wasm .
export CPATH="$CPATH:/opt/homebrew/opt/nlohmann-json/include:/opt/homebrew/opt/gmp/include"
end=`date +%s`
echo "DONE COMPILING CIRCUIT ($((end-start))s)"

# Generate zkey
echo "****GENERATE ZKEY****"
start=`date +%s`
yarn run snarkjs groth16 setup "$CIRCUIT_NAME".r1cs $PTAU "$CIRCUIT_NAME".zkey
end=`date +%s`
echo "DONE GENERATING ZKEY ($((end-start))s)"

# Export verification key
echo "****EXPORT VERIFICATION KEY****"
start=`date +%s`
yarn run snarkjs zkey export verificationkey "$CIRCUIT_NAME".zkey "$CIRCUIT_NAME".vkey.json
end=`date +%s`
echo "DONE EXPORTING VERIFICATION KEY ($((end-start))s)"

# Verify protocol transcript, zkey <-- commented out to save on compilation time
# yarn run snarkjs zkey verify verif-manager.r1cs $PTAU verif-manager.zkey

# Generate the witness, primarily as a smoke test for the circuit
node "$CIRCUIT_NAME"_js/generate_witness.js "$CIRCUIT_NAME".wasm "$CIRCUIT_NAME".json "$CIRCUIT_NAME".wtns

# Export verifier to smart contract for on-chain verification
yarn run snarkjs zkey export solidityverifier "$CIRCUIT_NAME".zkey "$CIRCUIT_NAME"Verifier.sol
sed -i -e 's/0.6.11;/0.8.13;/g' "$CIRCUIT_NAME"Verifier.sol
mv "$CIRCUIT_NAME"Verifier.sol ../contracts
rm "$CIRCUIT_NAME"Verifier.sol-e