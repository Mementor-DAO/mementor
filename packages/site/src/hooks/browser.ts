import { useCallback } from "react";
import { useMediaQuery } from "@mantine/hooks";
import { useLocation, useNavigate } from "react-router-dom";

interface BrowserProps {
    isMobile: boolean;
    returnToLastPage: () => void;
    navigateTo: (page: string) => void;
    redirectToLogin: () => void;
    redirectToSignup: () => void;
    navigateToHome: () => void;
    navigateToUserPage: () => void;
};

export const useBrowser = (): BrowserProps => {
    const navigate = useNavigate();
    const location = useLocation();
    
    const getReturnUrl = (): string => {
        const returnTo = location.search.match(/return=([^&]+)/);
        return (returnTo && returnTo[1]) || '/';
    }

    const returnToLastPage = useCallback(() => {
        navigate(getReturnUrl());
    }, [getReturnUrl]);
    
    const navigateTo = useCallback((page: string) => {
        navigate(page);
    }, []);

    const redirectToLogin = useCallback(() => {
        navigate(`/users/login?return=${window.location.hash.replace('#', '')}`);
    }, []);

    const redirectToSignup = useCallback(() => {
        navigate(`/users/signup?return=${window.location.hash.replace('#', '')}`);
    }, []);

    const navigateToHome = useCallback(() => {
        navigate("/");
    }, []);

    const navigateToUserPage = useCallback(() => {
        navigate("/users");
    }, []);
    
    return {
        isMobile: !useMediaQuery('(min-width: 62em)'),
        returnToLastPage,
        navigateTo,
        redirectToLogin,
        redirectToSignup,
        navigateToHome,
        navigateToUserPage,
    };
};
