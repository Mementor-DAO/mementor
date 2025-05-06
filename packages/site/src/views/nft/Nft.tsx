import React from "react";
import { Center, Container } from "@mantine/core";
import { PageTitle } from "@components/PageTitle";
import { IconPhoto } from "@tabler/icons-react";

const Nft = () => {
    return (
        <Container>
            <PageTitle 
                title="MEME NFT"
                icon={IconPhoto}
            />
            <Center>
                <h1>
                    Soon...
                </h1>
            </Center>
        </Container>
    );
}

export default Nft;
