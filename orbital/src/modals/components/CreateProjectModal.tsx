import { TbMapPlus, TbPencil } from "react-icons/tb";
import { createModal } from "../util";
import { ContextModalProps } from "@mantine/modals";
import { useLoadingPromise, useLoading } from "../../types/loading";
import { useApi } from "../../utils/api";
import { useForm } from "@mantine/form";
import { Stack, TextInput } from "@mantine/core";
import { useTranslation } from "react-i18next";

function CreateProjectModalContents(_props: ContextModalProps<{}>) {
    const api = useApi();
    const { t } = useTranslation();
    const {
        value: versions,
        error: versionsError,
        loading: versionsLoading,
    } = useLoading(useLoadingPromise(api.asset_versions.list_available, []));
    const form = useForm<{
        name: string;
        assetVersion: string | null;
    }>({
        initialValues: {
            name: "",
            assetVersion: null,
        },
    });

    return (
        <form onSubmit={form.onSubmit(console.log)}>
            <Stack gap="sm">
                <TextInput
                    label={t("modals.createProject.name.label")}
                    placeholder={t("modals.createProject.name.placeholder")}
                    leftSection={<TbPencil size={20} />}
                    {...form.getInputProps("name")}
                />
            </Stack>
        </form>
    );
}

export const { modal: CreateProjectModal, factory: CreateProjectFactory } =
    createModal("createProject", CreateProjectModalContents, {
        icon: TbMapPlus,
        size: "xl",
        centered: true,
        overlayProps: {
            style: {
                "-webkit-backdrop-filter": "blur(4px)",
            },
        },
    });
