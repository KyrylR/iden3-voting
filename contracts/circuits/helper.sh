#!/bin/bash
set -e

command_name="circom"

CIRCUIT_NAME="Voting"
SETUP_POWERS=13
BUILD_DIR=""
CIRCUIT_FILE=""
POWERS_FILE="/Users/kirilrs/ptau/24.ptau"

# Define the build directory where intermediate files will be stored
if [ -d ./circuits ]; then
    BUILD_DIR="./circuits/outputs"
    CIRCUIT_FILE="./circuits/$CIRCUIT_NAME.circom"
elif [ -d ../circuits ]; then
    BUILD_DIR="../circuits/outputs"
    CIRCUIT_FILE="../circuits/$CIRCUIT_NAME.circom"
else
    echo "Error: can't find way to circuits folder: unknow directory."
    exit 1
fi

if [ -z "$CIRCUIT_NAME" ]; then
    echo "Error: CIRCUIT_NAME is empty."
    exit 1
elif [ ! -e "$CIRCUIT_FILE" ]; then
    echo "Error: circuit doesn't exist."
    exit 1
fi

rm -rf ${BUILD_DIR}
mkdir -p ${BUILD_DIR}

# Compiling circuit with .r1cs and .wasm files as result
echo -e "\nCompiling the circuits..."

$command_name ${CIRCUIT_FILE} --r1cs --wasm --sym -o ${BUILD_DIR}

mv ${BUILD_DIR}/${CIRCUIT_NAME}_js/${CIRCUIT_NAME}.wasm ${BUILD_DIR}/voting.wasm

echo -e "\nCircuit compiled ${BUILD_DIR}"

# Exporting key with verification_key.json, verifier.sol and circtuis_final.zkey as a result
echo -e "\nExporting keys..."

snarkjs groth16 setup ${BUILD_DIR}/${CIRCUIT_NAME}.r1cs ${POWERS_FILE} ${BUILD_DIR}/circuit_final.zkey -v

snarkjs zkey export verificationkey ${BUILD_DIR}/circuit_final.zkey ${BUILD_DIR}/verification_key.json
snarkjs zkey export solidityverifier ${BUILD_DIR}/circuit_final.zkey ${BUILD_DIR}/verifier.sol

echo -e "\nKeys exported $BUILD_DIR/circuit_final.zkey, $BUILD_DIR/verification_key.json, $BUILD_DIR/verifier.sol"

cp $BUILD_DIR/verifier.sol ./contracts/utils/Verifier.sol
