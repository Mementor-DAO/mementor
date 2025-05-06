import React, { FunctionComponent, ReactElement } from 'react';
import { Card, Center, Group, Stack, Text, ThemeIcon, createStyles, useMantineTheme } from '@mantine/core';
import { IconProps } from '@tabler/icons-react';
import logo from "@assets/logo-mini-bg.svg";

const useStyles = createStyles((theme) => ({
    card: {
        backgroundRepeat: 'repeat-x',
        backgroundPositionX: 'center',
        backgroundPositionY: '-128px',
        backgroundImage: `url(${logo})`
    }
}));

interface Props {
    title: string;
    icon: FunctionComponent<IconProps>;
    children?: ReactElement|ReactElement[];
};

export const PageTitle = (props: Props) => {
    const theme = useMantineTheme();
    const {classes} = useStyles();

    const color = theme.colors.magenta[theme.colorScheme === 'dark'? 4: 9];
    
    return (
        <Card 
            className={classes.card}    
            radius="md" 
            p="md" 
            mb="lg" 
            shadow="sm"
        >
            <Group noWrap spacing="xl" position="apart">
                <Stack>
                    <Center>
                        <ThemeIcon size="3rem" variant='' radius="md">
                            <props.icon size="3rem" color={color} />
                        </ThemeIcon>
                        &nbsp;
                        <Text fz="md" fw={500} color={color}>
                            {props.title}
                        </Text>
                    </Center>
                </Stack>
                <Center>
                    {props.children}
                </Center>
            </Group>
        </Card>
    );
}