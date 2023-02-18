/** @type import('hardhat/config').HardhatUserConfig */
require("hardhat-circom");

module.exports = {
  solidity: {
    compilers: [
      {
        version: "0.6.11",
      },
      {
        version: "0.8.9",
      },
      {
        version: "0.8.17",
      },
    ],
  },
  circom: {
    inputBasePath: "./scripts",
    ptau: "https://hermez.s3-eu-west-1.amazonaws.com/powersOfTau28_hez_final_25.ptau",
    circuits: [
      {
        name: "signature",
      },
    ],
  },
};
