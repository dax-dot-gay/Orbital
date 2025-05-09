import { createInstance } from "i18next";
import { I18nextProvider } from "react-i18next";
import * as LangEN from "../lang/en.json";
import { ReactNode } from "react";

const instance = createInstance({
    fallbackLng: "en",
    interpolation: {
        escapeValue: false, // not needed for react!!
    },
    resources: {
        en: {
            translation: LangEN,
        },
    },
});

instance.init();

export function LocalizationProvider({
    children,
}: {
    children?: ReactNode | ReactNode[];
}) {
    return (
        <I18nextProvider i18n={instance} defaultNS="translation">
            {children}
        </I18nextProvider>
    );
}
