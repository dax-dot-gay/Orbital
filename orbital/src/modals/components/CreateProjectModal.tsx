import {
    TbAlertTriangleFilled,
    TbMapPlus,
    TbPencil,
    TbVersionsFilled,
    TbX,
} from "react-icons/tb";
import { createModal } from "../util";
import { ContextModalProps } from "@mantine/modals";
import { useLoadingPromise, useLoading } from "../../types/loading";
import { useApi } from "../../utils/api";
import { useForm } from "@mantine/form";
import {
    Badge,
    Button,
    Checkbox,
    Group,
    Loader,
    Select,
    Stack,
    Text,
    TextInput,
} from "@mantine/core";
import { useTranslation } from "react-i18next";

function CreateProjectModalContents({ context, id }: ContextModalProps<{}>) {
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
        validate: {
            name: (value) =>
                value.length > 0 ? null : t("errors.form.required"),
            assetVersion: (value) => (value ? null : t("errors.form.required")),
        },
    });

    return (
        <form onSubmit={form.onSubmit(console.log)}>
            <Stack gap="sm">
                <TextInput
                    label={t("modals.createProject.name.label")}
                    placeholder={t("modals.createProject.name.placeholder")}
                    leftSection={<TbPencil size={20} />}
                    size="md"
                    withAsterisk
                    {...form.getInputProps("name")}
                />
                <Select
                    withAsterisk
                    label={t("modals.createProject.assetVersion.label")}
                    leftSection={
                        versionsLoading ? (
                            <Loader size="xs" />
                        ) : versionsError ? (
                            <TbAlertTriangleFilled color="orange" size={20} />
                        ) : (
                            <TbVersionsFilled size={20} />
                        )
                    }
                    size="md"
                    data={versions ?? []}
                    placeholder={
                        versionsError && !versionsLoading && !versions
                            ? t("modals.createProject.error.noVersions")
                            : undefined
                    }
                    allowDeselect={false}
                    renderOption={({ option: { value }, checked }) => {
                        const versionNumber = value.split("-", 2)[0];
                        const versionTag: "stable" | "legacy" | "experimental" =
                            (value.split("-", 2)[1] as any) ?? "experimental";

                        return (
                            <Group gap="sm">
                                <Checkbox checked={checked} />
                                <Text>{versionNumber}</Text>
                                <Badge
                                    fw="normal"
                                    color={
                                        versionTag == "stable"
                                            ? "green"
                                            : versionTag == "experimental"
                                              ? "orange"
                                              : "gray"
                                    }
                                >
                                    {versionTag.toUpperCase()}
                                </Badge>
                            </Group>
                        );
                    }}
                    {...form.getInputProps("assetVersion")}
                />
                <Group gap="sm" justify="space-between" grow>
                    <Button
                        variant="light"
                        color="red"
                        leftSection={<TbX size={20} />}
                        onClick={() => context.closeContextModal(id, true)}
                        justify="space-between"
                    >
                        {t("actions.cancel")}
                    </Button>
                    <Button
                        leftSection={<TbMapPlus size={20} />}
                        type="submit"
                        justify="space-between"
                    >
                        {t("actions.create")}
                    </Button>
                </Group>
            </Stack>
        </form>
    );
}

export const { modal: CreateProjectModal, factory: CreateProjectFactory } =
    createModal("createProject", CreateProjectModalContents, {
        icon: TbMapPlus,
        size: "lg",
        centered: true,
        overlayProps: {
            style: {
                "-webkit-backdrop-filter": "blur(4px)",
            },
        },
    });
