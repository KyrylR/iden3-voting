import { expect } from "chai";
import { ethers } from "hardhat";

import { MerkleTree } from "merkletreejs";

import {
  getPoseidon,
  poseidonHash,
  buildSparseMerkleTree,
  getBytes32ElementHash,
  getBytes32PoseidonHash,
  getRoot,
  Reverter,
} from "@test-helpers";

import { IncrementalMerkleTree, PoseidonIMT } from "@ethers-v6";

describe("IncrementalMerkleTree", () => {
  const reverter = new Reverter();

  let poseidonIMT: PoseidonIMT;
  let merkleTree: IncrementalMerkleTree;

  let localMerkleTree: MerkleTree;
  let poseidonMerkleTree: MerkleTree;

  let treeHeight = 0n;
  let poseidonTreeHeight = 6n;

  before(async () => {
    const incrementalMerkleTree = await ethers.getContractFactory("IncrementalMerkleTree");
    merkleTree = await incrementalMerkleTree.deploy(treeHeight);

    localMerkleTree = buildSparseMerkleTree(ethers.keccak256, [], treeHeight);
    poseidonMerkleTree = buildSparseMerkleTree(poseidonHash, [], poseidonTreeHeight);

    const poseidonIMTFactory = await ethers.getContractFactory("PoseidonIMT", {
      libraries: {
        PoseidonUnit1L: await (await getPoseidon(1)).getAddress(),
        PoseidonUnit2L: await (await getPoseidon(2)).getAddress(),
      },
    });

    poseidonIMT = await poseidonIMTFactory.deploy(poseidonTreeHeight);

    await reverter.snapshot();
  });

  afterEach(reverter.revert);

  describe("Basic IMT", () => {
    it("should add element to tree", async () => {
      const element = ethers.toBeHex("0x1234", 32);

      await merkleTree.add(element);

      const elementHash = getBytes32ElementHash("0x1234");

      localMerkleTree = buildSparseMerkleTree(ethers.keccak256, [elementHash], await merkleTree.getHeight());

      expect(await merkleTree.getRoot()).to.eq(getRoot(localMerkleTree));

      expect(await merkleTree.getLength()).to.eq(1);
    });

    it("should add elements to tree", async () => {
      const elements = [];

      for (let i = 1; i < 33; i++) {
        const element = ethers.toBeHex(`0x${i}234`, 32);

        await merkleTree.add(element);

        const elementHash = getBytes32ElementHash(element);

        elements.push(elementHash);

        localMerkleTree = buildSparseMerkleTree(ethers.keccak256, elements, await merkleTree.getHeight());

        expect(await merkleTree.getRoot()).to.eq(getRoot(localMerkleTree));

        expect(await merkleTree.getLength()).to.eq(i);
      }
    });

    it("should return zeroHash if tree is empty", async () => {
      expect(await merkleTree.getRoot()).to.eq(getRoot(localMerkleTree));
    });
  });

  describe("Poseidon IMT", () => {
    it("should add element to tree", async () => {
      const element = ethers.toBeHex("0x1234", 32);

      await poseidonIMT.add(element);

      const elementHash = getBytes32PoseidonHash("0x1234");

      poseidonMerkleTree = buildSparseMerkleTree(poseidonHash, [elementHash], await poseidonIMT.getHeight());

      expect(await poseidonIMT.getRoot()).to.eq(getRoot(poseidonMerkleTree));

      expect(await poseidonIMT.getLength()).to.eq(1);
    });

    it("should add elements to tree", async () => {
      const elements = [];

      for (let i = 1; i < 33; i++) {
        const element = ethers.toBeHex(`0x${i}234`, 32);

        await poseidonIMT.add(element);

        const elementHash = getBytes32PoseidonHash(element);

        elements.push(elementHash);

        poseidonMerkleTree = buildSparseMerkleTree(poseidonHash, elements, await poseidonIMT.getHeight());

        expect(await poseidonIMT.getRoot()).to.eq(getRoot(poseidonMerkleTree));

        expect(await poseidonIMT.getLength()).to.eq(i);
      }
    });

    it("should return zeroHash if tree is empty", async () => {
      expect(await poseidonIMT.getRoot()).to.eq(getRoot(poseidonMerkleTree));
    });
  });
});
