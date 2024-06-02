import { Deployer, Reporter } from "@solarity/hardhat-migrate";

import { Voting__factory, Groth16Verifier__factory } from "@ethers-v6";

import { deployPoseidons } from "@/deploy/helpers";

export = async (deployer: Deployer) => {
  const verifier = await deployer.deploy(Groth16Verifier__factory);

  await deployPoseidons(deployer, [1, 2, 3]);

  const voting = await deployer.deploy(Voting__factory, [80, await verifier.getAddress()]);

  Reporter.reportContracts(["Verifier", await verifier.getAddress()], ["Voting", await voting.getAddress()]);
};
