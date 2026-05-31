<script lang="ts">
    const { source }: { source: string } = $props();
    import { invoke } from "@tauri-apps/api/core";

    let root: HTMLDivElement | undefined;

    type XmlNode =
        | { type: "Text"; from: number; to: number; content: string }
        | {
              type: "Element";
              open: XmlOpenTag;
              content: XmlNode[];
              close: XmlCloseTag;
          }
        | { type: "SelfClosingElement"; tag: XmlOpenTag };
    type XmlOpenTag = {
        from: number;
        to: number;
        name: string;
        attributes: { key: string; value: string }[];
    };
    type XmlCloseTag = { from: number; to: number };

    let nodeToDocumentOffsets = $state<Map<Node, number>>(new Map());

    $effect(() => {
        if (root!.firstChild) {
            root!.firstChild.remove();
        }
        invoke("parse", { xml: source })
            .then((x) => {
                nodeToDocumentOffsets.clear();
                let nodes = x as XmlNode[];
                for (const node of nodes) {
                    root!.append(toDom(node));
                }
            })
            .catch((err) => {
                console.error(err);
            });
    });

    function getStartPos(xml: XmlNode): number {
        if (xml.type === "Text") {
            return xml.from;
        } else if (xml.type === "Element") {
            return xml.content.length === 0
                ? xml.open.from
                : getStartPos(xml.content[0]);
        } else if (xml.type === "SelfClosingElement") {
            return xml.tag.from;
        } else {
            throw new Error("Unknown XML node type");
        }
    }

    function toDom(xml: XmlNode): Node {
        const node = toDomInner(xml);
        nodeToDocumentOffsets.set(node, getStartPos(xml));
        return node;
    }

    function toDomInner(xml: XmlNode): Node {
        if (xml.type === "Text") {
            return document.createTextNode(xml.content);
        } else if (xml.type === "Element") {
            const el = document.createElement(xml.open.name);
            for (const attr of xml.open.attributes) {
                el.setAttribute(attr.key, attr.value);
            }
            for (const child of xml.content) {
                el.append(toDom(child));
            }
            return el;
        } else if (xml.type === "SelfClosingElement") {
            const el = document.createElement(xml.tag.name);
            for (const attr of xml.tag.attributes) {
                el.setAttribute(attr.key, attr.value);
            }
            return el;
        } else {
            throw new Error("Unknown XML node type " + xml);
        }
    }

    export function getPosition(node: Node, offset: number): number {
        return nodeToDocumentOffsets.has(node)
            ? nodeToDocumentOffsets.get(node)! + offset
            : 0;
    }
</script>

<div bind:this={root}></div>
