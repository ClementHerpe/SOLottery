import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { Soloterie } from "../target/types/soloterie";

describe("soloterie", () => {
  // Configure le provider
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.soloterie as Program<Soloterie>; 

  it("Initialise la loterie avec paramètres valides", async () => {
    const lottery = anchor.web3.Keypair.generate();

    const maxSol = new anchor.BN(anchor.web3.LAMPORTS_PER_SOL * 2); // 2 SOL
    const ticketPrice = new anchor.BN(anchor.web3.LAMPORTS_PER_SOL / 10); // 0.1 SOL

    await program.methods
      .initialize(maxSol, ticketPrice)
      .accounts({
        lottery: lottery.publicKey,
        user: provider.wallet.publicKey,
      })
      .signers([lottery])
      .rpc();

    const state = await program.account.lottery.fetch(lottery.publicKey);

    console.log("Nombre de tickets :", state.nbTickets.toString());

    const expectedTickets = maxSol.div(ticketPrice);
    assert.strictEqual(
      state.nbTickets.toString(),
      expectedTickets.toString(),
      "Le nombre de tickets est incorrect"
    );
  });
});
