import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { BN } from "bn.js";
import { CrowdfundingPlatform } from "../target/types/crowdfunding_platform";

describe("crowdfunding-platform",  () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CrowdfundingPlatform as Program<CrowdfundingPlatform>;
  it("creates campaign!", async () => {

    const publicKey = anchor.AnchorProvider.local().wallet.publicKey;
    const [campaignPDA] = await anchor.web3.PublicKey.findProgramAddress([
      utf8.encode('Campaign'),
      publicKey.toBuffer(),
      ],
      program.programId
    );
    console.log("campaignPDA", campaignPDA);
    // Add your test here.
    const tx = await program.methods.createCampaign("Cihan Campaign", new BN(100))
    .accounts({
      campaign: campaignPDA,
      user: publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();

    const acc = program.account.campaign.fetch(campaignPDA);
    console.log(acc);

    console.log("Your transaction signature", tx);
  });

  it("donates to campaign",async () => {

  });



});
