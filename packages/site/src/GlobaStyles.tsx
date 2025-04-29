import React from "react";
import { Global } from "@mantine/core";

export const GlobalStyles = () => {
    return (
        <Global
            styles={(theme) => ({
                'a': {
                    textDecoration: 'none',
                }
            })}
        />
    );
  }