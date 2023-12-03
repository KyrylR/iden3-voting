import { Deployer, Reporter } from "@solarity/hardhat-migrate";
import { Voting__factory, Groth16Verifier__factory } from "@ethers-v6";
import { generateSecrets, getCommitment, getNullifierHash } from "@/test/helpers/zkp-helper";
import {ethers} from "hardhat";
import { buildSparseMerkleTree, getBytes32PoseidonHash, getPositionalProof } from "@/test/helpers/merkle-tree-helper";
import { poseidonHash } from "@/test/helpers/poseidon-hash";

export = async (deployer: Deployer) => {
  // const verifier = await deployer.deploy(Groth16Verifier__factory);
  // const voting = await deployer.deploy(Voting__factory, [6, verifier.address]);
  //
  // const voting = await deployer.deployed(Voting__factory, "0x0Cc167025b2923fea6b6D54e0cC942fe107417fB");
  //
  // const proposalData = {
  //   commitmentPeriod: 100n,
  //   votingPeriod: 100n,
  //   proposalExecutionPeriod: 100n,
  //   requiredQuorum: 100n,
  //   requiredMajority: 100n,
  // };
  //
  // const proposalId = 1n;
  //
  // await voting.createProposal("remark", proposalData, "0x");
  //
  // let commitment1 = getCommitment(generateSecrets());
  // let commitment2 = getCommitment(generateSecrets());
  // let commitment3 = getCommitment(generateSecrets());
  // let commitment4 = getCommitment(generateSecrets());
  // let commitment5 = getCommitment(generateSecrets());

  const localMerkleTree = buildSparseMerkleTree(poseidonHash, [
    "0x177d98c97142d7a318fbbee6797b30c23a4c454c69ca50d3d7aac4d7d2889922",
    "0x064485cd762f32e405913a22ff41619a8fc7f095df2c41b09e91ae8358754b3b",
    "0x09251d514d26d40b3359b68285fd4857402ae2bb907fba07034ed110a66aa500",
    "0x015b149a8d6076573bc59307b4073d506585d1b36721e1bdd5de28f33461f9a3",
    "0x3063e7ed2c8a3e00254175f7f2001eb567db2e71847ad730705e34a475da80b3",
    "0x11a0b53a215568d828e4a4f80722fbf88a8de6c14194116bd23cda6fbd8c21af",
    "0x0b4587534f43840ba106ca40ef38c87d230188ad1a594e6849edd2cbdad4ff6b",
    "0x131d7a8c38b3d72770d4ba4354676a6b74a6eebb3a507b75b05fcfb0166829ab",
    "0x24dcdacdeac518ad2c4852b3e64219a2d33792ebd9a128beb1c8605cbbfd7b6b",
    "0x20d7b2d276befaf345849b9429e552d279352bc584dda9e904d11ac350a7b9d1"
  ], 6n);

  // await voting.commitOnProposal(proposalId,  commitment1, { value: ethers.parseEther("1") });
  // await voting.commitOnProposal(proposalId,  commitment2, { value: ethers.parseEther("1") });
  // await voting.commitOnProposal(proposalId,  commitment3, { value: ethers.parseEther("1") });
  // await voting.commitOnProposal(proposalId,  commitment4, { value: ethers.parseEther("1") });
  // await voting.commitOnProposal(proposalId,  commitment5, { value: ethers.parseEther("1") });

  // console.log("commitment", "0x11a0b53a215568d828e4a4f80722fbf88a8de6c14194116bd23cda6fbd8c21af");

  console.log("Leafs: ");
  console.log(localMerkleTree.getLeaves())

  console.log("leaf to prove: ", "0x11a0b53a215568d828e4a4f80722fbf88a8de6c14194116bd23cda6fbd8c21af");

  let [pathIndices, pathElements] = getPositionalProof(localMerkleTree,"0x11a0b53a215568d828e4a4f80722fbf88a8de6c14194116bd23cda6fbd8c21af");

  console.log("result: ", pathIndices, pathElements);

  console.log("leaf to prove: ", "0x20d7b2d276befaf345849b9429e552d279352bc584dda9e904d11ac350a7b9d1");

  [pathIndices, pathElements] = getPositionalProof(localMerkleTree, "0x20d7b2d276befaf345849b9429e552d279352bc584dda9e904d11ac350a7b9d1");

  console.log("result: ", pathIndices, pathElements);
  //
  // console.log("leaf to prove: ", commitment3);
  //
  // [pathIndices, pathElements] = getPositionalProof(localMerkleTree, commitment3);
  //
  // console.log("result: ", pathIndices, pathElements);
  //
  // console.log("leaf to prove: ", commitment4);
  //
  // [pathIndices, pathElements] = getPositionalProof(localMerkleTree, commitment4);
  //
  // console.log("result: ", pathIndices, pathElements);
  //
  // console.log("leaf to prove: ", commitment5);
  //
  // [pathIndices, pathElements] = getPositionalProof(localMerkleTree, commitment5);
  //
  // console.log("result: ", pathIndices, pathElements);

  // Reporter.reportContracts(["Verifier", verifier.address], ["Voting", voting.address]);
};
