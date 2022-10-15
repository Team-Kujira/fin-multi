# Fin Multi

A basic contract designed to support consolidation of staking "dust" into a single asset, via FIN Market Swaps.

`ExecuteMsg` requires a `Vec<Vec<(Addr, Denom)>>`, which is stepped through (right-to-left), and the full contract balance of the offer Denom is swapped. This allows the total returned from a previous stage to be swapped in a latter stage, without having to know the return amount in advance.

After all stages are complete, the total contract balance is returned to the sender.

## Deployments

Testnet: code_id 167

`kujira1leuud33wer0hcxyskldutxc4agnum3uvdvdfl38tllhx4f6dtuhsqyatf8`

Mainnet: code_id 26

`kujira1a7pjjyvng22a8msatp4zj6ut9tmsd9qvp26gaj7tnrjrqtx7yafqk8zsqv`
