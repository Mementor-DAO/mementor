import React from 'react';
import { Anchor, Box, Center, Container, Divider, Grid, Space, Stack, Text } from '@mantine/core';
import { IconBook, IconBrandGithubFilled, IconBrandTwitterFilled, IconCoin, IconHelp, IconLock, IconPhoto } from '@tabler/icons-react';
import poweredBy from "@assets/powered-by.svg";
import openchat from "@assets/openchat.svg";

interface Props {
}

const AppFooter = (props: Props) => {
    
    const year = new Date().getFullYear();
    
    return (
        <Container pt="2rem">
            <Divider pb="xl" />
            <Grid>
                <Grid.Col span={12}>
                    <Center>
                        <a href="https://dfinity.org" target="_blank">
                            <img src={poweredBy} />
                        </a>
                    </Center>
                </Grid.Col>
            </Grid>
            <Grid pt="md">
                <Grid.Col span={4}>
                    <Stack>
                        <Anchor href="#/faq">
                            <IconHelp size="1rem" />&nbsp;FAQ
                        </Anchor>
                        <Anchor href="#/policies">
                            <IconBook size="1rem" />&nbsp;Policies
                        </Anchor>
                    </Stack>
                </Grid.Col>
                <Grid.Col span={4}>
                    <Stack>
                        <Anchor href="#/privacy">
                            <IconLock size="1rem" />&nbsp;Privacy
                        </Anchor>
                    </Stack>
                </Grid.Col>
                <Grid.Col span={4}>
                    <Grid>
                        <Grid.Col span={4}>
                            <Anchor 
                                href="https://twitter.com/mementor_dao"
                                target="_blank"
                                title='twitter'
                            >
                                <IconBrandTwitterFilled size="2rem" />
                            </Anchor>
                        </Grid.Col>
                        <Grid.Col span={4}>
                            <Anchor 
                                href="https://oc.app/community/hgisd-iiaaa-aaaac-aqlrq-cai"
                                target="_blank"
                                title='openchat'
                            >
                                <img 
                                    width="32"
                                    src={openchat} 
                                />
                            </Anchor>
                        </Grid.Col>
                        <Grid.Col span={4}>
                            <Anchor
                                href="https://github.com/Mementor-DAO"
                                target="_blank"
                                title='github'
                            >
                                <IconBrandGithubFilled size="2rem" />
                            </Anchor>
                        </Grid.Col>
                    </Grid>
                </Grid.Col>
            </Grid>
            <Space />
            <Center mt="xl">
                <small>
                    © {year} Mementor DAO
                </small>
            </Center>
            <Box bg="pink" p="xs" mt="xl">
                <Center>
                    <Text color='white' size="xs">Notice: this site is under development and should be used for testing and demonstration only.</Text>
                </Center>
            </Box>
        </Container>
    );
}

export default AppFooter;