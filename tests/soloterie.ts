import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { Soloterie } from "../target/types/soloterie";

//1 : TEST INIT - OK
/**
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
*/

//2 : TEST BUY TICKET
describe("soloterie test achat ", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.soloterie as Program<Soloterie>;

  it("Permet l'achat d'un ticket", async () => {
    const lottery = anchor.web3.Keypair.generate();

    const maxSol = new anchor.BN(anchor.web3.LAMPORTS_PER_SOL * 2); // 2 SOL
    const ticketPrice = new anchor.BN(anchor.web3.LAMPORTS_PER_SOL / 10); // 0.1 SOL

    // 1. Initialisation de la loterie
    await program.methods
      .initialize(maxSol, ticketPrice)
      .accounts({
        lottery: lottery.publicKey,
        user: provider.wallet.publicKey,
      })
      .signers([lottery])
      .rpc();

    const ticketsNumberRAW = 3;
    const ticketsNumber = new anchor.BN(ticketsNumberRAW);
    // 2. Achat d'un ticket
    await program.methods
      .buyTicket(ticketsNumber)
      .accounts({
        lottery: lottery.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc();

    // 3. Vérification : le nombre de tickets vendus doit être ticketsNumber
    const state = await program.account.lottery.fetch(lottery.publicKey);

    console.log("Nombre de tickets :", state.nbTickets.toString());

    console.log("Tickets vendus :", state.ticketsSold.toString());

    assert.strictEqual(
      state.ticketsSold.toNumber(),
      ticketsNumberRAW,
      "Le nombre de tickets est incorrect"
    );
  });
});