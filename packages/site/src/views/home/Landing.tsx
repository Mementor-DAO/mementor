import React, { useCallback } from "react";
import { createStyles, Image, Container, Title, Button, Group, Text, List, ThemeIcon, rem, Space } from '@mantine/core';
import { IconBook, IconCheck } from '@tabler/icons-react';
import { useBrowser } from "@hooks/browser";
import logo from "@assets/logo-vert.svg";
import { config } from "../../config";
import { useUI } from "@hooks/ui";
import { Link } from "react-router-dom";
  
const useStyles = createStyles((theme) => ({
    inner: {
        display: 'flex',
        flexWrap: 'wrap',
    },

    content: {
        maxWidth: rem(480),
        marginRight: `calc(${theme.spacing.xl} * 3)`,

        [theme.fn.smallerThan('md')]: {
            marginRight: 0,
            maxWidth: rem(820),
        },
    },
  
    title: {
        color: theme.colorScheme === 'dark' ? theme.white : theme.black,
        fontFamily: `Greycliff CF, ${theme.fontFamily}`,
        fontSize: rem(42),
        lineHeight: 1.2,
        fontWeight: 900,
        textAlign: 'justify',

        [theme.fn.smallerThan('xs')]: {
            fontSize: rem(28),
        },
    },

    subtitle: {
        textAlign: 'justify',
    },

    item: {
        textAlign: 'justify',
    },
  
    control: {
        [theme.fn.smallerThan('xs')]: {
            flex: 1,
        },
    },
  
    imageContainer: {
        flex: 1,
        display: 'flex',
        justifyContent: 'center',

        [theme.fn.smallerThan('md')]: {
            marginTop: '2rem'
        },
    },

    image: {
        margin: 'auto',
    },
  
    highlight: {
        position: 'relative',
        backgroundColor: theme.fn.variant({ variant: 'light', color: theme.primaryColor }).background,
        borderRadius: theme.radius.md,
        padding: '0.5rem 1rem',
        lineHeight: '4rem',
    },
}));

const Landing = () => {
    const { classes, cx } = useStyles();
    const { classes: globClasses } = useUI();

    const handleReadWhitepaper = useCallback(() => {
        window.open(/*config.WHITEPAPER_URL*/ '', "blank");
    }, []);

    return (
        <Container>
            <div className={classes.inner}>
                <div className={classes.content}>
                    <Title className={classes.title}>
                    <span className={classes.highlight}>Create</span> memes, <span className={classes.highlight}>mint</span> them as exclusive Meme NFTs, and <span className={classes.highlight}>earn</span> Meme coins in return!
                    </Title>
                    <Text color="dimmed" mt="md" className={classes.subtitle}>
                        <b>Mementor</b> is a decentralized web3 <b><Link to="https://oc.app/" target="blank">Open Chat</Link></b> bot where users can create and post memes, mint them as NFTs, and receive coins as rewards. Read more details in our <b><Link to="/faq">FAQ</Link></b>!
                    </Text>

                    <List
                        mt={30}
                        spacing="sm"
                        size="sm"
                        icon={
                            <ThemeIcon size={20} radius="xl">
                                <IconCheck size={rem(12)} stroke={1.5} />
                            </ThemeIcon>
                        }
                    >
                        <List.Item className={classes.item}>
                            <b>Memes</b> are created by users through our Open Chat bot, choosing from <b>over 3,000 templates</b>. Users can <b>utilize AI</b> to generate captions for their memes.
                        </List.Item>
                        <List.Item className={classes.item}>
                            A meme can then be <b>minted as a NFT</b> directly through our bot. All content is <b>stored on-chain</b>. Duplicate memes cannot be minted again.
                        </List.Item>
                        <List.Item className={classes.item}>
                            The <b>Meme Coin blockchain</b> follows <b>Bitcoin</b>'s model, creating a new block every 15 minutes, with the block reward <b>halved</b> approximately every 9.5 months. The maximum supply is <b>capped at 21 million</b>. The <b>top 3 NFTs</b> with most reactions, minted during the block interval, will <b>earn Meme coins</b> as reward. Block rewards will accumulate if no Meme NFTs are minted.
                        </List.Item>
                        <List.Item className={classes.item}>
                            Only Open Chat users who have <b>verified their uniqueness</b> can mint NFTs or have their reactions to meme posts counted, <b>ensuring that no bots can rig</b> the Meme Coin distribution.
                        </List.Item>
                        <List.Item className={classes.item}>
                             Mementor is <b>open-source</b> and will be controlled by an <b>SNS Decentralized Autonomous Organization (DAO)</b> in the future. 
                        </List.Item>
                    </List>

                    <Group mt={30}>
                        <Link to="/whitepaper">
                            <Button 
                                variant="default" 
                                bg="green"
                                radius="xl" 
                                size="md" 
                                className={cx(classes.control, {[globClasses.animatedOutline]: true})}
                            >
                                <IconBook size="1rem" />&nbsp;Read our whitepaper
                            </Button>
                        </Link>
                    </Group>
                </div>

                <div className={classes.imageContainer}>
                    <Image src={logo} className={classes.image} />
                </div>
            </div>

            <Space h="4rem" />

            <div>
            </div>
        </Container>
    );
};

export default Landing;