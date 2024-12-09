import * as sb from "@switchboard-xyz/on-demand";
// import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

import { SwitchboardDemo } from "../target/types/switchboard_demo";

const main = async () => {
  const { keypair, connection } = await sb.AnchorUtils.loadEnv();
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SwitchboardDemo as Program<SwitchboardDemo>;

  it("test", async () => {
    const feed = new anchor.web3.PublicKey("HvMrsyD5p1Jg7PTCkLq3bkb5Hs1r3ToYex3ixZ1Mq47A");
    const feedAccount = new sb.PullFeed(program, feed);

    const [pullIx, _responses, _success, luts] = await feedAccount.fetchUpdateIx();
    const myIx = await program.methods.test().accounts({ feed }).instruction();

    const tx = await sb.asV0Tx({
      connection: provider.connection,
      ixs: [pullIx, myIx],
      signers: [keypair],
      computeUnitPrice: 200_000,
      computeUnitLimitMultiple: 1.3,
      lookupTables: luts,
    });

    const simulateResult = await program.provider.connection.simulateTransaction(tx, {
      commitment: "processed",
    });

    // const sig = await connection.sendTransaction(tx, [keypair], { commitment: "processed" });
  });
};

main();
