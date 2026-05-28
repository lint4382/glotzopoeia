import { createContext } from "svelte";
import { SvelteSet } from "svelte/reactivity";

export const [getDocument, setDocument] = createContext<Document>();

export class Document {
    root: BlockId;
    blocks: Record<BlockId, Block>;
    selection: SvelteSet<BlockId> = $state(new SvelteSet());
    edit: { id: BlockId; initialPosition: number } | undefined = $state();

    constructor(root: BlockId, blocks: Record<BlockId, Block>) {
        this.root = $state(root);
        this.blocks = $state(blocks);
    }

    handleKeyDown(e: KeyboardEvent) {
        if (e.key === "ArrowUp" && e.ctrlKey) {
        } else if (e.key === "ArrowDown" && e.ctrlKey) {
        }
    }

    private assertBlockType<T extends BlockData["tag"]>(
        block: Block,
        tag: T,
    ): asserts block is Block & { data: Extract<BlockData, { tag: T }> } {
        if (block.data.tag !== tag) {
            throw "Err!";
        }
    }

    getBlock<T extends BlockData["tag"]>(
        id: BlockId,
        tag: T,
    ): Block & { data: Extract<BlockData, { tag: T }> } {
        const block = this.blocks[id];
        this.assertBlockType(block, tag);
        return block;
    }
}

export type BlockId = string;

export type Block = { data: BlockData; children: BlockId[] };

export type BlockData =
    | { tag: "Gloss"; value: GlossData }
    | { tag: "List"; value: ListData }
    | { tag: "Opaque"; value: undefined }
    | { tag: "Text"; value: TextData };

export type GlossData = { rows: Record<string, GlossRow> };

export type GlossRow =
    | { tag: "FreeForm"; value: string }
    | { tag: "Interlinear"; value: string[] };

export type ListData = { style: "Bullet" | "Numbered" };

export type TextData = {
    content: string;
    style: TextStyle;
};

export type TextStyle = "P" | "H1" | "H2" | "H3" | "H4" | "H5" | "H6";
