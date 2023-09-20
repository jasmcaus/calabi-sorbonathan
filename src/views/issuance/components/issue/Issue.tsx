import { t, Trans } from "@lingui/macro"
import { useWeb3React } from "@web3-react/core"
import { Button } from "components"
import { useRToken } from "hooks/useRToken"
import { useAtom, useAtomValue } from "jotai"
import { useSetAtom } from "jotai"
import { useState } from "react"
import { signed } from "state/web3/lib/notifications"
import { drip } from "state/web3/utils/utils"
import { Card } from "theme-ui"
import { issueAmountAtom, isValidIssuableAmountAtom, quantitiesAtom } from "views/issuance/atoms"
import ConfirmIssuance from "./ConfirmIssuance"
import IssueInput from "./IssueInput"
import MaxIssuableUpdater from "./MaxIssuableUpdater"
import QuantitiesUpdater from "./QuantitiesUpdater"

/**
 * Issuance
 */
const Issue = () => {
    const { account } = useWeb3React()

    const [amount, setAmount] = useAtom(issueAmountAtom)
    const setQuantities = useSetAtom(quantitiesAtom)
    const isValid = useAtomValue(isValidIssuableAmountAtom)
    const [issuing, setIssuing] = useState(false)

    async function handleSubmit() {
        drip(account!, +amount)
            .then(() => {
                signed("Transaction Signed", 4000)
            })
            .finally(() => {
                setIssuing(false)
            })
    }

    return (
        <>
            <QuantitiesUpdater amount={amount} onChange={setQuantities} />
            {issuing && (
                <ConfirmIssuance
                    onClose={() => {
                        setIssuing(false)
                        setAmount("")
                    }}
                />
            )}
            <Card p={4}>
                <IssueInput title={t`Drip MAV and FRA`} />
                <Button
                    sx={{ width: "100%" }}
                    disabled={!isValid || issuing}
                    variant={"primary"}
                    mt={3}
                    onClick={() => handleSubmit()}
                >
                    <Trans>+ Drip</Trans>
                </Button>
            </Card>
        </>
    )
}

export default Issue
