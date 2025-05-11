import {
    Box,
    BoxProps,
    Center,
    Image,
    ImageProps,
    Loader,
    PolymorphicComponentProps,
} from "@mantine/core";
import { useAssetPath } from ".";
import { useLoading } from "../../types/loading";
import { TbError404 } from "react-icons/tb";

export function AssetImage({
    src,
    fallback,
    imgProps,
    ...props
}: {
    src: string;
    fallback?: string;
    imgProps?: Partial<
        Omit<PolymorphicComponentProps<"img", ImageProps>, "src">
    >;
} & Partial<PolymorphicComponentProps<"div", BoxProps>>) {
    const { value: imgpath, error } = useLoading(useAssetPath(src));
    return (
        <Box {...props}>
            {imgpath || fallback ? (
                <Image src={imgpath || fallback} {...imgProps} />
            ) : (
                <Center h="100%" w="100%">
                    {error ? <TbError404 size={24} /> : <Loader />}
                </Center>
            )}
        </Box>
    );
}
