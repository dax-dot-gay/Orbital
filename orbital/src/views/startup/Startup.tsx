import {
    Box,
    Button,
    Group,
    Paper,
    ScrollArea,
    ScrollAreaAutosize,
    SimpleGrid,
    Skeleton,
    Stack,
    Text,
    Title,
} from "@mantine/core";
import {
    TbFolderOpen,
    TbMapOff,
    TbMapPlus,
    TbSatellite,
    TbX,
} from "react-icons/tb";
import { useTranslation } from "react-i18next";
import { useModal } from "../../modals";

export function StartupView() {
    const { t } = useTranslation();
    const { open: openCreate, close: closeCreate } = useModal("createProject");
    return (
        <Box className="view startup">
            <Stack gap="sm" className="startup-stack">
                <Stack
                    gap="md"
                    align="center"
                    justify="center"
                    w="100%"
                    h="128px"
                >
                    <Title order={3} fw={400} c="dimmed">
                        - {t("views.start.titleWelcome")} -
                    </Title>
                    <Group gap="md">
                        <TbSatellite size={40} />
                        <Title order={1}>{t("app.name")}</Title>
                    </Group>
                </Stack>
                <Paper className="startup-list" withBorder p="sm" shadow="sm">
                    {false ? (
                        <ScrollArea mah="100%" style={{ overflowY: "scroll" }}>
                            <SimpleGrid
                                cols={{ base: 1, lg: 2 }}
                                spacing="sm"
                                verticalSpacing="sm"
                            ></SimpleGrid>
                        </ScrollArea>
                    ) : (
                        <Stack gap="sm" className="no-projects">
                            <Group
                                gap="sm"
                                style={{
                                    userSelect: "none",
                                    pointerEvents: "none",
                                }}
                            >
                                <TbMapOff size={28} opacity={0.6} />
                                <Text
                                    style={{
                                        userSelect: "none",
                                        pointerEvents: "none",
                                    }}
                                    size="lg"
                                    c="dimmed"
                                >
                                    {t("views.start.noProjects")}
                                </Text>
                            </Group>
                            <Button
                                leftSection={<TbMapPlus size={20} />}
                                size="sm"
                                justify="space-between"
                                onClick={() => openCreate()}
                            >
                                {t("views.start.createProject")}
                            </Button>
                        </Stack>
                    )}
                </Paper>
                <Group wrap="nowrap" grow gap="sm">
                    <Button
                        leftSection={<TbX size={20} />}
                        size="sm"
                        justify="space-between"
                    >
                        {t("actions.remove")}
                    </Button>
                    <Button
                        leftSection={<TbFolderOpen size={20} />}
                        size="sm"
                        justify="space-between"
                    >
                        {t("actions.open")}
                    </Button>
                    <Button
                        leftSection={<TbMapPlus size={20} />}
                        size="sm"
                        justify="space-between"
                        onClick={() => openCreate()}
                    >
                        {t("views.start.createProject")}
                    </Button>
                </Group>
            </Stack>
        </Box>
    );
}
