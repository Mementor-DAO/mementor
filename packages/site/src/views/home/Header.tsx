import React from 'react';
import { ActionIcon, Box, Burger, Divider, Drawer, Group, Header, ScrollArea, createStyles, rem, useMantineColorScheme } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { IconCoin, IconMoonStars, IconPhoto, IconSun } from '@tabler/icons-react';
import logo from "@assets/logo-horz.svg";
import { Link } from 'react-router-dom';

const useStyles = createStyles((theme) => ({
    header: {
    },
    
    link: {
        display: 'flex',
        alignItems: 'center',
        height: '100%',
        paddingLeft: theme.spacing.md,
        paddingRight: theme.spacing.md,
        textDecoration: 'none',
        color: theme.colorScheme === 'dark' ? theme.white : theme.black,
        fontWeight: 500,
        fontSize: theme.fontSizes.sm,
    
        [theme.fn.smallerThan('sm')]: {
            height: rem(42),
            display: 'flex',
            alignItems: 'center',
            width: '100%',
        },
    
        ...theme.fn.hover({
            backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[6] : theme.colors.gray[0],
        }),
    },
  
    hiddenMobile: {
        [theme.fn.smallerThan('sm')]: {
            display: 'none',
        },
    },
  
    hiddenDesktop: {
        [theme.fn.largerThan('sm')]: {
            display: 'none',
        },
    },
}));
  
interface Props {
}

const ToggleThemeButton = () => {
    const { colorScheme, toggleColorScheme } = useMantineColorScheme();
    const dark = colorScheme === 'dark';

    return (
        <ActionIcon
            variant="outline"
            color={dark ? 'yellow' : 'blue'}
            onClick={() => toggleColorScheme()}
            title="Toggle color scheme"
            >
            {dark ? <IconSun size="1.1rem" /> : <IconMoonStars size="1.1rem" />}
        </ActionIcon>
    );
};

const AppHeader = (props: Props) => {
    const [drawerOpened, { toggle: toggleDrawer, close: closeDrawer }] = useDisclosure(false);
    const [linksOpened, { toggle: toggleLinks }] = useDisclosure(false);
    const {classes, theme} = useStyles();

    return (
        <Box>
            <Header height={60} px="md" zIndex={101} className={classes.header}>
                <Group position="apart" sx={{ height: '100%' }}>
                    <Group sx={{ height: '100%' }} spacing={0} className={classes.hiddenMobile}>

                        <a href="#" className={classes.link}>
                            <img src={logo} />
                        </a>

                        <a href="#/nft" className={classes.link}>
                            <IconPhoto size="1rem" />&nbsp;Meme NFT
                        </a>

                        <a href="#/coin" className={classes.link}>
                            <IconCoin size="1rem" />&nbsp;Meme Coin
                        </a>
                    </Group>

                    <Group className={classes.hiddenMobile}>
                        <ToggleThemeButton />
                    </Group>
                    
                    <Burger opened={drawerOpened} onClick={toggleDrawer} className={classes.hiddenDesktop} />
                </Group>
            </Header>

            <Drawer
                opened={drawerOpened}
                onClose={closeDrawer}
                size="100%"
                padding="md"
                title={
                    <a href="#" className={classes.link} onClick={closeDrawer}>
                        <img src={logo} />
                    </a>
                }
                className={classes.hiddenDesktop}
                zIndex={1000000}
            >
                <ScrollArea h={`calc(100vh - ${rem(60)})`} mx="-md">
                    <Divider my="sm" color={theme.colorScheme === 'dark' ? 'dark.5' : 'gray.1'} />
        
                    <Link to="/nft" className={classes.link} onClick={closeDrawer}>
                        <IconPhoto size="1rem" />&nbsp;Meme NFT
                    </Link>

                    <Link to="/coin" className={classes.link} onClick={closeDrawer}>
                        <IconCoin size="1rem" />&nbsp;Meme Coin
                    </Link>

                    <Divider my="sm" color={theme.colorScheme === 'dark' ? 'dark.5' : 'gray.1'} />
        
                    <Group className={classes.hiddenDesktop} position="center" grow pb="xl" px="md">
                        <div>
                            <ToggleThemeButton />
                        </div>
                    </Group>
                </ScrollArea>
            </Drawer>
        </Box>
    );
}

export default AppHeader;