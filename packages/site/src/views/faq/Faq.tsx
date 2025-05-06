import React from 'react';
import { Accordion, Container, Tabs } from '@mantine/core';
import { IconPhoto, IconRobot, IconCoin, IconCircle0, IconHelp } from '@tabler/icons-react';
import { PageTitle } from '@components/PageTitle';
import { Link } from 'react-router-dom';

const Faq = () => {
    return (
        <Container>
            <PageTitle 
                title="Frequent Asked Questions"
                icon={IconHelp}
            />
            <Tabs variant="outline" defaultValue="bot">
                <Tabs.List>
                    <Tabs.Tab value="bot" icon={<IconRobot size="1rem" />}>
                        Bot
                    </Tabs.Tab>
                    <Tabs.Tab value="nft" icon={<IconPhoto size="1rem" />}>
                        MEME NFT
                    </Tabs.Tab>
                    <Tabs.Tab value="coin" icon={<IconCoin size="1rem" />}>
                        MEME Coin
                    </Tabs.Tab>
                </Tabs.List>

                <Tabs.Panel value="bot" pt="xs">
                    <Accordion defaultValue="create_meme" pt="lg">
                        <Accordion.Item value="create_meme">
                            <Accordion.Control>
                                <b><IconCircle0 size="0.8rem"/> How to create and post memes?</b>
                            </Accordion.Control>
                            <Accordion.Panel>
                                    <Container>
                                        <ol>
                                            <li>
                                                Type <b>/meme search <u><i>term</i></u></b> to search for memes.<br/>
                                                <small>eg: /meme search victory</small>
                                                <ul>
                                                    <li>
                                                        Type <b>/meme search <i>term</i> <u><i>page</i></u></b> to view results on the following pages.<br/>
                                                        <small>eg: /meme search victory 2</small>
                                                    </li>
                                                </ul>
                                            </li>
                                            <li>Use either:
                                                <ul>
                                                    <li>
                                                        <b>/meme gen <u><i>id_returned_in_step_1</i></u> <u><i>'caption1'</i></u> <u><i>'caption2'</i></u> etc</b> to generate the captions yourself.<br/>
                                                        <small>eg: /meme gen 1333 'asked a girl' 'she said maybe'</small>
                                                    </li>
                                                    <li>
                                                        <b>/meme suggest <i>id_returned_in_step_1</i> <u><i>'story mood'</i></u> <u><i>'story theme'</i></u></b> to let the AI (LLM Canister) to generate the captions for you.<br/>
                                                        <small>eg: /meme suggest 1333 funny 'how I asked a girl on a date and she said maybe'</small>
                                                    </li>
                                                </ul>
                                            </li>
                                            <li>
                                                Repeat step 2 until you are satisfied.
                                            </li>
                                            <li>
                                                Type <b>/meme post</b> to share your meme in the current group or channel.
                                            </li>
                                        </ol>
                                        <Container bg="green">
                                            <small><b>NOTE: All the interactions with the bot will not be visible to other users in the channel unless /meme post is used!</b></small>
                                        </Container>
                                    </Container>
                                </Accordion.Panel>
                        </Accordion.Item>

                        <Accordion.Item value="mint_nft">
                            <Accordion.Control>
                                <b><IconCircle0 size="0.8rem"/> How to mint a meme as a ICRC-7 NFT?</b>
                            </Accordion.Control>
                            <Accordion.Panel>
                                <Container>
                                    <ol>
                                        <li>
                                            Type <b>/meme wallet balance</b> to check your ICP balance.<br/>
                                        </li>
                                        <li>
                                            Type <b>/meme nft status</b> to check the current minting cost.<br/>
                                        </li>
                                        <li>
                                            Type <b>/meme wallet address</b> and transfer to the displayed address enough ICP to cover the minting cost.
                                        </li>
                                        <li>
                                            Type <b>/meme nft mint</b>, if you have used <b>/meme post</b> before and your meme post has at least the number of reactions shown in step 2.
                                        </li>
                                    </ol>
                                    <Container bg="red">
                                        <small><b>ALERT: The mint operation is irreversible and there’s no confirmation dialog!</b></small>
                                    </Container>
                                </Container>
                            </Accordion.Panel>
                        </Accordion.Item>

                        <Accordion.Item value="transfer_nft">
                            <Accordion.Control>
                                <b><IconCircle0 size="0.8rem"/> How to transfer your MEME NFT?</b>
                            </Accordion.Control>
                            <Accordion.Panel>
                                <Container>
                                    <ol>
                                        <li>
                                            Type <b>/meme nft tokens</b> to see all the MEME NFT's you own.
                                        </li>
                                        <li>
                                            While Open Chat doesn't support ICRC-7 NFT's, you must do the transfer from you OC user using our bot.<br/>
                                            Type <b>/meme nft transfer <u><i>nft_id_returned_in_step_1</i></u> <u><i>target_id</i></u></b> to transfer a single MEME NFT.<br/>
                                            <small>eg: /meme nft transfer 1234 agb37-diaaa-aaaac-qadyq-cai</small>
                                        </li>
                                    </ol>
                                    <Container bg="red">
                                        <small><b>ALERT: The transfer operation is irreversible and there’s no confirmation dialog!</b></small>
                                    </Container>
                                </Container>
                            </Accordion.Panel>
                        </Accordion.Item>

                        <Accordion.Item value="withdraw_icp">
                            <Accordion.Control>
                                <b><IconCircle0 size="0.8rem"/> How to withdraw ICP from my wallet?</b>
                            </Accordion.Control>
                            <Accordion.Panel>
                                <Container>
                                    <ol>
                                        <li>
                                            Type <b>/meme wallet balance</b> to see your current ICP balance.
                                        </li>
                                        <li>
                                            Type <b>/meme wallet withdraw <u><i>amount_in_decimal_notation</i></u> <u><i>target_account_id</i></u></b> to transfer your ICP.<br/>
                                            <small>eg: /meme wallet withdraw 1.75 dfebcce104959959adaab0444fd2fe8555300d0d301e07fc3ea0fec779c1c736</small>
                                        </li>
                                    </ol>
                                    <Container bg="red">
                                        <small><b>ALERT: The transfer operation is irreversible and there’s no confirmation dialog!</b></small>
                                    </Container>
                                </Container>
                            </Accordion.Panel>
                        </Accordion.Item>
                    </Accordion>
                </Tabs.Panel>

                <Tabs.Panel value="nft" pt="xs">
                    <Accordion defaultValue="status" pt="lg">
                        <Accordion.Item value="status">
                            <Accordion.Control>
                                <b><IconCircle0 size="0.8rem"/> How to check the NFT collection status?</b>
                            </Accordion.Control>
                            <Accordion.Panel>
                                <Container>
                                    <ol>
                                        <li>
                                            Type <b>/meme nft status</b> to display the current minting cost, total and max supply, contract address etc.<br/>
                                        </li>
                                    </ol>
                                </Container>
                            </Accordion.Panel>
                        </Accordion.Item>
                        <Accordion.Item value="standards">
                            <Accordion.Control>
                                <b><IconCircle0 size="0.8rem"/> Which standards are supported by MEME NFT?</b>
                            </Accordion.Control>
                            <Accordion.Panel>
                                <Container>
                                    <ol>
                                        <li>
                                            Meme NFT supports:<br/>
                                            <ul>
                                                <li><Link to="https://github.com/dfinity/ICRC/blob/icrc_7_and_37/ICRCs/ICRC-7/ICRC-7.md" target='blank'>ICRC-7</Link> <small>Minimal Non-Fungible Token (NFT) Standard</small></li>
                                                <li><Link to="https://github.com/dfinity/ICRC/blob/icrc_7_and_37/ICRCs/ICRC-37/ICRC-37.md" target='blank'>ICRC-37</Link> <small>Approval Support for the Minimal Non-Fungible Token (NFT) Standard</small></li>
                                                <li><Link to="https://github.com/dfinity/ICRC-1/blob/main/standards/ICRC-3/README.md" target='blank'>ICRC-3</Link> <small>Block Log</small></li>
                                            </ul>
                                        </li>
                                    </ol>
                                </Container>
                            </Accordion.Panel>
                        </Accordion.Item>
                        <Accordion.Item value="requirements">
                            <Accordion.Control>
                                <b><IconCircle0 size="0.8rem"/> Which are the requirements to mint a NFT?</b>
                            </Accordion.Control>
                            <Accordion.Panel>
                                <Container>
                                    <ol>
                                        <li>
                                            The post must have at least the number of reactions, from Open Chat users with proof of humanity, returned by <small><b>/meme nft status</b></small>.
                                        </li>
                                        <li>
                                            The channel/group where the meme was posted must have at least <b>50</b> members.
                                        </li>
                                        <li>
                                            The balance in your Mementor wallet must be enough to cover the minting cost, returned also by <small><b>/meme nft status</b></small>.
                                        </li>
                                        <li>
                                            The MEME NFT max supply must be bellow the total supply. That information is also returned by <small><b>/meme nft status</b></small>.
                                        </li>
                                        <li>
                                            Your meme <b>must be unique</b> - that is, there must not be an existing minted NFT with the same background image and identical captions.
                                        </li>
                                    </ol>
                                </Container>
                            </Accordion.Panel>
                        </Accordion.Item>
                    </Accordion>
                </Tabs.Panel>

                <Tabs.Panel value="coin" pt="xs">
                    Soon...
                </Tabs.Panel>
            </Tabs>
        </Container>
    );
};

export default Faq;