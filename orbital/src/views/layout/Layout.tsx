import {
    ActionIcon,
    AppShell,
    Avatar,
    Burger,
    Center,
    Divider,
    Group,
    ScrollArea,
    Stack,
    Text,
    ThemeIcon,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { Outlet } from "react-router";
import {
    TbBuildingFactory,
    TbMoonFilled,
    TbSatellite,
    TbSun,
} from "react-icons/tb";
import { useTranslation } from "react-i18next";
import { useAppTheme } from "../../utils/theme/themeContext";
import { invoke } from "@tauri-apps/api/core";

export function AppLayout() {
    const [collapsed, { toggle: toggleCollapsed }] = useDisclosure(false);
    const { t } = useTranslation();
    const [theme, setTheme] = useAppTheme();

    invoke("test_command").then(console.log);

    return (
        <AppShell
            id="app-root"
            layout="alt"
            header={{ height: "64px" }}
            navbar={{
                width: "256px",
                breakpoint: "sm",
                collapsed: { desktop: collapsed, mobile: collapsed },
            }}
        >
            <AppShell.Header className="app-header">
                <Group p={0} align="center" h="100%" gap={0} wrap="nowrap">
                    <Center h="64px" w="64px">
                        <Burger
                            size="sm"
                            lineSize="2"
                            onClick={() => toggleCollapsed()}
                            opened={!collapsed}
                            style={{
                                outline: "calc(0.0625rem * 2)",
                            }}
                        />
                    </Center>
                    <Group gap="sm" p={0} h="100%" style={{ flexGrow: 2 }}>
                        <Divider orientation="vertical" />
                        <Avatar size="md" radius="sm" variant="filled">
                            <TbSatellite size={24} />
                        </Avatar>
                        <Stack gap={0}>
                            <Text ff="monospace" size="lg" lh="md">
                                {t("app.name")}
                            </Text>
                            <Text c="dimmed" size="xs">
                                {t("app.subtitle")}
                            </Text>
                        </Stack>
                    </Group>
                    <Center h="64px" w="64px">
                        <ActionIcon
                            size="xl"
                            radius="sm"
                            variant="subtle"
                            onClick={() =>
                                setTheme(theme === "dark" ? "light" : "dark")
                            }
                        >
                            {theme === "dark" ? (
                                <TbSun size={24} />
                            ) : (
                                <TbMoonFilled size={24} />
                            )}
                        </ActionIcon>
                    </Center>
                </Group>
            </AppShell.Header>
            <AppShell.Navbar className="app-nav">
                <Stack className="nav-stack" gap={0} p={0}>
                    <Group
                        gap="sm"
                        wrap="nowrap"
                        h="63px"
                        px="md"
                        align="center"
                    >
                        <ThemeIcon size="lg" radius="sm">
                            <TbBuildingFactory size={24} />
                        </ThemeIcon>
                        <Stack gap={0} style={{ flexGrow: 1 }}>
                            <Text lineClamp={1}>
                                Test Factory With A Very Long Name
                            </Text>
                            <Text c="dimmed" size="xs">
                                1.0 - Stable
                            </Text>
                        </Stack>
                    </Group>
                    <Divider />
                </Stack>
            </AppShell.Navbar>
            <AppShell.Main
                className={collapsed ? "app-body collapsed" : "app-body"}
            >
                <Outlet />
            </AppShell.Main>
        </AppShell>
    );
}
