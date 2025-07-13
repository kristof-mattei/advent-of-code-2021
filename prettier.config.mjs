/** @type {import("prettier").Config} */
export default {
    arrowParens: "always",
    bracketSpacing: true,
    printWidth: 120,
    quoteProps: "as-needed",
    semi: true,
    singleQuote: false,
    tabWidth: 4,
    trailingComma: "all",
    useTabs: false,
    overrides: [
        {
            files: ["*.toml"],
            options: {
                printWidth: 80,
            },
        },
        {
            files: ["*.ts", "*.tsx"],
            options: {
                parser: "typescript",
            },
        },
        {
            files: ["*.json"],
            options: {
                trailingComma: "none",
                printWidth: 80,
            },
        },
        {
            files: ["*.yaml", "*.yml"],
            options: {
                tabWidth: 2,
            },
        },
    ],
    plugins: ["prettier-plugin-sh", "prettier-plugin-toml"],
};
