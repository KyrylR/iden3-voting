import { assert, expect } from "chai";
import { BytesLike } from "ethers";
import { ethers } from "hardhat";

import { SignerWithAddress } from "@nomicfoundation/hardhat-ethers/signers";
import { impersonateAccount, time } from "@nomicfoundation/hardhat-network-helpers";

import { Voting, Voting__factory } from "@ethers-v6";

import {
  CommitmentFields,
  getZKP,
  getPoseidon,
  getCommitment,
  generateSecrets,
  Reverter, getBytes32PoseidonHash
} from "@test-helpers";


describe("Voting", () => {
  const reverter = new Reverter();

  let OWNER: SignerWithAddress;
  let USER1: SignerWithAddress;

  let voting: Voting;

  let treeHeight = 80n;

  const DEFAULT_DATA = {
    commitmentPeriod: 100n,
    votingPeriod: 100n,
    proposalExecutionPeriod: 100n,
    requiredQuorum: 100n,
    requiredMajority: 100n,
  };

  before(async () => {
    [OWNER, USER1] = await ethers.getSigners();

    const verifierFactory = await ethers.getContractFactory("Groth16Verifier");
    const verifier = await verifierFactory.deploy();

    const Voting = await ethers.getContractFactory("Voting", {
      libraries: {
        PoseidonUnit1L: await (await getPoseidon(1)).getAddress(),
        PoseidonUnit2L: await (await getPoseidon(2)).getAddress(),
        PoseidonUnit3L: await (await getPoseidon(3)).getAddress(),
      },
    });
    voting = await Voting.deploy(treeHeight, await verifier.getAddress());

    await reverter.snapshot();
  });

  afterEach(reverter.revert);

  async function prepareForVoting(
    proposalId: number,
    proposalArgs: [remark_: string, proposalData_: Voting.ProposalDataStruct, callData_: BytesLike],
    isTimeIncreased: boolean = true
  ): Promise<[CommitmentFields, string]> {
    assert.isTrue((await voting.proposalsCount()) === BigInt(proposalId) - 1n, "Proposal ID is not correct");

    await voting.createProposal(...proposalArgs);

    const pair = generateSecrets(proposalId);
    const commitment = getCommitment(pair);

    await voting.commitOnProposal(proposalId, commitment, { value: ethers.parseEther("1").toString() });

    if (isTimeIncreased) {
      await time.increase(101);
    }

    return [pair, commitment];
  }

  describe("#proposal creation", () => {
    it("should create proposal", async () => {
      const proposalData = {
        commitmentPeriod: 100n,
        votingPeriod: 100n,
        proposalExecutionPeriod: 100n,
        requiredQuorum: 100n,
        requiredMajority: 100n,
      };

      let nextBlockTime = ethers.toBigInt((await time.latest()) + 10);
      await time.setNextBlockTimestamp(nextBlockTime);

      await voting.createProposal("remark", proposalData, "0x");

      const proposal = await voting.proposals(1);

      expect(proposal.remark).to.equal("remark");
      expect(proposal.params.commitmentEndTime).to.equal(nextBlockTime + proposalData.commitmentPeriod);
      expect(proposal.params.votingEndTime).to.equal(proposal.params.commitmentEndTime + proposalData.votingPeriod);
      expect(proposal.params.proposalExecutionEndTime).to.equal(
        proposal.params.votingEndTime + proposalData.proposalExecutionPeriod
      );
      expect(proposal.params.requiredQuorum).to.equal(proposalData.requiredQuorum);
      expect(proposal.params.requiredMajority).to.equal(proposalData.requiredMajority);
      expect(proposal.callData).to.equal("0x");
    });

    it("should not create proposal with commitment period less than 0", async () => {
      const proposalData = {
        commitmentPeriod: 0,
        votingPeriod: 100,
        proposalExecutionPeriod: 100,
        requiredQuorum: 100,
        requiredMajority: 100,
      };

      await expect(voting.createProposal("remark", proposalData, "0x")).to.be.rejectedWith(
        "Voting: commitment period must be greater than 0"
      );
    });

    it("should not create proposal with voting period less than 0", async () => {
      const proposalData = {
        commitmentPeriod: 100,
        votingPeriod: 0,
        proposalExecutionPeriod: 100,
        requiredQuorum: 100,
        requiredMajority: 100,
      };

      await expect(voting.createProposal("remark", proposalData, "0x")).to.be.rejectedWith(
        "Voting: voting period must be greater than 0"
      );
    });

    it("should not create proposal with proposal execution period less than 0", async () => {
      const proposalData = {
        commitmentPeriod: 100,
        votingPeriod: 100,
        proposalExecutionPeriod: 0,
        requiredQuorum: 100,
        requiredMajority: 100,
      };

      await expect(voting.createProposal("remark", proposalData, "0x")).to.be.rejectedWith(
        "Voting: proposal execution period must be greater than 0"
      );
    });
  });

  describe("#commitment", () => {
    let proposalId: number = 1;

    beforeEach(async () => {
      await voting.createProposal("remark", DEFAULT_DATA, "0x");
    });

    it("should commit on proposal", async () => {
      const commitment = getCommitment(generateSecrets(proposalId));

      await voting.commitOnProposal(proposalId, commitment, { value: ethers.parseEther("1") });

      const node = await voting.getNodeByKey(getBytes32PoseidonHash(commitment));

      expect(node.value).to.equal(commitment);
      expect(await voting.commitments(commitment)).to.be.true;
      expect(await ethers.provider.getBalance(await voting.getAddress())).to.equal(ethers.parseEther("1"));
    });

    it("should not commit without value", async () => {
      const commitment = getCommitment(generateSecrets(proposalId));

      await expect(
        voting.commitOnProposal(proposalId, commitment, { value: ethers.parseEther("0") })
      ).to.be.revertedWith("Voting: value must be 1 ether");
    });

    it("should not commit two times", async () => {
      const commitment = getCommitment(generateSecrets(proposalId));

      await voting.commitOnProposal(proposalId, commitment, { value: ethers.parseEther("1") });
      await expect(
        voting.commitOnProposal(proposalId, commitment, { value: ethers.parseEther("1") })
      ).to.be.revertedWith("Voting: commitment already exists");
    });

    it("should not commit after commitment period", async () => {
      const commitment = getCommitment(generateSecrets(proposalId));

      await time.increase(101);

      await expect(
        voting.commitOnProposal(proposalId, commitment, { value: ethers.parseEther("1") })
      ).to.be.revertedWith("Voting: status is not COMMITMENT");
    });
  });

  describe("#distributeFunds", () => {
    beforeEach("setup", async () => {
      await OWNER.sendTransaction({
        from: await OWNER.getAddress(),
        to: await voting.getAddress(),
        value: ethers.parseEther("10"),
        data: "0x",
      });
    });

    it("should distribute funds", async () => {
      const userBalanceBefore = await ethers.provider.getBalance(await USER1.getAddress());

      await impersonateAccount(await voting.getAddress());
      const signer = await ethers.provider.getSigner(await voting.getAddress());

      await voting.connect(signer).distributeFunds(await USER1.getAddress(), ethers.parseEther("1"));

      const userBalanceAfter = await ethers.provider.getBalance(await USER1.getAddress());

      expect(userBalanceAfter - userBalanceBefore).to.equal(ethers.parseEther("1"));
    });

    it("should not distribute funds if called not from voting contract", async () => {
      await expect(voting.distributeFunds(await USER1.getAddress(), ethers.parseEther("1"))).to.be.revertedWith(
        "Voting: caller is not this contract"
      );
    });

    it("should not distribute funds if amount is zero", async () => {
      await impersonateAccount(await voting.getAddress());
      const signer = await ethers.provider.getSigner(await voting.getAddress());

      await expect(
        voting.connect(signer).distributeFunds(await USER1.getAddress(), ethers.parseEther("0"))
      ).to.be.revertedWith("Voting: amount must be greater than 0");
    });

    it("should not distribute funds if call failed", async () => {
      await impersonateAccount(await voting.getAddress());
      const signer = await ethers.provider.getSigner(await voting.getAddress());

      await expect(
        voting.connect(signer).distributeFunds(await USER1.getAddress(), ethers.parseEther("100"))
      ).to.be.revertedWith("Voting: funds distribution failed");
    });
  });

  describe("#vote", () => {
    let proposalId: number = 1;

    let pair: CommitmentFields;

    beforeEach(async () => {
      [pair,] = await prepareForVoting(proposalId, ["remark", DEFAULT_DATA, "0x"]);
    });

    it("should vote for proposal", async () => {
      const dataToVerify = await getZKP(voting, pair, OWNER.address, proposalId.toString());

      await voting.voteOnProposal(
        proposalId,
        dataToVerify.nullifierHash,
        await voting.getRoot(),
        dataToVerify.formattedProof,
        0
      );
    });

    it("should vote against proposal", async () => {
      const dataToVerify = await getZKP(voting, pair, OWNER.address, proposalId.toString());

      await voting.voteOnProposal(
        proposalId,
        dataToVerify.nullifierHash,
        await voting.getRoot(),
        dataToVerify.formattedProof,
        1
      );
    });

    it("should not vote with same nullifier", async () => {
      const dataToVerify = await getZKP(voting, pair, OWNER.address, proposalId.toString());

      await voting.voteOnProposal(
        proposalId,
        dataToVerify.nullifierHash,
        await voting.getRoot(),
        dataToVerify.formattedProof,
        0
      );
      await expect(
        voting.voteOnProposal(
          proposalId,
          dataToVerify.nullifierHash,
          await voting.getRoot(),
          dataToVerify.formattedProof,
          0
        )
      ).to.be.revertedWith("Voting: nullifier already exists");
    });

    it("should not vote with wrong root", async () => {
      const dataToVerify = await getZKP(voting, pair, OWNER.address, proposalId.toString());

      await expect(
        voting.voteOnProposal(proposalId, dataToVerify.nullifierHash, ethers.ZeroHash, dataToVerify.formattedProof, 0)
      ).to.be.revertedWith("Voting: root does not exist");
    });

    it("should not vote with wrong proof", async () => {
      const dataToVerify = await getZKP(voting, pair, USER1.address, proposalId.toString());

      await expect(
        voting.voteOnProposal(
          proposalId,
          dataToVerify.nullifierHash,
          await voting.getRoot(),
          dataToVerify.formattedProof,
          0
        )
      ).to.be.revertedWith("Voting: Invalid vote proof");
    });

    it("should not vote before commitment period", async () => {
      const localProposalId = 2;

      let newPair: CommitmentFields;

      [newPair] = await prepareForVoting(localProposalId, ["remark", DEFAULT_DATA, "0x"], false);

      const dataToVerify = await getZKP(voting, newPair, OWNER.address, localProposalId.toString());

      await expect(
        voting.voteOnProposal(
          localProposalId,
          dataToVerify.nullifierHash,
          await voting.getRoot(),
          dataToVerify.formattedProof,
          0
        )
      ).to.be.revertedWith("Voting: status is not VOTING");
    });

    it("should not vote after voting period", async () => {
      const dataToVerify = await getZKP(voting, pair, OWNER.address, proposalId.toString());

      await time.increase(101);

      await expect(
        voting.voteOnProposal(
          proposalId,
          dataToVerify.nullifierHash,
          await voting.getRoot(),
          dataToVerify.formattedProof,
          0
        )
      ).to.be.revertedWith("Voting: status is not VOTING");
    });
  });

  describe("#execution", () => {
    let proposalId: number = 1;

    let pair: CommitmentFields;
    let commitment: string;

    beforeEach(async () => {
      const proposalData = {
        commitmentPeriod: 100n,
        votingPeriod: 100n,
        proposalExecutionPeriod: 100n,
        requiredQuorum: 100n,
        requiredMajority: 100n,
      };

      const votingInterface = Voting__factory.createInterface();

      [pair, commitment] = await prepareForVoting(proposalId, [
        "remark",
        proposalData,
        votingInterface.encodeFunctionData("distributeFunds", [USER1.address, ethers.parseEther("1").toString()]),
      ]);

      const dataToVerify = await getZKP(voting, pair, OWNER.address, proposalId.toString());

      await voting.voteOnProposal(
        proposalId,
        dataToVerify.nullifierHash,
        await voting.getRoot(),
        dataToVerify.formattedProof,
        1
      );

      await time.increase(101);
    });

    it("should execute proposal", async () => {
      const userBalanceBefore = await ethers.provider.getBalance(USER1.address);

      await voting.executeProposal(proposalId);

      const userBalanceAfter = await ethers.provider.getBalance(USER1.address);

      expect(userBalanceAfter - userBalanceBefore).to.equal(ethers.parseEther("1"));
    });

    it("should not execute proposal before voting period ends", async () => {
      const proposalData = {
        commitmentPeriod: 100n,
        votingPeriod: 100n,
        proposalExecutionPeriod: 100n,
        requiredQuorum: 100n,
        requiredMajority: 100n,
      };

      const votingInterface = Voting__factory.createInterface();

      await voting.createProposal(
        "remark",
        proposalData,
        votingInterface.encodeFunctionData("distributeFunds", [USER1.address, ethers.parseEther("1").toString()])
      );

      const localProposalId = 2;

      await expect(voting.executeProposal(localProposalId)).to.be.revertedWith("Voting: status is not EXECUTION");
    });

    it("should not execute proposal twice", async () => {
      await voting.executeProposal(proposalId);

      await expect(voting.executeProposal(proposalId)).to.be.revertedWith("Voting: status is not EXECUTION");
    });

    it("should not execute proposal with less than required majority", async () => {
      const proposalData = {
        commitmentPeriod: 100n,
        votingPeriod: 100n,
        proposalExecutionPeriod: 100n,
        requiredQuorum: 100n,
        requiredMajority: 100n,
      };

      const votingInterface = Voting__factory.createInterface();

      const localProposalId = 2;

      let [newPair, ,] = await prepareForVoting(localProposalId, [
        "remark",
        proposalData,
        votingInterface.encodeFunctionData("distributeFunds", [USER1.address, ethers.parseEther("1").toString()]),
      ]);

      const dataToVerify = await getZKP(voting, newPair, OWNER.address, localProposalId.toString());

      await voting.voteOnProposal(
        localProposalId,
        dataToVerify.nullifierHash,
        await voting.getRoot(),
        dataToVerify.formattedProof,
        0
      );

      await time.increase(101);

      await expect(voting.executeProposal(localProposalId)).to.be.revertedWith("Voting: status is not EXECUTION");
    });

    it("should not execute proposal with less than required quorum", async () => {
      const proposalData = {
        commitmentPeriod: 100n,
        votingPeriod: 100n,
        proposalExecutionPeriod: 100n,
        requiredQuorum: 100n,
        requiredMajority: 100n,
      };

      const votingInterface = Voting__factory.createInterface();

      const localProposalId = 2;

      [,] = await prepareForVoting(localProposalId, [
        "remark",
        proposalData,
        votingInterface.encodeFunctionData("distributeFunds", [USER1.address, ethers.parseEther("1").toString()]),
      ]);

      await time.increase(100);

      await expect(voting.executeProposal(localProposalId)).to.be.revertedWith("Voting: status is not EXECUTION");
    });

    it("should not execute proposal if it fails", async () => {
      const proposalData = {
        commitmentPeriod: 100n,
        votingPeriod: 100n,
        proposalExecutionPeriod: 100n,
        requiredQuorum: 100n,
        requiredMajority: 100n,
      };

      const votingInterface = Voting__factory.createInterface();

      const localProposalId = 2;

      let newPair;

      [newPair] = await prepareForVoting(localProposalId, [
        "remark",
        proposalData,
        votingInterface.encodeFunctionData("distributeFunds", [USER1.address, ethers.parseEther("10").toString()]),
      ]);

      const dataToVerify = await getZKP(voting, newPair, OWNER.address, localProposalId.toString());

      await voting.voteOnProposal(
        localProposalId,
        dataToVerify.nullifierHash,
        await voting.getRoot(),
        dataToVerify.formattedProof,
        1
      );

      await time.increase(101);

      await expect(voting.executeProposal(localProposalId)).to.be.revertedWith("Voting: proposal execution failed");
    });

    it("should execute proposal with zero calldata", async () => {
      const proposalData = {
        commitmentPeriod: 100n,
        votingPeriod: 100n,
        proposalExecutionPeriod: 100n,
        requiredQuorum: 100n,
        requiredMajority: 100n,
      };

      const localProposalId = 2;

      let newPair;
      [newPair, ,] = await prepareForVoting(localProposalId, ["remark", proposalData, "0x"]);

      const dataToVerify = await getZKP(voting, newPair, OWNER.address, localProposalId.toString());

      await voting.voteOnProposal(
        localProposalId,
        dataToVerify.nullifierHash,
        await voting.getRoot(),
        dataToVerify.formattedProof,
        1
      );

      await time.increase(101);

      await expect(voting.executeProposal(localProposalId)).to.not.be.reverted;
    });

    it("should not execute proposal with zero commitments", async () => {
      const proposalData = {
        commitmentPeriod: 100n,
        votingPeriod: 100n,
        proposalExecutionPeriod: 100n,
        requiredQuorum: 100n,
        requiredMajority: 100n,
      };

      await voting.createProposal("remark", proposalData, "0x");

      const localProposalId = 2;

      await time.increase(201);

      await expect(voting.executeProposal(localProposalId)).to.be.revertedWith("Voting: status is not EXECUTION");
    });
  });

  describe("#getters", () => {
    const proposalId: number = 1;

    beforeEach(async () => {
      const proposalData = {
        commitmentPeriod: 100n,
        votingPeriod: 100n,
        proposalExecutionPeriod: 100n,
        requiredQuorum: 100n,
        requiredMajority: 100n,
      };

      const votingInterface = Voting__factory.createInterface();

      let [pair, ,] = await prepareForVoting(proposalId, [
        "remark",
        proposalData,
        votingInterface.encodeFunctionData("distributeFunds", [USER1.address, ethers.parseEther("1").toString()]),
      ]);

      const dataToVerify = await getZKP(voting, pair, OWNER.address, proposalId.toString());

      await voting.voteOnProposal(
        proposalId,
        dataToVerify.nullifierHash,
        await voting.getRoot(),
        dataToVerify.formattedProof,
        1
      );

      await time.increase(101);
    });

    it("should return correct statuses", async () => {
      expect(await voting.getProposalStatus(0)).to.be.equal(0n, "Should be None");

      await time.increase(101);

      expect(await voting.getProposalStatus(proposalId)).to.be.equal(4n, "Should be Rejected");
    });
  });
});
