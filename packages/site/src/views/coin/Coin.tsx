import React from "react";
import { Center, Container } from "@mantine/core";
import { PageTitle } from "@components/PageTitle";
import { IconCoin } from "@tabler/icons-react";

const Coin = () => {
    return (
        <Container>
            <PageTitle 
                title="MEME Coin"
                icon={IconCoin}
            />
            <Center>
                <h1>
                    Soon...
                </h1>
            </Center>
        </Container>
    );
}

export default Coin;
