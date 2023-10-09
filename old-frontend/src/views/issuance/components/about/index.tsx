import { Trans } from "@lingui/macro"
import { Box, Link, Text } from "theme-ui"

const About = () => (
    <Box variant="layout.borderBox" p={4} sx={{ height: "fit-content" }}>
        <Text variant="strong" mb={2}>
            "How does this work? "
        </Text>
        <Text as="p" variant="legend">
            " Minting requires a deposit of the defined collateral tokens in equal value amounts to the RToken smart
            contracts. "
        </Text>
        <Text variant="strong" mb={2} mt={4}>
            "When will I get my RTokens?"
        </Text>
        <Text as="p" variant="legend">
            " Depending on RToken minting activity and the size of your deposit, the protocol will either issue your
            RTokens immediately or mint them over the period of a few blocks (a "slow mint"). "Slow mints" are designed
            into the protocol to ensure stability of the RToken's price and redemption rate while there are ongoing
            mints and revenue operations. "
            <br />
            <br />
            "To learn more about minting and redemption operations, read the documentation"{" "}
            <Link
                href="https://reserve.org/protocol/protocol_operations/"
                target="_blank"
                sx={{ textDecoration: "underline" }}
            >
                "here"
            </Link>
        </Text>
    </Box>
)

export default About
