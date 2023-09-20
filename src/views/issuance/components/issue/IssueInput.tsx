import { t } from "@lingui/macro"
import TransactionInput, { TransactionInputProps } from "components/transaction-input"
import { useAtomValue } from "jotai"
import { isRTokenDisabledAtom } from "state/atoms"
import { issueAmountAtom, maxIssuableAtom } from "../../atoms"

const IssueInput = (props: Partial<TransactionInputProps>) => {
    return (
        <TransactionInput
            placeholder={t`Mint amount`}
            amountAtom={issueAmountAtom}
            maxAmount={5000}
            disabled={false}
            {...props}
        />
    )
}

export default IssueInput
