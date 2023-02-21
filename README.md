# Proactive Refresh for BLS Threshold Signatures 

Based on work by [Boneh, Partap, and Rotem](https://eprint.iacr.org/2022/1656.pdf). 

## Motivation 
Threshold signatures secure billions in assets across crypto and traditional finance. They're meant to increase vault security by splitting up a digital signature across N parties, requiring at least T of them to sign off before any action (e.g. "liquidate funds" or "pay contractor") is taken. Though adversaries now need to compromise at least T parties to exploit, vanilla schemes come with a significant drawback. Signature shares are stagnant, so adversaries can target the T parties over the course of months, eventually resulting in a hack regardless of the threshold parameter. 

It's a common occurrence. The Ronin Bridge was secured by a 5-of-9 multisig. Adversaries compromised keys one at a time to drain the contract of $650M. The Harmony Bridge was secured by a 2-of-5 multisig. Again, adversaries compromised two keys separately for a $100M exploit. 

## What
Proactive refresh is a promising solution to these vulnerabilities. It's a way to renew signature shares every 30 seconds. Think of it as Google Authenticator for threshold signatures. Adversaries would then need to compromise all T keys (eg. all 5 keys for Ronin) within the span of 30 seconds- a much harder task. 

A simple concept, but building it to be cryptographically secure is fairly challenging. In fact, a valid construction was only recently proposed in a 2022 theory paper coming out of the Boneh laboratory. It satisfied many of the challenging requirements, namely: 1) having no central point of failure, 2) preserving the original vault, and 3) tracing back to signers. The scheme does so while providing security guarantees of unforgeability and accountability. 

We built out this primitive from scratch (first team to do so, to our knowledge). To demonstrate how powerful it is, we deployed a cross-chain vault on ZetaChain that's secured by a 5-of-7 threshold signature. Any adversary that wishes to drain the vault must compromise 5 signature shares in the span of 30 seconds. We think this would be unlikely. 

## How
The scheme is instantiated by sampling N secret keys from the base field of the BLS12-381 curve. One key is handed to each party via a secure communication channel. T parties can then come together, each individually signing the message to create T signatures. We use lagrange interpolation to then find the lowest degree polynomial that "fits" to these signatures, with f(0) for the polynomial set as the collective signature. Notice that each set of T signers has a unique collective signature, allowing us trace back to who signed. Accountability! A big deal when dealing with mission-critical requests. 

To verify these signatures, we can again fit the lowest degree polynomial over the public keys for these parties. Due to the handy linearity property of BLS signatures (tracing back to the linearity of pairings over elliptic curves), this collective public key verifies the signature as expected. 

That's how we implement accountable threshold BLS signatures. Now for proactive refresh. Every 30 seconds, all N parties are required to sample a new T-1 degree polynomial that passes through the origin. Every party then needs to add the corresponding point from all N-1 other polynomials to their local secret key. This completely changes what they secret key is, but notice that since all the new polynomials pass through the origin, all collective signatures at f(0) are still the same (read: vault kept intact). What's more, each party contributed equally to the update process, so there's no single point of failure. A key that is compromised in one update round by an adversary is now useless in the following updated round since the joint polynomial is completely different. Mission accomplished. 

To demonstrate how powerful this primitive is, we also deployed our own vault on ZetaChain off of a Gnosis Safe fork. [TODO: Nate adds implementation details] 

## Challenges
Implementing the scheme was pretty difficult. It's elegant and doable to understand from a high level, but the nitty gritty lost us a good bit of sleep:

1) Existing libraries that had components that we needed had group elements that didn't play well together, so we had to implement the primitive from scratch. Our rust implementation included BLS signatures (elliptic curve pairings and group operations on BLS12-381), accountability (variant of Shamir's secret sharing scheme with lagrange interpolation), and proactive refresh (sampling these strategic polynomials over the correct field).

2) Needed zkSNARKs. ZetaChain (& EVM) doesn't have the BLS precompile, so verifying the signature on-chain would've costed an ungodly amount of gas. We remedied this by instead sending a SNARK proof on-chain and verifying that. In other words, instead of checking the BLS signature on chain, we check it in a circuit and verify the proof on chain. This was a hefty engineering lift since the computational model in circuits fairly different than what we did in rust. It was especially difficult because we needed to do complex field arithmetic (pairings) that doesn't play well with the field that bn128 (the main zkSNARK elliptic curve) provides. We built this on the circom/snarkjs stack and used groth16 as our proving system. NOTE: The correct circuit is still compiling here, so the vault we have live right now can be spoofed by a well-informed adversary. 

3) [TODO: Nate talks about challenges with deploying to ZetaChain]

## Summary of Contributions 
1) Rust crate that's the first working example of proactive refresh for accountable threshold signatures. 

2) Cross-chain vault on ZetaChain that we're willing to put money on (maybe?). 

3) ZK proofs that solidified the end-to-end implementation from primitive to dapp. 

## What's next
Increased security for this vaults is one among many threshold-related use cases such as bridging, timelock encryption, and commitment pools. We're excited to see what projects spawn from proactive-refresh enabled accountable threshold signatures. Cleaning up the code now and fleshing out tests, then will make the crate available for the open source community! 
