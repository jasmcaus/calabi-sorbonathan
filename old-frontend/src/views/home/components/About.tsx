import { Trans } from "@lingui/macro"
import { SmallButton } from "components/button"
import { useNavigate } from "react-router-dom"
import { Box, BoxProps, Grid, Link, Text } from "theme-ui"
import { ROUTES } from "utils/constants"

/**
 * Section: Home > About footer
 */
const About = (props: BoxProps) => {
    const navigate = useNavigate()

    const handleDeploy = () => {
        navigate(ROUTES.DEPLOY)
        document.getElementById("app-container")?.scrollTo(0, 0)
    }

    return (
        <Box {...props}>
            <Grid columns={[1, 1, 2]} mt={6} pl={4} gap={[4, 4, 7]}>
                <Box>
                    <Text mb={2} variant="strong">
                        "RTokens & Deploying your own"
                    </Text>
                    <Text variant="legend" as="p">
                        " The creation of new RToken designs is permissionless. If you are the inventive type and have
                        ideas for what assets should be in the basket, what a good governance looks like, or anything
                        novel that could work within the realms of the protocol, please consider putting those ideas
                        into practice or sharing them with the community. "
                    </Text>
                    <SmallButton py={2} mt={3} mb={4} onClick={handleDeploy}>
                        "Deploy RToken"
                    </SmallButton>
                    <Text mb={2} variant="strong">
                        "The Reserve Project"
                    </Text>
                    <Text variant="legend" as="p">
                        " Reserve aims to help people around the world maintain their spending power by allowing anyone
                        to create asset-backed currencies with tokenized assets on the Ethereum blockchain in
                        customizable and novel ways. Read more here in "{" "}
                        <Link sx={{ textDecoration: "underline" }} href="https://reserve.org/protocol/" target="_blank">
                            "Reserve's documentation."
                        </Link>
                    </Text>
                </Box>
                <Box>
                    <Text mb={2} variant="strong">
                        "This app"
                    </Text>
                    <Text variant="legend" as="p" mb={2}>
                        " Register is developed and maintained by LC Labs as the first dApp to interact with the Reserve
                        Protocol and various RTokens deployed with the platform. "
                    </Text>
                    <Text variant="legend" as="p" mb={4}>
                        " If an RToken is listed on Register, it doesn't mean that Reserve or LC Labs endorses the
                        safety or risk levels of the RToken. LC Labs requires Github requests with additional
                        information beyond what is available on the blockchain to give users relevant data to make
                        informed decisions. As a user, please evaluate any new RToken carefully before holding or
                        staking your RSR on them. "
                    </Text>
                    <Text mb={2} variant="strong">
                        "User tracking"
                    </Text>
                    <Text variant="legend" as="p" mb={2}>
                        " LC Labs uses industry standard anonymized analytics tools to understand usage and improve the
                        user experience. LC labs does not collect any information about users or their financial
                        activity. "
                    </Text>
                    <Text variant="legend" as="p">
                        " Please keep in mind that interactions with the Ethereum blockchain are pseudonymous and
                        publicly available. "
                    </Text>
                </Box>
            </Grid>
        </Box>
    )
}

export default About
