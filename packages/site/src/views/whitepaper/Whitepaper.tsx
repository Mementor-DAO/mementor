import React from "react";
import { Center, Container } from "@mantine/core";
import { PageTitle } from "@components/PageTitle";
import { IconBook } from "@tabler/icons-react";

const Whitepaper = () => {
    return (
        <Container>
            <PageTitle 
                title="Whitepaper"
                icon={IconBook}
            />
            <Center>
                <h1>
                    Soon...
                </h1>
            </Center>
        </Container>
    );
}

export default Whitepaper;
