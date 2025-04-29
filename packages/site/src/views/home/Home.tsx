import React from "react";
import { Route, Routes } from "react-router-dom";
import { AppShell, Container, LoadingOverlay, Space, useMantineTheme } from '@mantine/core';
import { useUI } from "@hooks/ui";
import Header from "./Header";
import Footer from "./Footer";
import {Front} from "./Front";
import loadingImage from "@assets/loading.svg";
import Nft from "@views/nft/Nft";
import Coin from "@views/coin/Coin";
import Whitepaper from "@views/whitepaper/Whitepaper";

interface Props {
}

const Home = (props: Props) => {
    const theme = useMantineTheme();
    const {isLoading} = useUI();

    return (
        <>
            <AppShell
                styles={{
                    root: {
                        background: theme.colorScheme === 'dark' ? 
                            theme.colors.dark[8]
                        : 
                            theme.white,
                        backgroundImage: theme.colorScheme === 'dark'?
                            'url("/images/bg-black.svg")'
                        :
                            'url("/images/bg-white.svg")',
                        minHeight: '10rem',
                    },
                }}
                navbarOffsetBreakpoint="sm"
                asideOffsetBreakpoint="sm"
                header={
                    <Header />
                }
                footer={
                    <Footer />
                }
            >
                <Space h="4rem" />

                <Container size="md">
                    <Routes>
                        <Route path="/" element={<Front />} />
                        <Route path="/nft" element={<Nft />} />
                        <Route path="/coin" element={<Coin />} />
                        <Route path="/whitepaper" element={<Whitepaper />} />
                    </Routes>
                </Container>
            </AppShell>
            
            <LoadingOverlay loader={<img src={loadingImage} />} visible={isLoading}  />
        </>
    );
};

export default Home;