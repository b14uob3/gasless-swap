import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { GaslessSwap } from "../target/types/gasless_swap";

describe("gasless-swap", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.GaslessSwap as Program<GaslessSwap>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
