import React, { useCallback } from "react";
import {HashRouter as Router} from "react-router-dom";
import {QueryClient, QueryClientProvider} from 'react-query';
import { ColorScheme, ColorSchemeProvider, MantineProvider } from '@mantine/core';
import { Notifications } from '@mantine/notifications';
import { useLocalStorage } from "@mantine/hooks";
import {AuthContextProvider} from "@stores/auth";
import { UIContextProvider } from "@stores/ui";
import { WalletContextProvider } from "@stores/wallet";
import { IcProviderBuider } from "@libs/icproviderbuilder";
import Home from "@views/home/Home";
import { GlobalStyles } from "./GlobaStyles";

const queryClient = new QueryClient({
    defaultOptions: {
        queries: {
            staleTime: Infinity,
        },
    },
});

const authProvider = new IcProviderBuider()
    .withInternetIdentity()
    .build();

const colors = {
    'magenta': [
        "#ffe9ff", "#ffd1ff", "#faa1fa", "#f66ef6", "#f243f2", 
        "#f028f0", "#f018f0", "#d608d6", "#c000c0", "#a900a9"
    ],
};

export const App = () => {
    const [colorScheme, setColorScheme] = useLocalStorage<ColorScheme>({
        key: 'mantine-color-scheme',
        defaultValue: 'dark',
        getInitialValueInEffect: true,
    });
    
    const toggleColorScheme = useCallback((value?: ColorScheme) => {
        setColorScheme(value || (colorScheme === 'dark' ? 'light' : 'dark'));
    }, [colorScheme, setColorScheme]);
        
    return (
        <QueryClientProvider 
            client={queryClient}
        >
            <AuthContextProvider 
                provider={authProvider}
            >
                <WalletContextProvider>
                    <UIContextProvider>
                            <ColorSchemeProvider 
                                colorScheme={colorScheme} 
                                toggleColorScheme={toggleColorScheme}
                            >
                                <MantineProvider 
                                    theme={{ 
                                        colorScheme, 
                                        primaryColor: 'magenta',
                                        colors: colors as any, 
                                        defaultGradient: {from: '#f06000', to: '#f000f0', deg: 135.0}
                                    }} 
                                    withGlobalStyles 
                                    withNormalizeCSS
                                >
                                    <GlobalStyles />
                                    <Notifications 
                                        position="bottom-right" 
                                    />
                                    <Router> 
                                        <Home />
                                    </Router>
                                </MantineProvider>
                            </ColorSchemeProvider>
                    </UIContextProvider>
                </WalletContextProvider>
            </AuthContextProvider>
        </QueryClientProvider>
    );
};


