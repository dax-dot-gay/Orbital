import { Group, ModalProps, Stack, Text } from "@mantine/core";
import { ContextModalProps, modals } from "@mantine/modals";
import { TFunction } from "i18next";
import { defaultsDeep, omit } from "lodash";
import { ReactElement } from "react";
import { IconType } from "react-icons/lib";
import { TbInfoCircleFilled } from "react-icons/tb";

export type ModalOptions = Partial<{
    title: string;
    subtitle: string;
    icon: IconType;
}> &
    Partial<Omit<ModalProps, "title">>;

export type ModalExport<T extends object | undefined = undefined> = {
    modal: (
        props: ContextModalProps<T extends object ? T : {}>,
    ) => ReactElement;
    factory: (
        t: TFunction<"translation", undefined>,
    ) => (props: T extends object ? T : {}, overrides?: ModalOptions) => string;
};

export function createModal<T extends object | undefined = undefined>(
    id: string,
    content: (
        props: ContextModalProps<T extends object ? T : {}>,
    ) => ReactElement,
    defaultOptions?: Partial<{
        title: string;
        subtitle: string;
        icon: IconType;
    }> &
        Partial<ModalProps>,
): ModalExport<T> {
    return {
        modal: content,
        factory: (t) => {
            const modalId = id;
            const opts = defaultOptions ?? {};
            return (
                props: T extends object ? T : {},
                overrides?: ModalOptions,
            ) => {
                const IconElement =
                    overrides?.icon ?? opts.icon ?? TbInfoCircleFilled;
                const titleString = t(
                    overrides?.title ?? opts.title ?? `modals.${modalId}.title`,
                );
                const subtitleLocalizationString =
                    overrides?.subtitle ?? opts.subtitle;
                const subtitleString = subtitleLocalizationString
                    ? t(subtitleLocalizationString)
                    : null;
                const remainingProps: Partial<Omit<ModalProps, "title">> = omit(
                    defaultsDeep(overrides ?? {}, opts),
                    "title",
                    "subtitle",
                    "icon",
                );

                return modals.openContextModal({
                    modal: modalId,
                    innerProps: props ?? {},
                    title: (
                        <Group gap="sm">
                            <IconElement size="28" />
                            <Stack gap={0}>
                                <Text size="lg" fw="400">
                                    {titleString}
                                </Text>
                                {subtitleString && (
                                    <Text c="dimmed" size="xs">
                                        {subtitleString}
                                    </Text>
                                )}
                            </Stack>
                        </Group>
                    ),
                    ...remainingProps,
                });
            };
        },
    };
}
