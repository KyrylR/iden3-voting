import { Deployer, Reporter } from "@solarity/hardhat-migrate";
import { Voting__factory, Groth16Verifier__factory } from "@ethers-v6";
import { generateSecrets, getCommitment, getNullifierHash, getZKP } from "@/test/helpers/zkp-helper";
import { ethers } from "hardhat";
import {
  buildSparseMerkleTree,
  getBytes32PoseidonHash,
  getPositionalProof,
  getRoot
} from "@/test/helpers/merkle-tree-helper";
import { getPoseidon, poseidonHash } from "@/test/helpers/poseidon-hash";

export = async (deployer: Deployer) => {
  const verifier = await deployer.deploy(Groth16Verifier__factory);
  const voting = await deployer.deploy(Voting__factory, [6, verifier.address], {
    libraries: {
      PoseidonUnit1L: await (await getPoseidon(1)).getAddress(),
      PoseidonUnit2L: await (await getPoseidon(2)).getAddress(),
    },
  });

  const proposalData = {
    commitmentPeriod: 100n,
    votingPeriod: 100n,
    proposalExecutionPeriod: 100n,
    requiredQuorum: 100n,
    requiredMajority: 100n,
  };

  const proposalId = 1n;

  await (await voting.createProposal("remark", proposalData, "0x")).wait();

  const pair1 = generateSecrets();
  const pair2 = generateSecrets();

  console.log("pair1: ", pair1);
  console.log("pair2: ", pair2);

  let commitment1 = getCommitment(pair1);
  let commitment2 = getCommitment(generateSecrets());
  let commitment3 = getCommitment(generateSecrets());
  let commitment4 = getCommitment(pair2);
  let commitment5 = getCommitment(generateSecrets());

  const localMerkleTree = buildSparseMerkleTree(
    poseidonHash,
    [
       getBytes32PoseidonHash(commitment1),
       getBytes32PoseidonHash(commitment2),
       getBytes32PoseidonHash(commitment3),
       getBytes32PoseidonHash(commitment4),
       getBytes32PoseidonHash(commitment5),
    ],
    6n
  );

  await (await voting.commitOnProposal(proposalId,  commitment1, { value: ethers.parseEther("1") })).wait();
  await (await voting.commitOnProposal(proposalId,  commitment2, { value: ethers.parseEther("1") })).wait();
  await (await voting.commitOnProposal(proposalId,  commitment3, { value: ethers.parseEther("1") })).wait();
  await (await voting.commitOnProposal(proposalId,  commitment4, { value: ethers.parseEther("1") })).wait();
  await (await voting.commitOnProposal(proposalId,  commitment5, { value: ethers.parseEther("1") })).wait();

  console.log("Leafs: ");
  console.log(localMerkleTree.getLeaves());

  console.log("leaf to prove: ", commitment1);
  console.log("leaf to prove: ", getBytes32PoseidonHash(commitment1));

  let [pathIndices, pathElements] = getPositionalProof(
    localMerkleTree,
    getBytes32PoseidonHash(commitment1)
  );

  console.log("result: ", pathIndices, pathElements);

  console.log("leaf to prove: ", commitment2);
  console.log("leaf to prove: ", getBytes32PoseidonHash(commitment2));

  [pathIndices, pathElements] = getPositionalProof(
    localMerkleTree,
    getBytes32PoseidonHash(commitment2)
  );

  console.log("result: ", pathIndices, pathElements);

  console.log("leaf to prove: ", commitment3);
  console.log("leaf to prove: ", getBytes32PoseidonHash(commitment3));

  [pathIndices, pathElements] = getPositionalProof(localMerkleTree, getBytes32PoseidonHash(commitment3));

  console.log("result: ", pathIndices, pathElements);

  console.log("leaf to prove: ", commitment4);
  console.log("leaf to prove: ", getBytes32PoseidonHash(commitment4));

  [pathIndices, pathElements] = getPositionalProof(localMerkleTree, getBytes32PoseidonHash(commitment4));

  console.log("result: ", pathIndices, pathElements);

  console.log("leaf to prove: ", commitment5);
  console.log("leaf to prove: ", getBytes32PoseidonHash(commitment5));

  [pathIndices, pathElements] = getPositionalProof(localMerkleTree, getBytes32PoseidonHash(commitment5));

  console.log("result: ", pathIndices, pathElements);

  console.log(await voting.getProposalStatus(proposalId));

  console.log("root: ");
  console.log(await voting.getRoot());
  console.log(getRoot(localMerkleTree));

  let dataToVerify = await getZKP(
    pair1,
    "0xf41cee234219d6cc3d90a6996dc3276ad378cfcf",
    proposalId.toString(),
    await voting.getRoot(),
    localMerkleTree
  );

  console.log("pair1: dataToVerify: ", dataToVerify);

  dataToVerify = await getZKP(
    pair1,
    "0xf41cee234219d6cc3d90a6996dc3276ad378cfcf",
    proposalId.toString(),
    await voting.getRoot(),
    localMerkleTree
  );

  console.log("pair2: dataToVerify: ", dataToVerify);

  Reporter.reportContracts(["Verifier", verifier.address], ["Voting", voting.address]);
};
