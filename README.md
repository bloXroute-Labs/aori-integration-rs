# aori-integration-rs
This project demonstrates how we can integrate Aori api to the Bloxroute intent gateway.

## The Intent Flow
* What does dapp_main.rs do?
  1. subscribes to order book updates coming from Aori and conver them to intents before sending them to the gateway
  2. subscribes to intent solutions coming from the gateway and converts them to taker orders before sending them to Aori


* What does the solver_main.rs do?
  1. subscribes to the intent gateway, reads the make orders coming from gateway as intent, creates a take order for each and submits them 
  back to the gateway as intent solutions

# How to run locally?
* There are two programs in this repo, `dapp_main.rs`, and `solver_main.rs` which simulate what a dapp and a solver do.
* in addition to running those, we need to have a running gateway instance as well,

# Environment variables
* we need an .env file in the root directory of the project with the following env variables
```
    # USED BY DAPP AND SOLVER 
    AORI_API_ENDPOINT="wss://staging.api.aori.io/"
    AORI_FEED_ENDPOINT="wss://staging.feed.aori.io/"
    GATEWAY_URL="http://localhost:5005"
    AUTH_HEADER="....."
    BUNDLER_PK="89df9c8c1c62bec4eaf8914ebe7bbab6f440f0b4d1................"
    BUNDLER_PUBLIC_KEY="0x6D2B5eA0212c4C32A92C6faB09081e2B4AAD9558"
    SOLVER_PK="b03c6b5dc8d58d3de283000bbdbb29576b15810...................."
    SOLVER_PUBLIC_KEY="0xC56dd8290b57108ca9c25B1b6f858e676BeCeA1b"
    
    # USED BY AORI PROVIDER
    WALLET_ADDRESS="0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC"     # this must be the same as SOLVER_PUBLIC_KEY
    PRIVATE_KEY="b03c6b5dc8d58d3de283000bbdbb29576b158............" # this must be the same as SOLVER_PK
    NODE_URL="ws://3.238.30.234:8546"
```