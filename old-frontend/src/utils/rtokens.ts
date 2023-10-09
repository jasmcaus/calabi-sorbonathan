import { ChainId } from "./chains"
import { CHAIN_ID } from "utils/chains"
import rtokens from "@lc-labs/rtokens"

export let tokenList = rtokens

export const TOKEN_STORAGE = "token_storage"

if (CHAIN_ID === ChainId.DeltaSaga) {
    tokenList = {}
}
