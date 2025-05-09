import {
    ActionIcon,
    AppShell,
    Avatar,
    Burger,
    Center,
    Divider,
    Group,
    Stack,
    Text,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { Outlet } from "react-router";
import { TbMoonFilled, TbSatellite, TbSun } from "react-icons/tb";
import { useTranslation } from "react-i18next";
import { useAppTheme } from "../../utils/theme/themeContext";

export function AppLayout() {
    const [collapsed, { toggle: toggleCollapsed }] = useDisclosure(false);
    const { t } = useTranslation();
    const [theme, setTheme] = useAppTheme();

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
            <AppShell.Navbar className="app-nav"></AppShell.Navbar>
            <AppShell.Main
                className={collapsed ? "app-body collapsed" : "app-body"}
            >
                <Outlet />
            </AppShell.Main>
        </AppShell>
    );
}
