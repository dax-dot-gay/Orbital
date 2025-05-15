import { TbMapPlus } from "react-icons/tb";
import { createModal } from "../util";

export const { modal: CreateProjectModal, factory: CreateProjectFactory } =
    createModal("createProject", () => <>TEST</>, {
        icon: TbMapPlus,
        size: "xl",
        centered: true,
    });
