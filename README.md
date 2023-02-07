# Fin Multi

A basic contract designed to support consolidation of staking "dust" into a single asset, via FIN Market Swaps.

`ExecuteMsg` requires a `Vec<Vec<(Addr, Denom)>>`, which is stepped through (right-to-left), and the full contract balance of the offer Denom is swapped. This allows the total returned from a previous stage to be swapped in a latter stage, without having to know the return amount in advance.

After all stages are complete, the total contract balance is returned to the sender.

## Deployments

Testnet: code_id 1257

`kujira1a493s7v9a4k46e4s9gg26wnqd0au63s9af2u6hgrnayfsyprn7fqg866wj`

Mainnet: code_id 84

`kujira1a7pjjyvng22a8msatp4zj6ut9tmsd9qvp26gaj7tnrjrqtx7yafqk8zsqv`
